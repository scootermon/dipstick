use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
use dipstick_proto::gpio::v1::{
    ChipEntity, ChipSpec, ChipSpecVariant, ChipStatus, Level, PinStatus, SubscribeChipResponse,
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use tonic::{Result, Status};

#[cfg(target_os = "linux")]
mod linux;

pub struct Chip {
    meta: EntityMeta,
    spec: ChipSpec,
    pins: Arc<PinMap>,
    tracker: TaskTracker,
    drop_guard: DropGuard,
}

impl Chip {
    pub async fn new(core: &Core, meta: EntityMeta, mut spec: ChipSpec) -> Result<Arc<Self>> {
        let cancel_token = core.new_cancel_token();
        let tracker = TaskTracker::new();
        let pins = Arc::new(PinMap::new());

        match &mut spec.chip_spec_variant {
            #[cfg(target_os = "linux")]
            Some(ChipSpecVariant::Linux(linux_spec)) => {
                linux::spawn(
                    &tracker,
                    cancel_token.clone(),
                    Arc::clone(&pins),
                    linux_spec,
                    &mut spec.pins,
                )
                .await?;
            }
            #[cfg(not(target_os = "linux"))]
            Some(ChipSpecVariant::Linux(_)) => {
                return Err(Status::unimplemented("linux chip support not available"))
            }
            None => return Err(Status::invalid_argument("missing chip spec variant")),
        }

        Ok(Arc::new(Self {
            meta,
            spec,
            pins,
            tracker,
            drop_guard: cancel_token.drop_guard(),
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
        drop(self.drop_guard);
        self.tracker.close();
        self.tracker.wait().await;
    }

    pub fn subscribe(&self) -> BroadcastStream<SubscribeChipResponse> {
        BroadcastStream::new(self.pins.tx.subscribe())
    }

    pub fn pin_status(&self, pin_id: &str) -> Option<PinStatus> {
        self.pins.pins.read().unwrap().get(pin_id).cloned()
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
