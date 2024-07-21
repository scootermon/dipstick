use std::any::TypeId;
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use dipstick_proto::core::v1::{EntityMetaSpec, EntityMetaStatus};

use crate::DependencyHandle;

pub type UniqueId = NonZeroU32;

pub trait Entity: Send + Sync + 'static {
    fn entity_meta(&self) -> &EntityMeta;

    fn entity_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl<T: Entity> Entity for Arc<T> {
    fn entity_meta(&self) -> &EntityMeta {
        (**self).entity_meta()
    }

    fn entity_type_id(&self) -> TypeId {
        (**self).entity_type_id()
    }
}

pub trait EntityKind {
    const PACKAGE: &'static str;
    const KIND: &'static str;
}

pub struct EntityMeta {
    unique_id: UniqueId,
    package: String,
    kind: String,
    key: Option<String>,
    created_at: SystemTime,
    annotations: RwLock<HashMap<String, String>>,
    dependencies: crate::dependency::DependencyTracker,
}

impl EntityMeta {
    pub fn new(unique_id: UniqueId, spec: EntityMetaSpec) -> Self {
        let EntityMetaSpec {
            unique_id: _,
            package,
            kind,
            key,
            annotations,
        } = spec;
        assert!(
            !package.is_empty()
                && !kind.is_empty()
                && !key.as_ref().is_some_and(|key| key.is_empty())
        );
        Self {
            unique_id,
            package,
            kind,
            key,
            created_at: SystemTime::now(),
            annotations: RwLock::new(annotations),
            dependencies: crate::dependency::DependencyTracker::new(unique_id),
        }
    }

    /// Adds `other` as a dependency of `self`.
    pub fn add_dependency(&self, other: &Self) -> DependencyHandle {
        self.dependencies.add_dependency(&other.dependencies)
    }

    pub(crate) fn unique_id(&self) -> UniqueId {
        self.unique_id
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

    pub fn spec(&self) -> EntityMetaSpec {
        EntityMetaSpec {
            unique_id: self.unique_id.get(),
            package: self.package.clone(),
            kind: self.kind.clone(),
            key: self.key.clone(),
            annotations: self.annotations.read().unwrap().clone(),
        }
    }

    pub fn status(&self) -> EntityMetaStatus {
        let (dependencies, dependents) = self.dependencies.to_proto();
        EntityMetaStatus {
            created_at: Some(self.created_at.into()),
            dependencies,
            dependents,
        }
    }

    pub fn to_proto(&self) -> dipstick_proto::core::v1::EntityMeta {
        dipstick_proto::core::v1::EntityMeta {
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedKey {
    pub package: String,
    pub kind: String,
    pub key: String,
}

impl QualifiedKey {
    pub fn from_spec(spec: &EntityMetaSpec) -> Option<Self> {
        let key = spec.key.clone()?;
        let package = spec.package.clone();
        let kind = spec.kind.clone();
        if package.is_empty() || kind.is_empty() {
            return None;
        }
        Some(Self { package, kind, key })
    }
}
