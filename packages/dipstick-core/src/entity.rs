use std::any::TypeId;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::SystemTime;

pub use self::id::{IdContext, QualifiedId, UniqueId};
pub use self::selector::EntitySelector;

mod id;
mod selector;

pub trait Entity: Send + Sync + 'static {
    fn entity_meta(&self) -> &EntityMeta;

    fn entity_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

pub struct EntityMeta {
    id: Option<QualifiedId<'static>>,
    unique_id: UniqueIdSlot,
    created_at: SystemTime,
    annotations: RwLock<HashMap<String, String>>,
}

impl EntityMeta {
    pub fn new(id: Option<QualifiedId<'static>>) -> Self {
        Self {
            id,
            unique_id: UniqueIdSlot(RwLock::new(None)),
            created_at: SystemTime::now(),
            annotations: RwLock::new(HashMap::new()),
        }
    }

    pub fn set_annotations(&self, annotations: HashMap<String, String>) {
        *self.annotations.write().unwrap() = annotations;
    }

    pub(crate) fn set_unique_id(&self, unique_id: UniqueId) {
        self.unique_id.set(unique_id);
    }

    pub fn id(&self) -> Option<&QualifiedId<'static>> {
        self.id.as_ref()
    }

    pub fn to_proto(&self) -> dipstick_proto::core::v1::EntityMeta {
        let id = self
            .id
            .as_ref()
            .map_or_else(String::new, |id| id.to_string());
        let annotations = self.annotations.read().unwrap().clone();
        dipstick_proto::core::v1::EntityMeta {
            id,
            unique_id: self.unique_id.get_proto(),
            created_at: Some(self.created_at.into()),
            annotations,
        }
    }
}

struct UniqueIdSlot(RwLock<Option<UniqueId>>);

impl UniqueIdSlot {
    fn set(&self, unique_id: UniqueId) {
        *self.0.write().unwrap() = Some(unique_id);
    }

    fn get(&self) -> Option<UniqueId> {
        self.0.read().unwrap().clone()
    }

    fn get_proto(&self) -> u32 {
        self.get().map_or(0, |id| id.get())
    }
}
