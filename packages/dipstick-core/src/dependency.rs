use std::sync::{Arc, RwLock, Weak};
use std::time::SystemTime;

use dipstick_proto::core::v1::EntityDependency;

use crate::UniqueId;

/// A handle to a dependency.
///
/// As long as this handle is alive, the dependency knows about the dependent.
#[must_use]
pub struct DependencyHandle(Arc<Link>);

impl DependencyHandle {
    pub fn dependency(&self) -> UniqueId {
        self.0.dependency
    }
}

pub struct DependencyTracker {
    unique_id: UniqueId,
    links: RwLock<LinkStorage>,
}

impl DependencyTracker {
    pub const fn new(unique_id: UniqueId) -> Self {
        Self {
            unique_id,
            links: RwLock::new(LinkStorage::new()),
        }
    }

    /// Returns (dependencies, dependents).
    pub fn to_proto(&self) -> (Vec<EntityDependency>, Vec<EntityDependency>) {
        let links = self.links.read().unwrap();
        let mut dependencies = Vec::new();
        let mut dependents = Vec::new();
        for link in links.iter() {
            if link.dependent == self.unique_id {
                dependencies.push(EntityDependency {
                    unique_id: link.dependency.get(),
                    created_at: Some(link.created_at.into()),
                });
            } else {
                dependents.push(EntityDependency {
                    unique_id: link.dependent.get(),
                    created_at: Some(link.created_at.into()),
                });
            }
        }
        (dependencies, dependents)
    }

    /// Adds `other` as a dependency of `self`.
    pub fn add_dependency(&self, other: &Self) -> DependencyHandle {
        let mut my_links = self.links.write().unwrap();
        if let Some(existing) = my_links.get(other.unique_id) {
            return DependencyHandle(existing);
        }

        let created_at = SystemTime::now();
        let mut other_links = other.links.write().unwrap();
        let link = Arc::new(Link {
            created_at,
            dependent: self.unique_id,
            dependency: other.unique_id,
        });
        my_links.push(&link);
        other_links.push(&link);
        DependencyHandle(link)
    }
}

/// Since the link is shared between dependent and dependency, it contains all
/// relevant information.
struct Link {
    created_at: SystemTime,
    dependent: UniqueId,
    dependency: UniqueId,
}

struct LinkStorage(Vec<Weak<Link>>);

impl LinkStorage {
    const fn new() -> Self {
        Self(Vec::new())
    }

    fn cleanup(&mut self) {
        self.0.retain(|weak| weak.strong_count() > 0);
    }

    fn iter(&self) -> impl Iterator<Item = Arc<Link>> + '_ {
        self.0.iter().filter_map(|weak| weak.upgrade())
    }

    fn get(&self, dependency: UniqueId) -> Option<Arc<Link>> {
        self.iter().find(|link| link.dependency == dependency)
    }

    fn push(&mut self, link: &Arc<Link>) {
        self.cleanup();
        self.0.push(Arc::downgrade(link));
    }
}
