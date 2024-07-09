use std::sync::Arc;

use dipstick_proto::shadow::v1::{
    AddSignalRequest,
    AddSignalResponse,
    CreateShadowRequest,
    CreateShadowResponse,
    DestroyShadowRequest,
    DestroyShadowResponse,
    ListShadowsRequest,
    ListShadowsResponse,
    ListSignalsRequest,
    ListSignalsResponse,
    RemoveSignalRequest,
    RemoveSignalResponse,
    ShadowService,
    ShadowServiceServer,
    SubscribeChangesRequest,
    SubscribeChangesResponse,
};
use futures::stream::BoxStream;
use tonic::{Request, Response, Status};

use self::single_shadow::SingleShadow;
use crate::core::registry::Registry;

mod single_shadow;
mod signal;

pub struct Shadow {
    shadows: Registry<SingleShadow>,
}

impl Shadow {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            shadows: Registry::new(),
        })
    }

    pub fn into_server(self: Arc<Self>) -> ShadowServiceServer<Self> {
        ShadowServiceServer::from_arc(self)
    }
}

#[async_trait::async_trait]
impl ShadowService for Shadow {
    async fn create_shadow(
        &self,
        request: Request<CreateShadowRequest>,
    ) -> tonic::Result<Response<CreateShadowResponse>> {
        todo!()
    }

    async fn destroy_shadow(
        &self,
        request: Request<DestroyShadowRequest>,
    ) -> tonic::Result<Response<DestroyShadowResponse>> {
        todo!()
    }

    async fn list_shadows(
        &self,
        request: Request<ListShadowsRequest>,
    ) -> tonic::Result<Response<ListShadowsResponse>> {
        todo!()
    }

    async fn add_signal(
        &self,
        request: Request<AddSignalRequest>,
    ) -> tonic::Result<Response<AddSignalResponse>> {
        todo!()
    }

    async fn remove_signal(
        &self,
        request: Request<RemoveSignalRequest>,
    ) -> tonic::Result<Response<RemoveSignalResponse>> {
        todo!()
    }

    async fn list_signals(
        &self,
        request: Request<ListSignalsRequest>,
    ) -> tonic::Result<Response<ListSignalsResponse>> {
        todo!()
    }

    type SubscribeChangesStream = BoxStream<'static, tonic::Result<SubscribeChangesResponse>>;

    async fn subscribe_changes(
        &self,
        request: Request<SubscribeChangesRequest>,
    ) -> tonic::Result<Response<Self::SubscribeChangesStream>> {
        todo!()
    }
}
