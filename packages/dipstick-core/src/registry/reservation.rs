use std::sync::{Arc, Weak};

use tonic::{Result, Status};

use crate::{QualifiedKey, UniqueId};

pub struct ReservationStorage(Vec<Weak<Reservation>>);

impl ReservationStorage {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_key_reserved(&self, key: &QualifiedKey) -> bool {
        self.iter()
            .any(|reservation| reservation.qualified_key.as_ref() == Some(key))
    }

    pub fn is_unique_id_reserved(&self, unique_id: UniqueId) -> bool {
        self.iter()
            .any(|reservation| reservation.unique_id == unique_id)
    }

    pub fn reserve(
        &mut self,
        unique_id: UniqueId,
        qualified_key: Option<QualifiedKey>,
    ) -> Result<ReservationHandle> {
        self.cleanup();

        if self.is_unique_id_reserved(unique_id) {
            return Err(Status::already_exists(format!(
                "unique id {unique_id} is already reserved"
            )));
        }
        if let Some(key) = &qualified_key {
            if self.is_key_reserved(key) {
                return Err(Status::already_exists(format!(
                    "key '{}' is already reserved",
                    key.key
                )));
            }
        }

        let reservation = Reservation {
            unique_id,
            qualified_key,
        };
        let handle = ReservationHandle(Arc::new(reservation));
        self.0.push(Arc::downgrade(&handle.0));
        Ok(handle)
    }

    fn cleanup(&mut self) {
        self.0.retain(|weak| weak.strong_count() > 0);
    }

    fn iter(&self) -> impl Iterator<Item = Arc<Reservation>> + '_ {
        self.0.iter().filter_map(|weak| weak.upgrade())
    }
}

#[must_use]
pub struct ReservationHandle(Arc<Reservation>);

impl ReservationHandle {
    pub fn unique_id(&self) -> UniqueId {
        self.0.unique_id
    }

    pub(crate) fn qualified_key(&self) -> Option<&QualifiedKey> {
        self.0.qualified_key.as_ref()
    }

    pub fn local_key(&self) -> Option<&str> {
        self.0.qualified_key.as_ref().map(|key| key.key.as_str())
    }
}

struct Reservation {
    unique_id: UniqueId,
    qualified_key: Option<QualifiedKey>,
}
