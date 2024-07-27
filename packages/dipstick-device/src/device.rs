use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
use dipstick_proto::device::v1::{
    DeviceEntity, DeviceEvent, DeviceSpec, DeviceStatus, SensorEvent, SensorStatus,
};
use dipstick_proto::wkt::IntoValue;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Result, Status};

use crate::devices::SharedDeviceVariant;

pub struct Device {
    meta: EntityMeta,
    inner: RwLock<Option<Inner>>,
    tx: broadcast::Sender<DeviceEvent>,
    attrs: RwLock<HashMap<String, dipstick_proto::wkt::Value>>,
    sensors: RwLock<HashMap<String, SensorStatus>>,
}

struct Inner {
    variant: SharedDeviceVariant,
    poll_interval: std::time::Duration,
    // TODO: shutdown
    _handle: JoinHandle<()>,
}

impl Device {
    pub async fn new(core: &Core, meta: EntityMeta, spec: DeviceSpec) -> Result<Arc<Self>> {
        let (tx, _) = broadcast::channel(16); // TODO
        let this = Arc::new(Self {
            meta,
            tx,
            inner: RwLock::new(None),
            attrs: RwLock::new(HashMap::new()),
            sensors: RwLock::new(HashMap::new()),
        });
        this.apply_spec(core, spec).await?;
        Ok(this)
    }

    async fn apply_spec(self: &Arc<Self>, core: &Core, mut spec: DeviceSpec) -> Result<()> {
        let Some(device_spec_variant) = spec.device_spec_variant.take() else {
            return Err(Status::invalid_argument("missing device spec variant"));
        };
        let variant = crate::devices::create_variant(core, self, device_spec_variant)?;
        let poll_interval = spec
            .poll_interval
            .ok_or_else(|| Status::invalid_argument("missing poll interval"))?
            .try_into()
            .map_err(|err| Status::invalid_argument(format!("invalid poll interval: {err}")))?;
        let handle = tokio::spawn(update_loop(
            Arc::clone(self),
            Arc::clone(&variant),
            poll_interval,
        ));

        *self.inner.write().unwrap() = Some(Inner {
            variant,
            poll_interval,
            _handle: handle,
        });
        Ok(())
    }

    fn visit_inner<T>(&self, f: impl FnOnce(&Inner) -> T) -> T {
        let inner = self.inner.read().unwrap();
        // LOGIC: we never expect inner to be `None`
        let inner = inner.as_ref().unwrap();
        f(inner)
    }

    pub fn spec(&self) -> DeviceSpec {
        self.visit_inner(|inner| {
            // LOGIC: conversion can't fail because it originates from the destination
            let poll_interval = inner.poll_interval.try_into().unwrap();
            let device_spec_variant = inner.variant.spec();
            DeviceSpec {
                poll_interval: Some(poll_interval),
                device_spec_variant: Some(device_spec_variant),
            }
        })
    }

    pub fn status(&self) -> DeviceStatus {
        DeviceStatus {
            attrs: self.attrs.read().unwrap().clone(),
            actions: HashMap::new(),
            sensors: self.sensors.read().unwrap().clone(),
        }
    }

    pub fn to_proto(&self) -> DeviceEntity {
        DeviceEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }

    pub fn subscribe(&self) -> BroadcastStream<DeviceEvent> {
        BroadcastStream::new(self.tx.subscribe())
    }

    pub async fn call_action(&self, action: &str) -> Result<()> {
        let variant = self.visit_inner(|inner| Arc::clone(&inner.variant));
        variant.call_action(self, action).await
    }

    pub(crate) fn set_attr(&self, attr: &str, value: impl IntoValue) {
        let mut attrs = self.attrs.write().unwrap();
        if let Some(slot) = attrs.get_mut(attr) {
            *slot = value.into_value();
        } else {
            attrs.insert(attr.to_owned(), value.into_value());
        }
    }

    pub(crate) fn set_sensor_value(
        &self,
        sensor: &str,
        timestamp: dipstick_proto::wkt::Timestamp,
        value: impl IntoValue,
    ) {
        let value = value.into_value();

        let mut sensors = self.sensors.write().unwrap();
        if let Some(status) = sensors.get_mut(sensor) {
            if status.value.as_ref() == Some(&value) {
                return;
            }

            status.timestamp = Some(timestamp);
            status.value = Some(value.clone());
        } else {
            sensors.insert(
                sensor.to_owned(),
                SensorStatus {
                    timestamp: Some(timestamp),
                    unit: String::new(),
                    value: Some(value.clone()),
                },
            );
        }

        let _ = self.tx.send(
            SensorEvent {
                timestamp: Some(timestamp),
                sensor: sensor.to_owned(),
                value: Some(value),
            }
            .into(),
        );
    }
}

impl Entity for Device {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for Device {
    const PACKAGE: &'static str = crate::PACKAGE;
    const KIND: &'static str = "Device";
}

async fn update_loop(device: Arc<Device>, variant: SharedDeviceVariant, poll_interval: Duration) {
    let mut poll_interval = tokio::time::interval(poll_interval);
    poll_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let mut started = false;
    loop {
        poll_interval.tick().await;
        if !started {
            match variant.start(&device).await {
                Ok(()) => started = true,
                Err(err) => {
                    tracing::error!(err = &err as &dyn std::error::Error, "device setup error");
                    continue;
                }
            }
        }

        if let Err(err) = variant.update(&device).await {
            tracing::error!(err = &err as &dyn std::error::Error, "device update error");
        }
    }
}
