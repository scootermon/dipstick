use dipstick_core::{Entity, EntityKind, EntityMeta};
use dipstick_proto::xcp::v1::{SessionEntity, SessionSpec, SessionStatus};

pub struct Session {
    meta: EntityMeta,
    spec: SessionSpec,
}

impl Session {
    pub fn spec(&self) -> SessionSpec {
        self.spec.clone()
    }

    pub fn status(&self) -> SessionStatus {
        SessionStatus {}
    }

    pub fn to_proto(&self) -> SessionEntity {
        SessionEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }
}

impl Entity for Session {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for Session {
    const PACKAGE: &'static str = crate::PACKAGE;
    const KIND: &'static str = "Session";
}
