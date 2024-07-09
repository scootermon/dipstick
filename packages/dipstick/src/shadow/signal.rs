use std::sync::RwLock;

pub struct Signal {
    value: RwLock<dipstick_proto::wkt::Value>,
}
