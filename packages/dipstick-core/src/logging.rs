use std::collections::{BTreeMap, HashMap};
use std::time::SystemTime;

use dipstick_proto::core::v1::{LogConfig, LogEvent, LogLevel, LogSpan};
use tokio::sync::broadcast;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, fmt, reload, Layer as _};

const CHANNEL_CAPACITY: usize = 128;

pub fn init() -> LoggingHandle {
    let global_filter = filter::Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("tokio", tracing::Level::TRACE)
        .with_target("runtime", tracing::Level::TRACE)
        .with_target("dipstick", tracing::Level::TRACE);
    let terminal_filter = filter::Targets::new()
        .with_default(tracing::Level::INFO)
        .with_target("dipstick", tracing::Level::DEBUG);
    let (dipstick_filter, dipstick_filter_get, dipstick_filter_modify) = {
        let (filter, handle) = reload::Layer::new(terminal_filter.clone());
        let get = {
            let handle = handle.clone();
            Box::new(move || handle.clone_current())
        };
        let modify = Box::new(move |modifier| handle.modify(modifier));
        (filter, get, modify)
    };

    let terminal_layer = fmt::layer().with_filter(terminal_filter);
    let (dipstick_layer, logs_tx) = Layer::new();
    let dipstick_layer = dipstick_layer.with_filter(dipstick_filter);
    let console_layer = console_subscriber::spawn();

    tracing_subscriber::registry()
        .with(global_filter)
        .with(terminal_layer)
        .with(dipstick_layer)
        .with(console_layer)
        .init();

    LoggingHandle {
        logs_tx,
        dipstick_filter_get,
        dipstick_filter_modify,
    }
}

type BoxedModifierFn = Box<dyn FnOnce(&mut filter::Targets)>;

pub struct LoggingHandle {
    pub logs_tx: broadcast::Sender<LogEvent>,
    dipstick_filter_get: Box<dyn Fn() -> Option<filter::Targets> + Send + Sync>,
    dipstick_filter_modify: Box<dyn Fn(BoxedModifierFn) -> Result<(), reload::Error> + Send + Sync>,
}

impl LoggingHandle {
    pub fn log_config(&self) -> Option<LogConfig> {
        let layer = (*self.dipstick_filter_get)()?;
        Some(LogConfig {
            default_level: layer
                .default_level()
                .map_or(LogLevel::Unspecified, tracing_level_filter_to_pb)
                as _,
            target_filters: layer
                .into_iter()
                .map(|(target, level)| (target, tracing_level_filter_to_pb(level) as _))
                .collect(),
        })
    }

    pub fn set_log_config(&self, config: LogConfig) -> anyhow::Result<()> {
        let modifier = Box::new(|old_layer: &mut filter::Targets| {
            let mut layer = filter::Targets::new();
            if let Some(level) = pb_to_tracing_filter(config.default_level()) {
                layer = layer.with_default(level);
            } else {
                // keep the old default level
                layer = layer.with_default(old_layer.default_level().unwrap_or(LevelFilter::OFF));
            }

            for (target, level) in config.target_filters {
                let level = LogLevel::try_from(level).unwrap_or(LogLevel::Unspecified);
                if let Some(level) = pb_to_tracing_filter(level) {
                    layer = layer.with_target(target, level);
                }
            }

            *old_layer = layer;
        });
        (*self.dipstick_filter_modify)(modifier).map_err(anyhow::Error::from)
    }
}

struct Layer {
    tx: broadcast::Sender<LogEvent>,
}

impl Layer {
    fn new() -> (Self, broadcast::Sender<LogEvent>) {
        let (logs_tx, _) = broadcast::channel(CHANNEL_CAPACITY);
        let layer = Layer {
            tx: logs_tx.clone(),
        };
        (layer, logs_tx)
    }
}

impl<S> tracing_subscriber::Layer<S> for Layer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_new_span(
        &self,
        attrs: &tracing::span::Attributes<'_>,
        id: &tracing::span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        if let Some(span) = ctx.span(id) {
            let mut storage = FieldStorage::new();
            attrs.record(&mut Visitor(&mut storage.0));
            let mut extensions = span.extensions_mut();
            extensions.insert::<FieldStorage>(storage);
        }
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        if self.tx.receiver_count() == 0 {
            return;
        }

        let now = SystemTime::now();
        let metadata = event.metadata();

        if metadata.level() >= &tracing::Level::DEBUG
            && ({
                let t = metadata.target();
                t.starts_with("h2::") || t.starts_with("hyper::") || t.starts_with("tokio")
            })
        {
            // ignore certain logs because they are caused by us sending these logs
            return;
        }

        let mut spans = Vec::new();
        if let Some(scope) = ctx.event_scope(event) {
            spans = scope
                .from_root()
                .map(|span| {
                    let metadata = span.metadata();
                    let fields = span
                        .extensions()
                        .get::<FieldStorage>()
                        .map_or_else(HashMap::new, |s| s.0.clone());
                    LogSpan {
                        id: span.id().into_u64(),
                        level: tracing_level_to_pb(*metadata.level()) as _,
                        name: metadata.name().to_owned(),
                        target: metadata.target().to_owned(),
                        fields,
                    }
                })
                .collect();
        }

        let mut pb_evt = LogEvent {
            timestamp: Some(now.into()),
            level: tracing_level_to_pb(*metadata.level()) as _,
            target: metadata.target().to_owned(),
            message: String::new(),
            fields: HashMap::with_capacity(metadata.fields().len()),
            spans,
        };

        event.record(&mut Visitor(&mut pb_evt.fields));
        // move message to the message field
        if let Some(dipstick_proto::wkt::Value {
            kind: Some(dipstick_proto::wkt::value::Kind::StringValue(msg)),
        }) = pb_evt.fields.remove("message")
        {
            pb_evt.message = msg;
        }

        let _ = self.tx.send(pb_evt);
    }
}

