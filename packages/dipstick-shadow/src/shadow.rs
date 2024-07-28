use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
use dipstick_proto::shadow::v1::{
    ShadowEntity, ShadowSpec, ShadowStatus, SignalSpecVariant, SignalStatus,
};
use dipstick_proto::wkt::{Timestamp, Value};
use tonic::Result;

use crate::listeners::Listeners;

pub struct Shadow {
    meta: EntityMeta,
    spec: RwLock<ShadowSpec>,
    signals: RwLock<HashMap<String, SignalStatus>>,
    listeners: ListenersCell,
}

impl Shadow {
    pub(crate) async fn new(core: &Core, meta: EntityMeta, spec: ShadowSpec) -> Result<Arc<Self>> {
        let this = Arc::new(Self {
            meta,
            spec: RwLock::new(ShadowSpec::default()),
            signals: RwLock::new(HashMap::new()),
            listeners: ListenersCell::new(),
        });
        Arc::clone(&this).update_spec(core, spec).await?;
        Ok(this)
    }

    async fn update_spec(self: Arc<Self>, core: &Core, mut new_spec: ShadowSpec) -> Result<()> {
        self.listeners.clear().await;

        let mut builder = Listeners::builder(core, Arc::clone(&self));
        for (signal_id, signal_spec) in new_spec.signals.iter_mut() {
            self.add_signal(signal_id);
            match signal_spec.signal_spec_variant.as_mut() {
                Some(SignalSpecVariant::Gpio(spec)) => {
                    builder.add_gpio_signal(signal_id.clone(), spec)?;
                }
                Some(SignalSpecVariant::A2lCharacteristic(spec)) => {
                    builder.add_a2l_characteristic_signal(signal_id.clone(), spec)?;
                }
                Some(SignalSpecVariant::A2lMeasurement(spec)) => {
                    builder.add_a2l_measurement_signal(signal_id.clone(), spec)?;
                }
                Some(SignalSpecVariant::DeviceSensor(_spec)) => {
                    todo!() // TODO
                }
                None => {
                    return Err(tonic::Status::invalid_argument(
                        "missing signal spec variant",
                    ))
                }
            }
        }
        *self.spec.write().unwrap() = new_spec;
        self.listeners.set(builder.build()).await;
        Ok(())
    }

    pub fn spec(&self) -> ShadowSpec {
        self.spec.read().unwrap().clone()
    }

    pub fn status(&self) -> ShadowStatus {
        let signals = self.signals.read().unwrap().clone();
        ShadowStatus { signals }
    }

    pub fn to_proto(&self) -> ShadowEntity {
        ShadowEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }

    fn add_signal(&self, signal_id: &str) {
        let mut signals = self.signals.write().unwrap();
        if signals.contains_key(signal_id) {
            return;
        }
        tracing::trace!(signal_id, "adding new signal");
        signals.insert(signal_id.to_owned(), SignalStatus::default());
    }

    pub fn set_signal_value(&self, signal_id: &str, timestamp: Timestamp, value: Value) {
        let mut signals = self.signals.write().unwrap();
        if let Some(signal) = signals.get_mut(signal_id) {
            tracing::trace!(signal_id, ?value, "updating signal value");
            signal.changed_at = Some(timestamp);
            signal.value = Some(value);
            signal.update_count += 1;
        } else {
            tracing::trace!(signal_id, ?value, "adding new signal value");
            signals.insert(
                signal_id.to_owned(),
                SignalStatus {
                    changed_at: Some(timestamp),
                    value: Some(value),
                    update_count: 1,
                },
            );
        }
    }
}

impl Entity for Shadow {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for Shadow {
    type Package = crate::ShadowService;
    const KIND: &'static str = "Shadow";
}

struct ListenersCell(RwLock<Option<Listeners>>);

impl ListenersCell {
    fn new() -> Self {
        Self(RwLock::new(None))
    }

    async fn clear(&self) {
        let listeners = self.0.write().unwrap().take();
        if let Some(listeners) = listeners {
            listeners.close().await;
        }
    }

    async fn set(&self, listeners: Listeners) {
        self.clear().await;
        self.0.write().unwrap().replace(listeners);
    }
}
