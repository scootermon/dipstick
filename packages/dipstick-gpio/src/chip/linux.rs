use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use anyhow::Context;
use dipstick_proto::core::v1::IoDir;
use dipstick_proto::gpio::v1::{Level, LinuxChipSpec, PinSpec};
use futures::StreamExt;
use gpiocdev::line::{EventClock, OffsetMap, Values};
use gpiocdev::tokio::AsyncRequest;
use gpiocdev::Request;
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use super::PinMap;

const CONSUMER: &str = "dipstick-gpio";

pub async fn spawn(
    tracker: &TaskTracker,
    cancel_token: CancellationToken,
    pins: Arc<PinMap>,
    spec: &LinuxChipSpec,
    pin_specs: &HashMap<String, PinSpec>,
) -> anyhow::Result<()> {
    tokio::task::block_in_place(|| {
        spawn_blocking(tracker, cancel_token, pins, spec, pin_specs)
    })
}

fn spawn_blocking(
    tracker: &TaskTracker,
    cancel_token: CancellationToken,
    pins: Arc<PinMap>,
    spec: &LinuxChipSpec,
    pin_specs: &HashMap<String, PinSpec>,
) -> anyhow::Result<()> {
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
        match pin.direction() {
            IoDir::Unspecified => builder.as_is(),
            IoDir::In => builder.as_input(),
            IoDir::Out => builder.as_output(gpiocdev::line::Value::Active), /* TODO default value
                                                                             * from spec */
        };
    }
    let req = builder.request()?;
    tracker.spawn(async move {
        worker(cancel_token, &AsyncRequest::new(req), &pins, &offset_to_id).await
    });
    Ok(())
}

async fn worker(
    cancel_token: CancellationToken,
    req: &AsyncRequest,
    pins: &PinMap,
    offset_to_id: &OffsetMap<String>,
) {
    if let Err(err) =
        tokio::task::block_in_place(|| full_read_blocking(req.as_ref(), pins, offset_to_id))
    {
        tracing::error!(err = &*err, "failed to read initial values");
    }

    let mut stream = req.edge_events();
    loop {
        let item = tokio::select! {
            _ = cancel_token.cancelled() => {
                break;
            }
            item = stream.next() => item,
        };
        match item {
            Some(Ok(event)) => {
                let Some(id) = offset_to_id.get(&event.offset) else {
                    continue;
                };
                let logical = match event.kind {
                    gpiocdev::line::EdgeKind::Rising => Level::High,
                    gpiocdev::line::EdgeKind::Falling => Level::Low,
                };
                let timestamp = SystemTime::UNIX_EPOCH + Duration::from_nanos(event.timestamp_ns);
                pins.set_pin_level(id, timestamp, logical)
            }
            Some(Err(err)) => {
                tracing::error!(err = &err as &dyn std::error::Error, "event error??");
            }
            None => {
                unreachable!()
            }
        }
    }
}

fn full_read_blocking(
    req: &Request,
    pins: &PinMap,
    offset_to_id: &OffsetMap<String>,
) -> anyhow::Result<()> {
    let mut values = Values::default();
    let timestamp = SystemTime::now();
    req.values(&mut values)?;
    for line_value in values.iter() {
        let Some(id) = offset_to_id.get(&line_value.offset) else {
            continue;
        };
        let logical = match line_value.value {
            gpiocdev::line::Value::Inactive => Level::Low,
            gpiocdev::line::Value::Active => Level::High,
        };
        pins.set_pin_level(id, timestamp, logical);
    }
    Ok(())
}
