use std::any::TypeId;

pub trait Package: Send + Sync + 'static {
    fn package_name(&self) -> &'static str;
    fn package_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

pub trait PackageKind {
    const PACKAGE_NAME: &'static str;
}
