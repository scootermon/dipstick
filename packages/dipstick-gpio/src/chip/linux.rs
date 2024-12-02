use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::Context;
use dipstick_proto::core::v1::IoDir;
use dipstick_proto::gpio::v1::{Bias, Level, LinuxChipSpec, PinSpec};
use gpiocdev::line::{EdgeEvent, EdgeKind, EventClock, OffsetMap, Values};
use gpiocdev::tokio::AsyncRequest;
use gpiocdev::Request;
use tokio::sync::{mpsc, oneshot};
use tokio::task;
use tonic::{Result, Status};

use super::PinMap;

const CONSUMER: &str = "dp-gpio";

pub enum Message {
    Shutdown,
    SetPinLevel {
        id: String,
        logical: Level,
        result: oneshot::Sender<Result<()>>,
    },
}

pub async fn spawn(
    receiver: mpsc::Receiver<Message>,
    pins: Arc<PinMap>,
    spec: &mut LinuxChipSpec,
    pin_specs: &mut HashMap<String, PinSpec>,
) -> Result<()> {
    let actor = task::block_in_place(|| {
        Actor::new_blocking(receiver, pins, spec, pin_specs)
            .map_err(|err| Status::internal(format!("failed to create linux gpio chip: {err}")))
    })?;
    task::Builder::new()
        .name(&format!("{CONSUMER}-{}", spec.name))
        .spawn(run_actor(actor))?;
    Ok(())
}

struct Actor {
    receiver: mpsc::Receiver<Message>,
    req: AsyncRequest,
    pins: Arc<PinMap>,
    offset_to_id: OffsetMap<String>,
}

impl Actor {
    fn new_blocking(
        receiver: mpsc::Receiver<Message>,
        pins: Arc<PinMap>,
        spec: &mut LinuxChipSpec,
        pin_specs: &mut HashMap<String, PinSpec>,
    ) -> anyhow::Result<Self> {
        let chip = gpiocdev::Chip::from_name(&spec.name)
            .with_context(|| format!("chip {:?} not found", spec.name))?;
        let mut builder = Request::builder();
        builder
            .on_chip(chip.path())
            .with_consumer(CONSUMER)
            .with_event_clock(EventClock::Realtime);

        let mut offset_to_id = OffsetMap::<String>::default();
        // TODO: if pin_specs is empty we can completely skip spawning the worker
        for (id, pin) in pin_specs {
            let line_info = chip
                .find_line_info(&pin.line_name)
                .with_context(|| format!("line {:?} not found", pin.line_name))?;

            pins.add_pin(id);
            offset_to_id.insert(line_info.offset, id.clone());

            builder.with_line(line_info.offset);
            if pin.direction() == IoDir::Unspecified {
                // fill in the direction based on the current setting of the pin
                pin.set_direction(match line_info.direction {
                    gpiocdev::line::Direction::Input => IoDir::In,
                    gpiocdev::line::Direction::Output => IoDir::Out,
                });
            }

            match pin.direction() {
                IoDir::In => apply_input_spec(pin, &line_info, &mut builder),
                IoDir::Out => apply_output_spec(pin, &mut builder),
                // unreachable because we handled unspecified above
                IoDir::Unspecified => unreachable!(),
            }
        }
        Ok(Self {
            receiver,
            req: AsyncRequest::new(builder.request()?),
            pins,
            offset_to_id,
        })
    }

    fn full_read_blocking(&self) -> anyhow::Result<()> {
        let mut values = Values::default();
        let timestamp = SystemTime::now();
        self.req.as_ref().values(&mut values)?;
        for line_value in values.iter() {
            let Some(id) = self.offset_to_id.get(&line_value.offset) else {
                continue;
            };
            let logical = match line_value.value {
                gpiocdev::line::Value::Inactive => Level::Low,
                gpiocdev::line::Value::Active => Level::High,
            };
            self.pins.set_pin_level(id, timestamp, logical);
        }
        Ok(())
    }

    fn id_to_offset(&self, id: &str) -> Option<gpiocdev::line::Offset> {
        self.offset_to_id
            .iter()
            .find_map(|(offset, other_id)| if other_id == id { Some(*offset) } else { None })
    }

