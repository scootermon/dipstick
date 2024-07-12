use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use dipstick_core::{Entity, EntityMeta, IdContext, QualifiedId};
use dipstick_proto::gpio::v1::{
    ChipEntity, ChipSpec, ChipSpecVariant, ChipStatus, Level, PinStatus, SubscribeChipResponse,
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_util::sync::{CancellationToken, DropGuard};
use tokio_util::task::TaskTracker;

mod linux;

pub const ID_CONTEXT: IdContext = IdContext::full_ref(crate::PACKAGE, "Chip");

pub struct Chip {
    meta: EntityMeta,
    spec: ChipSpec,
    pins: Arc<PinMap>,
    tracker: TaskTracker,
    drop_guard: DropGuard,
}

impl Chip {
    pub async fn new(id: Option<QualifiedId<'static>>, spec: ChipSpec) -> anyhow::Result<Self> {
        let tracker = TaskTracker::new();
        let cancel_token = CancellationToken::new();
        let pins = Arc::new(PinMap::new());

        match &spec.chip_spec_variant {
            Some(ChipSpecVariant::Linux(linux_spec)) => {
                linux::spawn(
                    &tracker,
                    cancel_token.child_token(),
                    Arc::clone(&pins),
                    linux_spec,
                    &spec.pins,
                )
                .await?;
            }
            None => anyhow::bail!("missing chip spec variant"),
        }

        let meta = EntityMeta::new(id);
        Ok(Self {
            meta,
            spec,
            pins,
            tracker,
            drop_guard: cancel_token.drop_guard(),
        })
    }

    pub fn to_proto(&self) -> ChipEntity {
        ChipEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec.clone()),
            status: Some(ChipStatus {
                pins: self.pins.to_proto(),
            }),
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
}

impl Entity for Chip {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
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
                        status: Some(status.clone()),
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
