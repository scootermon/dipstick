use std::sync::Arc;

use dipstick_core::{Core, Package, PackageKind};
use dipstick_proto::core::v1::EntityMetaSpec;
use dipstick_proto::xcp::v1::{
    A2lSpec, CommandRequest, CommandResponse, CreateA2lRequest, CreateA2lResponse,
    CreateSessionRequest, CreateSessionResponse, GetA2lCharacteristicRequest,
    GetA2lCharacteristicResponse, GetA2lMeasurementRequest, GetA2lMeasurementResponse,
    GetA2lRequest, GetA2lResponse, GetSessionRequest, GetSessionResponse,
    ReadCharacteristicRequest, ReadCharacteristicResponse, ReadMeasurementRequest,
    ReadMeasurementResponse, SessionSpec, WriteCharacteristicRequest, WriteCharacteristicResponse,
    XcpServiceServer,
};
use futures::future::BoxFuture;
use futures::FutureExt;
use tonic::{Request, Response, Result};

pub use self::a2l::A2l;
pub use self::session::Session;

mod a2l;
mod protocol;
mod session;
mod transport;

pub struct XcpService {
    core: Arc<Core>,
}

impl XcpService {
    pub fn new(core: Arc<Core>) -> Arc<Self> {
        Arc::new(Self { core })
    }

    pub fn into_server(self: Arc<Self>) -> XcpServiceServer<Self> {
        XcpServiceServer::from_arc(self)
    }

    pub async fn create_a2l_impl(&self, meta: EntityMetaSpec, spec: A2lSpec) -> Result<Arc<A2l>> {
        let (meta, reservation) = self.core.new_entity_meta::<A2l>(meta)?;
        let a2l = A2l::new(&self.core, meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&a2l));
        Ok(a2l)
    }

    pub async fn create_session_impl(
        &self,
        meta: EntityMetaSpec,
        spec: SessionSpec,
    ) -> Result<Arc<Session>> {
        let (meta, reservation) = self.core.new_entity_meta::<Session>(meta)?;
        let session = Session::new(&self.core, meta, spec).await?;
        self.core.add_entity(reservation, Arc::clone(&session));
        Ok(session)
    }
}

impl Package for XcpService {
    fn package_name(&self) -> &'static str {
        Self::PACKAGE_NAME
    }
}

impl PackageKind for XcpService {
    const PACKAGE_NAME: &'static str = "xcp.v1";
}