    fn set_pin_level_blocking(&self, id: &str, logical: Level) -> Result<()> {
        let value = match logical {
            Level::Low => gpiocdev::line::Value::Inactive,
            Level::High => gpiocdev::line::Value::Active,
            Level::Unspecified => return Err(Status::invalid_argument("unspecified level")),
        };
        let offset = self
            .id_to_offset(id)
            .ok_or_else(|| Status::not_found(format!("unknown pin id {id:?}")))?;
        self.req
            .as_ref()
            .set_value(offset, value)
            .map_err(|err| Status::internal(format!("failed to set pin level: {err}")))?;
        let timestamp = SystemTime::now();
        self.pins.set_pin_level(id, timestamp, logical);
        Ok(())
    }

    async fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::Shutdown => self.receiver.close(),
            Message::SetPinLevel {
                id,
                logical: level,
                result,
            } => {
                let res = task::block_in_place(|| self.set_pin_level_blocking(&id, level));
                let _ = result.send(res);
            }
        }
    }

    async fn handle_edge_event(&self, event: gpiocdev::Result<EdgeEvent>) {
        match event {
            Ok(event) => {
                let Some(id) = self.offset_to_id.get(&event.offset) else {
                    // unknown pin, ignore
                    return;
                };
                let logical = match event.kind {
                    EdgeKind::Rising => Level::High,
                    EdgeKind::Falling => Level::Low,
                };
                let timestamp = SystemTime::UNIX_EPOCH + Duration::from_nanos(event.timestamp_ns);
                self.pins.set_pin_level(id, timestamp, logical)
            }
            Err(err) => {
                tracing::error!(err = &err as &dyn std::error::Error, "event error??");
            }
        }
    }
}

async fn run_actor(mut actor: Actor) {
    if let Err(err) = task::block_in_place(|| actor.full_read_blocking()) {
        tracing::error!(err = &*err, "failed to read initial values");
    }

    enum Event {
        Shutdown,
        Message(Message),
        EdgeEvent(gpiocdev::Result<EdgeEvent>),
    }
    loop {
        let event = tokio::select! {
            msg = actor.receiver.recv() => msg.map_or(Event::Shutdown, Event::Message),
            event = actor.req.read_edge_event() => Event::EdgeEvent(event),
        };
        match event {
            Event::Shutdown => break,
            Event::Message(msg) => actor.handle_message(msg).await,
            Event::EdgeEvent(event) => actor.handle_edge_event(event).await,
        }
    }
    tracing::info!("linux gpio actor shutting down");
}

fn apply_input_spec(
    pin: &mut PinSpec,
    line_info: &gpiocdev::line::Info,
    builder: &mut gpiocdev::request::Builder,
) {
    builder
        .as_input()
        .with_edge_detection(gpiocdev::line::EdgeDetection::BothEdges);
    // we ignore the default level, so set it to unspecified
    pin.set_default_level(Level::Unspecified);
    if pin.bias() == Bias::Unspecified {
        // if bias isn't set, use the current bias setting
        pin.set_bias(match line_info.bias {
            Some(gpiocdev::line::Bias::Disabled) | None => Bias::Disable,
            Some(gpiocdev::line::Bias::PullUp) => Bias::PullUp,
            Some(gpiocdev::line::Bias::PullDown) => Bias::PullDown,
        });
    }
    builder.with_bias(match pin.bias() {
        Bias::Disable => gpiocdev::line::Bias::Disabled,
        Bias::PullUp => gpiocdev::line::Bias::PullUp,
        Bias::PullDown => gpiocdev::line::Bias::PullDown,
        // unreachable because we handled unspecified above
        Bias::Unspecified => unreachable!(),
    });
    match pin
        .debounce_period
        .and_then(|period| Duration::try_from(period).ok())
    {
        Some(period) => {
            builder.with_debounce_period(period);
        }
        None => {
            // if debounce period isn't set, use the current setting
            pin.debounce_period = line_info
                .debounce_period
                .and_then(|period| period.try_into().ok());
        }
    }
}

fn apply_output_spec(pin: &mut PinSpec, builder: &mut gpiocdev::request::Builder) {
    builder.as_output(match pin.default_level() {
        Level::Unspecified | Level::Low => gpiocdev::line::Value::Inactive,
        Level::High => gpiocdev::line::Value::Active,
    });
    // remove ignored fields
    pin.set_bias(Bias::Unspecified);
    pin.debounce_period = None;
}
