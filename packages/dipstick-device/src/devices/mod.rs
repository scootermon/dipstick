use std::sync::Arc;

use dipstick_core::Core;
use dipstick_proto::device::v1::DeviceSpecVariant;
use futures::future::BoxFuture;
use tonic::Result;

pub use self::ina2xx::Ina2xx;
use crate::Device;

mod ina2xx;

pub trait DeviceVariant: Send + Sync + 'static {
    fn spec(&self) -> DeviceSpecVariant;

    fn start<'s: 'fut, 'fut>(&'s self, device: &'fut Device) -> BoxFuture<'fut, Result<()>>;
    // TODO: shutdown
    #[allow(dead_code)]
    fn stop<'s: 'fut, 'fut>(&'s self, device: &'fut Device) -> BoxFuture<'fut, Result<()>>;

    fn call_action<'s: 'fut, 'fut>(
        &'s self,
        device: &'fut Device,
        action: &'fut str,
    ) -> BoxFuture<'fut, Result<()>>;
    fn update<'s: 'fut, 'fut>(&'s self, device: &'fut Device) -> BoxFuture<'fut, Result<()>>;
}

pub type SharedDeviceVariant = Arc<dyn DeviceVariant>;

pub fn create_variant(
    core: &Core,
    device: &Device,
    spec: DeviceSpecVariant,
) -> Result<SharedDeviceVariant> {
    match spec {
        DeviceSpecVariant::Ina2xx(spec) => {
            Ina2xx::new(core, device, spec).map(|variant| Arc::new(variant) as _)
        }
    }
}
