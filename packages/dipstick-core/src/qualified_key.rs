use dipstick_proto::core::v1::EntityMetaSpec;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QualifiedKey {
    pub package: String,
    pub kind: String,
    pub key: String,
}

impl QualifiedKey {
    pub fn _from_spec(spec: &EntityMetaSpec) -> Option<Self> {
        let key = spec.key.clone()?;
        let package = spec.package.clone();
        let kind = spec.kind.clone();
        if package.is_empty() || kind.is_empty() {
            return None;
        }
        Some(Self { package, kind, key })
    }
}