impl dipstick_proto::xcp::v1::XcpService for XcpService {
    fn create_a2l<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateA2lRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateA2lResponse>>> {
        async move {
            let CreateA2lRequest { meta, spec } = request.into_inner();
            let a2l = self
                .create_a2l_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
                .await?;
            Ok(Response::new(CreateA2lResponse {
                a2l: Some(a2l.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_a2l<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetA2lRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetA2lResponse>>> {
        async move {
            let GetA2lRequest { selector } = request.into_inner();
            let a2l = self
                .core
                .select_entity::<A2l>(selector.unwrap_or_default())?;
            Ok(Response::new(GetA2lResponse {
                a2l: Some(a2l.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_a2l_measurement<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetA2lMeasurementRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetA2lMeasurementResponse>>> {
        async move {
            let GetA2lMeasurementRequest {
                selector,
                measurement_name,
            } = request.into_inner();
            let a2l = self
                .core
                .select_entity::<A2l>(selector.unwrap_or_default())?;
            let measurement = a2l.get_measurement(&measurement_name)?;
            Ok(Response::new(GetA2lMeasurementResponse {
                measurement: Some(measurement),
            }))
        }
        .boxed()
    }

    fn get_a2l_characteristic<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetA2lCharacteristicRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetA2lCharacteristicResponse>>> {
        async move {
            let GetA2lCharacteristicRequest {
                selector,
                characteristic_name,
            } = request.into_inner();
            let a2l = self
                .core
                .select_entity::<A2l>(selector.unwrap_or_default())?;
            let characteristic = a2l.get_characteristic(&characteristic_name)?;
            Ok(Response::new(GetA2lCharacteristicResponse {
                characteristic: Some(characteristic),
            }))
        }
        .boxed()
    }

    fn create_session<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CreateSessionRequest>,
    ) -> BoxFuture<'fut, Result<Response<CreateSessionResponse>>> {
        async move {
            let CreateSessionRequest { meta, spec } = request.into_inner();
            let session = self
                .create_session_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
                .await?;
            Ok(Response::new(CreateSessionResponse {
                session: Some(session.to_proto()),
            }))
        }
        .boxed()
    }

    fn get_session<'s: 'fut, 'fut>(
        &'s self,
        request: Request<GetSessionRequest>,
    ) -> BoxFuture<'fut, Result<Response<GetSessionResponse>>> {
        async move {
            let GetSessionRequest { selector } = request.into_inner();
            let session = self
                .core
                .select_entity::<Session>(selector.unwrap_or_default())?;
            Ok(Response::new(GetSessionResponse {
                session: Some(session.to_proto()),
            }))
        }
        .boxed()
    }

    fn command<'s: 'fut, 'fut>(
        &'s self,
        request: Request<CommandRequest>,
    ) -> BoxFuture<'fut, Result<Response<CommandResponse>>> {
        async move {
            let CommandRequest { selector, request } = request.into_inner();
            let session = self
                .core
                .select_entity::<Session>(selector.unwrap_or_default())?;

            let response = session.cto_command(&request.unwrap_or_default()).await?;
            Ok(Response::new(CommandResponse {
                response: Some(response),
            }))
        }
        .boxed()
    }

    fn read_measurement<'s: 'fut, 'fut>(
        &'s self,
        request: Request<ReadMeasurementRequest>,
    ) -> BoxFuture<'fut, Result<Response<ReadMeasurementResponse>>> {
        async move {
            let ReadMeasurementRequest {
                selector,
                a2l_selector,
                measurement_name,
            } = request.into_inner();
            let session = self
                .core
                .select_entity::<Session>(selector.unwrap_or_default())?;
            let a2l = self
                .core
                .select_entity::<A2l>(a2l_selector.unwrap_or_default())?;

            let measurement = a2l.get_measurement(&measurement_name)?;

            let (timestamp, value) = session.read_measurement(&measurement).await?;
            Ok(Response::new(ReadMeasurementResponse {
                timestamp: Some(timestamp),
                value: Some(value),
            }))
        }
        .boxed()
    }

    fn read_characteristic<'s: 'fut, 'fut>(
        &'s self,
        request: Request<ReadCharacteristicRequest>,
    ) -> BoxFuture<'fut, Result<Response<ReadCharacteristicResponse>>> {
        async move {
            let ReadCharacteristicRequest {
                selector,
                a2l_selector,
                characteristic_name,
            } = request.into_inner();
            let session = self
                .core
                .select_entity::<Session>(selector.unwrap_or_default())?;
            let a2l = self
                .core
                .select_entity::<A2l>(a2l_selector.unwrap_or_default())?;

            let characteristic = a2l.get_characteristic(&characteristic_name)?;
            let (timestamp, value) = session.read_characteristic(&characteristic).await?;
            Ok(Response::new(ReadCharacteristicResponse {
                timestamp: Some(timestamp),
                value: Some(value),
            }))
        }
        .boxed()
    }

    fn write_characteristic<'s: 'fut, 'fut>(
        &'s self,
        request: Request<WriteCharacteristicRequest>,
    ) -> BoxFuture<'fut, Result<Response<WriteCharacteristicResponse>>> {
        async move {
            let WriteCharacteristicRequest {
                selector,
                a2l_selector,
                characteristic_name,
                value,
            } = request.into_inner();
            let session = self
                .core
                .select_entity::<Session>(selector.unwrap_or_default())?;
            let a2l = self
                .core
                .select_entity::<A2l>(a2l_selector.unwrap_or_default())?;

            let characteristic = a2l.get_characteristic(&characteristic_name)?;
            session
                .write_characteristic(&characteristic, &value.unwrap_or_default())
                .await?;
            Ok(Response::new(WriteCharacteristicResponse {}))
        }
        .boxed()
    }
}
