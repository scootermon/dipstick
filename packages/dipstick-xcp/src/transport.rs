use std::sync::Arc;

use dipstick_can::Bus;
use dipstick_proto::xcp::v1::{CanTransportSpec, XcpTransportSpec};
use tonic::Result;

pub enum Transport {
    Can(CanTransport),
}

impl Transport {
    pub async fn new(spec: &mut XcpTransportSpec) -> Result<Self> {
        match spec {
            XcpTransportSpec::Can(can) => CanTransport::new(can).await.map(Self::Can),
        }
    }
}

struct CanTransport {
    bus: Arc<Bus>,
}

impl CanTransport {
    pub async fn new(spec: &mut CanTransportSpec) -> Result<Self> {
        let bus = todo!(); // TODO
        Ok(Self { bus })
    }
}
