use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
use dipstick_proto::gpio::v1::{
    ChipEntity, ChipSpec, ChipSpecVariant, ChipStatus, Level, PinStatus, SubscribeChipResponse,
};
use tokio::sync::{broadcast, mpsc, oneshot};
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Result, Status};

#[cfg(target_os = "linux")]
mod linux;

pub struct Chip {
    meta: EntityMeta,
    spec: ChipSpec,
    pins: Arc<PinMap>,
    variant: Variant,
}

impl Chip {
    pub async fn new(_core: &Core, meta: EntityMeta, mut spec: ChipSpec) -> Result<Arc<Self>> {
        let pins = Arc::new(PinMap::new());

        // Take it so we can use mutable references to both the spec and the inner
        // variant spec. We'll put it back at the end.
        let mut variant_spec = spec
            .chip_spec_variant
            .take()
            .ok_or_else(|| Status::invalid_argument("missing chip spec variant"))?;
        let variant = Variant::new(Arc::clone(&pins), &mut spec, &mut variant_spec).await?;
        spec.chip_spec_variant = Some(variant_spec);

        Ok(Arc::new(Self {
            meta,
            spec,
            pins,
            variant,
        }))
    }

    pub fn spec(&self) -> ChipSpec {
        self.spec.clone()
    }

    pub fn status(&self) -> ChipStatus {
        ChipStatus {
            pins: self.pins.to_proto(),
        }
    }

    pub fn to_proto(&self) -> ChipEntity {
        ChipEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }

    pub async fn shutdown(self) {
        self.variant.shutdown().await;
    }

    pub fn subscribe(&self) -> BroadcastStream<SubscribeChipResponse> {
        BroadcastStream::new(self.pins.tx.subscribe())
    }

    pub fn pin_status(&self, pin_id: &str) -> Option<PinStatus> {
        self.pins.pins.read().unwrap().get(pin_id).cloned()
    }

    pub async fn set_pin_level(&self, pin_id: String, logical: Level) -> Result<()> {
        self.variant.set_pin_level(pin_id, logical).await
    }
}

impl Entity for Chip {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for Chip {
    type Package = crate::Gpio;
    const KIND: &'static str = "Chip";
}

enum Variant {
    #[cfg(target_os = "linux")]
    Linux {
        sender: mpsc::Sender<linux::Message>,
    },
}

impl Variant {
    async fn new(
        pins: Arc<PinMap>,
        spec: &mut ChipSpec,
        variant_spec: &mut ChipSpecVariant,
    ) -> Result<Self> {
        match variant_spec {
            #[cfg(target_os = "linux")]
            ChipSpecVariant::Linux(linux) => {
                let (sender, receiver) = mpsc::channel(16); // TODO
                linux::spawn(receiver, pins, linux, &mut spec.pins).await?;
                Ok(Self::Linux { sender })
            }
            #[cfg(not(target_os = "linux"))]
            ChipSpecVariant::Linux(_) => {
                // mark a few things as used to avoid warnings
                let _ = || {
                    let _ = (spec, variant_spec);
                    mpsc::channel::<()>(0);
                    pins.add_pin("");
                    pins.set_pin_level("", SystemTime::now(), Level::Unspecified);
                };
                Err(Status::invalid_argument(
                    "linux gpio chip support not available",
                ))
            }
        }
    }

    async fn shutdown(self) {
        match self {
            #[cfg(target_os = "linux")]
            Self::Linux { sender } => {
                let _ = sender.send(linux::Message::Shutdown).await;
                sender.closed().await;
            }
        }
    }

    async fn set_pin_level(&self, pin_id: String, logical: Level) -> Result<()> {
        match self {
            #[cfg(target_os = "linux")]
            Self::Linux { sender } => {
                const CHANNEL_ERR_MSG: &str = "linux gpio channel closed";

                let permit = sender
                    .reserve()
                    .await
                    .map_err(|_err| Status::internal(CHANNEL_ERR_MSG))?;
                let (tx, rx) = oneshot::channel();
                permit.send(linux::Message::SetPinLevel {
                    id: pin_id,
                    logical,
                    result: tx,
                });
                rx.await
                    .map_err(|_err| Status::internal(CHANNEL_ERR_MSG))??;
                Ok(())
            }
            #[cfg(not(target_os = "linux"))]
            _ => {
                let _ = || oneshot::channel::<()>();
                let _ = (pin_id, logical);
                unreachable!()
            }
        }
    }
}

struct PinMap {
    pins: RwLock<HashMap<String, PinStatus>>,
    tx: broadcast::Sender<SubscribeChipResponse>,
}

impl PinMap {
    fn new() -> Self {
        let (tx, _) = broadcast::channel(16); // TODO
        Self {
            pins: RwLock::new(HashMap::new()),
            tx,
        }
    }

    pub fn add_pin(&self, id: &str) {
        let mut pins = self.pins.write().unwrap();
        if pins.contains_key(id) {
            return;
        }
        pins.insert(
            id.to_owned(),
            PinStatus {
                changed_at: None,
                logical: Level::Unspecified as _,
            },
        );
    }

    fn _remove_pin(&self, id: &str) -> bool {
        let mut pins = self.pins.write().unwrap();
        pins.remove(id).is_some()
    }

    pub fn set_pin_level(&self, id: &str, timestamp: SystemTime, logical: Level) {
        let mut pins = self.pins.write().unwrap();
        if let Some(status) = pins.get_mut(id) {
            let changed = status.logical() != logical;
            status.set_logical(logical);
            if changed {
                status.changed_at = Some(timestamp.into());
                // avoid cloning if no one is listening
                if self.tx.receiver_count() > 0 {
                    let _ = self.tx.send(SubscribeChipResponse {
                        pin_id: id.to_owned(),
                        status: Some(*status),
                    });
                }
            }
        } else {
            pins.insert(
                id.to_owned(),
                PinStatus {
                    changed_at: Some(timestamp.into()),
                    logical: logical as _,
                },
            );
        }
    }

    fn to_proto(&self) -> HashMap<String, PinStatus> {
        let pins = self.pins.read().unwrap();
        pins.clone()
    }
}