fn tracing_level_to_pb(level: tracing::Level) -> LogLevel {
    tracing_level_filter_to_pb(level.into())
}

fn tracing_level_filter_to_pb(level_filter: LevelFilter) -> LogLevel {
    match level_filter {
        LevelFilter::TRACE => LogLevel::Trace,
        LevelFilter::DEBUG => LogLevel::Debug,
        LevelFilter::INFO => LogLevel::Info,
        LevelFilter::WARN => LogLevel::Warn,
        LevelFilter::ERROR => LogLevel::Error,
        LevelFilter::OFF => LogLevel::Off,
    }
}

fn pb_to_tracing_filter(level: LogLevel) -> Option<LevelFilter> {
    match level {
        LogLevel::Trace => Some(LevelFilter::TRACE),
        LogLevel::Debug => Some(LevelFilter::DEBUG),
        LogLevel::Info => Some(LevelFilter::INFO),
        LogLevel::Warn => Some(LevelFilter::WARN),
        LogLevel::Error => Some(LevelFilter::ERROR),
        LogLevel::Off => Some(LevelFilter::OFF),
        LogLevel::Unspecified => None,
    }
}

struct FieldStorage(HashMap<String, dipstick_proto::wkt::Value>);

impl FieldStorage {
    fn new() -> Self {
        Self(HashMap::new())
    }
}

struct Visitor<'a>(&'a mut HashMap<String, dipstick_proto::wkt::Value>);

impl<'a> tracing::field::Visit for Visitor<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let kind = dipstick_proto::wkt::value::Kind::StringValue(format!("{value:?}"));
        self.0.insert(field.name().to_owned(), pb_value(kind));
    }

    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        let kind = dipstick_proto::wkt::value::Kind::NumberValue(value);
        self.0.insert(field.name().to_owned(), pb_value(kind));
    }

    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        let value_f64 = value as f64;
        if value_f64 as i64 == value {
            let kind = dipstick_proto::wkt::value::Kind::NumberValue(value_f64);
            self.0.insert(field.name().to_owned(), pb_value(kind));
        } else {
            // value is too large to fit in a f64
            self.record_debug(field, &value);
        }
    }

    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        let value_f64 = value as f64;
        if value_f64 as u64 == value {
            let kind = dipstick_proto::wkt::value::Kind::NumberValue(value_f64);
            self.0.insert(field.name().to_owned(), pb_value(kind));
        } else {
            // value is too large to fit in a f64
            self.record_debug(field, &value);
        }
    }

    fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
        self.record_debug(field, &value)
    }

    fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
        self.record_debug(field, &value)
    }

    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        let kind = dipstick_proto::wkt::value::Kind::BoolValue(value);
        self.0.insert(field.name().to_owned(), pb_value(kind));
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        fn err_to_struct(err: &(dyn std::error::Error + 'static)) -> dipstick_proto::wkt::Struct {
            let mut fields = BTreeMap::new();
            fields.insert(
                "type".to_owned(),
                pb_value(dipstick_proto::wkt::value::Kind::StringValue(
                    std::any::type_name_of_val(err).to_owned(),
                )),
            );
            fields.insert(
                "message".to_owned(),
                pb_value(dipstick_proto::wkt::value::Kind::StringValue(
                    err.to_string(),
                )),
            );
            if let Some(source) = err.source() {
                fields.insert(
                    "source".to_owned(),
                    pb_value(dipstick_proto::wkt::value::Kind::StructValue(
                        err_to_struct(source),
                    )),
                );
            }
            dipstick_proto::wkt::Struct { fields }
        }

        let kind = dipstick_proto::wkt::value::Kind::StructValue(err_to_struct(value));
        self.0.insert(field.name().to_owned(), pb_value(kind));
    }
}

fn pb_value(kind: dipstick_proto::wkt::value::Kind) -> dipstick_proto::wkt::Value {
    dipstick_proto::wkt::Value { kind: Some(kind) }
}
