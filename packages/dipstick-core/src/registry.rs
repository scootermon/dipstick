use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock, Weak};

use crate::{Entity, EntitySelector, QualifiedId, UniqueId};

pub struct Registry {
    reservations: RwLock<Vec<Weak<QualifiedId<'static>>>>,
    inner: RwLock<Inner>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            reservations: RwLock::new(Vec::new()),
            inner: RwLock::new(Inner::new()),
        }
    }

    pub fn add_entity<T: Entity>(&self, entity: Arc<T>) -> anyhow::Result<()> {
        if let Some(id) = entity.entity_meta().id() {
            let reservations = self.reservations.read().unwrap();
            let is_reserved = reservations.iter().any(|weak| {
                weak.upgrade()
                    .is_some_and(|existing_id| *existing_id == *id)
            });
            if is_reserved {
                anyhow::bail!("id is reserved");
            }
        }

        let mut inner = self.inner.write().unwrap();
        inner.insert(entity)?;
        Ok(())
    }

    pub fn add_entity_with_reservation<T: Entity>(
        &self,
        reservation: ReservationHandle,
        entity: Arc<T>,
    ) {
        let mut inner = self.inner.write().unwrap();
        drop(reservation);
        inner.insert(entity).unwrap();
    }

    pub fn reserve_id(&self, id: QualifiedId<'static>) -> Option<ReservationHandle> {
        let mut reservations = self.reservations.write().unwrap();

        let mut already_reserved = false;
        // we use the chance to clean up dead weak references at the same time
        reservations.retain(|weak| {
            if let Some(existing_id) = weak.upgrade() {
                if *existing_id == id {
                    already_reserved = true;
                }
                // retain
                true
            } else {
                // drop
                false
            }
        });
        if already_reserved {
            return None;
        }

        // we don't have an existing reservation, but the registry may already have the
        // ID
        {
            let inner = self.inner.read().unwrap();
            if inner.has_id(&id) {
                // the ID is already in use
                return None;
            }
        }

        let handle = ReservationHandle { id: Arc::new(id) };
        reservations.push(Arc::downgrade(&handle.id));
        Some(handle)
    }

    pub fn get_by_selector<T>(&self, selector: &EntitySelector) -> Option<Arc<T>>
    where
        T: Entity,
    {
        let entity = self.get_raw_by_selector(selector)?;
        if entity.type_id() == TypeId::of::<T>() {
            // SAFETY: We just checked that the type ID matches.
            Some(unsafe { Arc::from_raw(Arc::into_raw(entity).cast()) })
        } else {
            None
        }
    }

    fn get_raw_by_selector(&self, selector: &EntitySelector) -> Option<Arc<dyn Entity>> {
        let inner = self.inner.read().unwrap();
        let entity = inner.get_by_selector(selector)?;
        Some(Arc::clone(entity))
    }
}

pub struct ReservationHandle {
    id: Arc<QualifiedId<'static>>,
}

struct Inner {
    by_unique_id: HashMap<UniqueId, Arc<dyn Entity>>,
    by_id: HashSet<EntityById>,
}

impl Inner {
    fn new() -> Self {
        Self {
            by_unique_id: HashMap::new(),
            by_id: HashSet::new(),
        }
    }

    fn insert(&mut self, entity: Arc<dyn Entity>) -> anyhow::Result<()> {
        let meta = entity.entity_meta();

        if self.by_id.contains(EntityById::new_ref(&entity)) {
            anyhow::bail!("id already exists");
        }

        let unique_id = loop {
            let candidate = rand::random();
            if !self.by_unique_id.contains_key(&candidate) {
                break candidate;
            }
        };
        meta.set_unique_id(unique_id);

        self.by_unique_id.insert(unique_id, Arc::clone(&entity));
        self.by_id.insert(EntityById(entity));
        Ok(())
    }

    fn get_by_selector(&self, selector: &EntitySelector) -> Option<&Arc<dyn Entity>> {
        match selector {
            EntitySelector::Id(id) => self.by_id.get(id).map(|by_id| &by_id.0),
            EntitySelector::UniqueId(unique_id) => self.by_unique_id.get(unique_id),
        }
    }

    fn has_id(&self, id: &QualifiedId) -> bool {
        self.by_id.contains(id)
    }
}

#[repr(transparent)]
struct EntityById(Arc<dyn Entity>);

impl EntityById {
    #[inline]
    fn new_ref(entity: &Arc<dyn Entity>) -> &Self {
        assert!(entity.entity_meta().id().is_some());
        // SAFETY: `EntityById` is a transparent wrapper around `Arc<dyn Entity>`.
        unsafe { &*(entity as *const Arc<dyn Entity> as *const EntityById) }
    }

    #[inline]
    fn id(&self) -> &QualifiedId<'static> {
        self.0.entity_meta().id().unwrap()
    }
}

impl PartialEq for EntityById {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for EntityById {}

impl Hash for EntityById {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl<'a> Borrow<QualifiedId<'a>> for EntityById {
    fn borrow(&self) -> &QualifiedId<'a> {
        self.id()
    }
}
