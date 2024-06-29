use std::sync::{Arc, RwLock};

use rand::Rng;
use tonic::Status;

pub struct Registry<T> {
    entries: RwLock<Vec<(u64, Arc<T>)>>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(Vec::new()),
        }
    }

    pub fn add_with_factory(&self, factory: impl FnOnce(u64) -> T) -> Arc<T> {
        let mut entries = self.entries.write().unwrap();
        let mut id;
        loop {
            id = rand::thread_rng().gen();
            if entries.iter().any(|(entry_id, _)| *entry_id == id) {
                continue;
            }
            break;
        }
        let entry = Arc::new(factory(id));
        entries.push((id, Arc::clone(&entry)));
        entry
    }

    pub fn get(&self, id: u64) -> tonic::Result<Arc<T>> {
        let entries = self.entries.read().unwrap();
        let Some((_, entry)) = entries.iter().find(|(entry_id, _)| *entry_id == id) else {
            return Err(Status::not_found(format!("id {id:?} not found")));
        };
        Ok(Arc::clone(entry))
    }
}
