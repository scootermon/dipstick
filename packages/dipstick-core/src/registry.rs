use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use dipstick_proto::core::v1::{EntitySelector, EntitySelectorVariant};
use tonic::{Result, Status};

pub use self::reservation::ReservationHandle;
use self::reservation::ReservationStorage;
use crate::{Entity, EntityKind, Package, PackageKind, QualifiedKey, UniqueId};

mod reservation;

pub struct Registry {
    packages: RwLock<Vec<Arc<dyn Package>>>,
    reservations: RwLock<ReservationStorage>,
    inner: RwLock<Inner>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            packages: RwLock::new(Vec::new()),
            reservations: RwLock::new(ReservationStorage::new()),
            inner: RwLock::new(Inner::new()),
        }
    }

    pub fn package<T: Package + PackageKind>(&self) -> Result<Arc<T>> {
        self.package_opt()
            .ok_or_else(|| Status::not_found(format!("package {} not found", T::PACKAGE_NAME)))
    }

    fn package_opt<T: Package + PackageKind>(&self) -> Option<Arc<T>> {
        let package = self.package_raw(T::PACKAGE_NAME)?;
        if package.package_type_id() == TypeId::of::<T>() {
            // SAFETY: We just checked that the type ID matches.
            Some(unsafe { Arc::from_raw(Arc::into_raw(package).cast()) })
        } else {
            None
        }
    }

    fn package_raw(&self, name: &str) -> Option<Arc<dyn Package>> {
        let packages = self.packages.read().unwrap();
        packages
            .iter()
            .find(|package| package.package_name() == name)
            .map(Arc::clone)
    }

    /// Registers a package with the registry.
    ///
    /// # Panics
    ///
    /// Panics if a package with the same name is already registered.
    pub fn add_package<T: Package>(&self, package: Arc<T>) {
        let mut packages = self.packages.write().unwrap();
        if packages
            .iter()
            .any(|p| p.package_name() == package.package_name())
        {
            panic!("package {} is already registered", package.package_name());
        }
        packages.push(package);
    }

    pub fn reserve(
        &self,
        unique_id: Option<UniqueId>,
        qualified_key: Option<QualifiedKey>,
    ) -> Result<ReservationHandle> {
        let inner = self.inner.read().unwrap();
        // make sure the entity is not already registered
        if let Some(unique_id) = unique_id {
            if inner.has_unique_id(unique_id) {
                return Err(Status::already_exists(format!(
                    "unique id {unique_id} is already in use"
                )));
            }
        }
        if let Some(qualified_key) = &qualified_key {
            if inner.has_qualified_key(qualified_key) {
                return Err(Status::already_exists(format!(
                    "key {qualified_key} is already in use"
                )));
            }
        }

        let mut reservations = self.reservations.write().unwrap();
        let unique_id = unique_id.unwrap_or_else(|| new_unique_id(&inner, &reservations));
        let handle = reservations.reserve(unique_id, qualified_key)?;
        tracing::debug!(unique_id = ?handle.unique_id(), qualified_key = ?handle.qualified_key(), "reserved entity");
        Ok(handle)
    }

    pub fn add_entity<T: Entity>(&self, reservation: ReservationHandle, entity: Arc<T>) {
        reservation.assert_matches(entity.entity_meta());
        let mut inner = self.inner.write().unwrap();
        inner.insert(entity);
    }

    pub fn select<T: EntityKind + Entity>(&self, selector: &EntitySelector) -> Result<Arc<T>> {
        match self.select_opt(selector) {
            Some(entity) => Ok(entity),
            None => {
                let entity_kind = format!("{}/{}", T::Package::PACKAGE_NAME, T::KIND);
                let status = match &selector.entity_selector_variant {
                    Some(EntitySelectorVariant::Key(key)) => {
                        Status::not_found(format!("{entity_kind} with key {key:?} not found"))
                    }
                    Some(EntitySelectorVariant::UniqueId(unique_id)) => Status::not_found(format!(
                        "{entity_kind} with unique id {unique_id} not found"
                    )),
                    None => Status::invalid_argument(format!(
                        "{entity_kind} selector variant is missing"
                    )),
                };
                Err(status)
            }
        }
    }

    fn select_opt<T: EntityKind + Entity>(&self, selector: &EntitySelector) -> Option<Arc<T>> {
        let entity = self.select_raw(selector, T::Package::PACKAGE_NAME, T::KIND)?;
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

    pub fn force_remove_all_entities(&self) {
        let mut inner = self.inner.write().unwrap();
        // TODO: actually stop entities
        inner.by_unique_id.clear();
        inner.by_key.clear();
    }
}

fn new_unique_id(inner: &Inner, reservations: &ReservationStorage) -> UniqueId {
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
        tracing::info!(
            package = meta.package(),
            kind = meta.kind(),
            key = meta.local_key(),
            unique_id = meta.unique_id(),
            "new entity registered"
        );

        self.by_unique_id
            .insert(meta.unique_id(), Arc::clone(&entity));
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
        self.by_unique_id.get(&unique_id).map(Arc::clone)
    }

    fn get_by_key(&self, package: &str, kind: &str, key: &str) -> Option<Arc<dyn Entity>> {
        let qualified_key = QualifiedKey {
            package: package.to_owned(),
            kind: kind.to_owned(),
            key: key.to_owned(),
        };
        self.by_key.get(&qualified_key).map(Arc::clone)
    }
}
