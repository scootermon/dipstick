use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use dipstick_proto::core::v1::{EntitySelector, EntitySelectorVariant};
use tonic::{Result, Status};

pub use self::reservation::ReservationHandle;
use self::reservation::Reservations;
use crate::{Entity, EntityKind, EntityMeta, QualifiedKey, UniqueId};

mod reservation;

pub struct Registry {
    reservations: RwLock<Reservations>,
    inner: RwLock<Inner>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            reservations: RwLock::new(Reservations::new()),
            inner: RwLock::new(Inner::new()),
        }
    }

    pub fn reserve(&self, meta: &EntityMeta) -> Result<ReservationHandle> {
        let inner = self.inner.read().unwrap();
        // make sure the entity is not already registered
        if let Some(unique_id) = meta.unique_id() {
            if inner.has_unique_id(unique_id) {
                return Err(Status::already_exists(format!(
                    "unique id {unique_id} is already in use"
                )));
            }
        }
        let qualified_key = meta.qualified_key();
        if let Some(qualified_key) = &qualified_key {
            if inner.has_qualified_key(qualified_key) {
                return Err(Status::already_exists(format!(
                    "key {} is already in use",
                    qualified_key.key
                )));
            }
        }

        let mut reservations = self.reservations.write().unwrap();
        let unique_id = meta
            .unique_id()
            .unwrap_or_else(|| new_unique_id(&inner, &reservations));
        reservations.reserve(unique_id, qualified_key)
    }

    pub fn add_entity<T: Entity>(&self, reservation: ReservationHandle, entity: Arc<T>) {
        assert!(reservation.local_key() == entity.entity_meta().local_key());
        entity.entity_meta().set_unique_id(reservation.unique_id());
        let mut inner = self.inner.write().unwrap();
        inner.insert(entity);
    }

    pub fn select<T: EntityKind + Entity>(&self, selector: &EntitySelector) -> Result<Arc<T>> {
        self.select_opt(selector)
            .ok_or_else(|| Status::not_found("entity not found"))
    }

    fn select_opt<T: EntityKind + Entity>(&self, selector: &EntitySelector) -> Option<Arc<T>> {
        let entity = self.select_raw(selector, T::PACKAGE, T::KIND)?;
        if entity.entity_type_id() == TypeId::of::<T>() {
            // SAFETY: We just checked that the type ID matches.
            Some(unsafe { Arc::from_raw(Arc::into_raw(entity).cast()) })
        } else {
            None
        }
    }

    fn select_raw(
        &self,
        selector: &EntitySelector,
        package: &str,
        kind: &str,
    ) -> Option<Arc<dyn Entity>> {
        let inner = self.inner.read().unwrap();
        match &selector.entity_selector_variant {
            Some(EntitySelectorVariant::Key(key)) => inner.get_by_key(package, kind, key),
            Some(EntitySelectorVariant::UniqueId(unique_id)) => {
                UniqueId::new(*unique_id).and_then(|unique_id| inner.get_by_unique_id(unique_id))
            }
            None => None,
        }
    }

    pub fn visit_all(&self, mut f: impl FnMut(&Arc<dyn Entity>)) {
        let inner = self.inner.read().unwrap();
        for entity in inner.by_unique_id.values() {
            f(entity);
        }
    }
}

fn new_unique_id(inner: &Inner, reservations: &Reservations) -> UniqueId {
    loop {
        let candidate = rand::random();
        if inner.by_unique_id.contains_key(&candidate) {
            continue;
        }
        if reservations.is_unique_id_reserved(candidate) {
            continue;
        }
        return candidate;
    }
}

struct Inner {
    by_unique_id: HashMap<UniqueId, Arc<dyn Entity>>,
    by_key: HashMap<QualifiedKey, Arc<dyn Entity>>,
}

impl Inner {
    fn new() -> Self {
        Self {
            by_unique_id: HashMap::new(),
            by_key: HashMap::new(),
        }
    }

    fn insert(&mut self, entity: Arc<dyn Entity>) {
        let meta = entity.entity_meta();
        // at this point we must have a unique_id
        let unique_id = meta.unique_id().unwrap();
        tracing::debug!(
            package = meta.package(),
            kind = meta.kind(),
            key = meta.local_key(),
            unique_id,
            "new entity registered"
        );

        self.by_unique_id.insert(unique_id, Arc::clone(&entity));
        if let Some(key) = meta.qualified_key() {
            self.by_key.insert(key, Arc::clone(&entity));
        }
    }

    fn has_unique_id(&self, unique_id: UniqueId) -> bool {
        self.by_unique_id.contains_key(&unique_id)
    }

    fn has_qualified_key(&self, qualified_key: &QualifiedKey) -> bool {
        self.by_key.contains_key(qualified_key)
    }

    fn get_by_unique_id(&self, unique_id: UniqueId) -> Option<Arc<dyn Entity>> {
        self.by_unique_id.get(&unique_id).map(|e| Arc::clone(e))
    }

    fn get_by_key(&self, package: &str, kind: &str, key: &str) -> Option<Arc<dyn Entity>> {
        let qualified_key = QualifiedKey {
            package: package.to_owned(),
            kind: kind.to_owned(),
            key: key.to_owned(),
        };
        self.by_key.get(&qualified_key).map(|e| Arc::clone(e))
    }
}
