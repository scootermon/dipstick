include!("../../../_gen/dipstick.core.v1.rs");

pub use self::core_service_client::CoreServiceClient;
pub use self::core_service_server::{CoreService, CoreServiceServer};
pub use self::entity_selector::EntitySelectorVariant;
pub use self::file::FileVariant;

mod selector_impl;
