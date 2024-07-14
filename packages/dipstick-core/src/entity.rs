use std::any::TypeId;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::RwLock;
use std::time::SystemTime;

use dipstick_proto::core::v1::{EntityMetaSpec, EntityMetaStatus};

use crate::QualifiedKey;

pub type UniqueId = NonZeroU32;

pub trait Entity: Send + Sync + 'static {
    fn entity_meta(&self) -> &EntityMeta;

    fn entity_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

pub trait EntityKind {
    const PACKAGE: &'static str;
    const KIND: &'static str;
}

pub struct EntityMeta {
    package: String,
    kind: String,
    key: Option<String>,
    unique_id: UniqueIdSlot,
    created_at: SystemTime,
    annotations: RwLock<HashMap<String, String>>,
}

impl EntityMeta {
    pub fn new(package: String, kind: String, spec: EntityMetaSpec) -> Self {
        assert!(!package.is_empty() && !kind.is_empty());
        let unique_id = UniqueId::new(spec.unique_id);
        Self {
            package,
            kind,
            key: spec.key,
            unique_id: UniqueIdSlot::new(unique_id),
            created_at: SystemTime::now(),
            annotations: RwLock::new(spec.annotations),
        }
    }

    pub fn set_annotations(&self, annotations: HashMap<String, String>) {
        *self.annotations.write().unwrap() = annotations;
    }

    pub(crate) fn set_unique_id(&self, unique_id: UniqueId) {
        self.unique_id.set(unique_id);
    }

    pub(crate) fn unique_id(&self) -> Option<UniqueId> {
        self.unique_id.get()
    }

    pub(crate) fn package(&self) -> &str {
        &self.package
    }

    pub(crate) fn kind(&self) -> &str {
        &self.kind
    }

    pub(crate) fn local_key(&self) -> Option<&str> {
        self.key.as_deref()
    }

    pub(crate) fn qualified_key(&self) -> Option<QualifiedKey> {
        Some(QualifiedKey {
            package: self.package.clone(),
            kind: self.kind.clone(),
            key: self.key.clone()?,
        })
    }

    pub fn to_proto(&self) -> dipstick_proto::core::v1::EntityMeta {
        let annotations = self.annotations.read().unwrap().clone();
        let spec = EntityMetaSpec {
            unique_id: self.unique_id.get_proto(),
            package: self.package.clone(),
            kind: self.kind.clone(),
            key: self.key.clone(),
            annotations,
        };
        let status = EntityMetaStatus {
            created_at: Some(self.created_at.into()),
            dependents: Vec::new(), // TODO
        };
        dipstick_proto::core::v1::EntityMeta {
            spec: Some(spec),
            status: Some(status),
        }
    }
}

struct UniqueIdSlot(RwLock<Option<UniqueId>>);

impl UniqueIdSlot {
    fn new(unique_id: Option<UniqueId>) -> Self {
        Self(RwLock::new(unique_id))
    }

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
