use std::sync::{Arc, RwLock};
use std::time::Duration;

use bytes::{Bytes, BytesMut};
use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
use dipstick_proto::xcp::v1::{
    A2lFullCharacteristic, A2lMeasurement, ByteOrder, CtoConnectReqData, CtoConnectRespData,
    CtoDownloadReqData, CtoReq, CtoReqPid, CtoResp, CtoRespData, CtoSetMtaReqData,
    CtoShortUploadReqData, SessionEntity, SessionSpec, SessionStatus,
};
use tokio::sync::Mutex;
use tonic::{Result, Status};

use crate::transport::Transport;

const DEFAULT_CTO_TIMEOUT: dipstick_proto::wkt::Duration = dipstick_proto::wkt::Duration {
    seconds: 0,
    nanos: 500_000_000,
};

pub struct Session {
    meta: EntityMeta,
    spec: SessionSpec,
    transport: Transport,
    connect_data: RwLock<Option<CtoConnectRespData>>,
    cto_guard: Mutex<()>,
}

impl Session {
    pub async fn new(core: &Core, meta: EntityMeta, mut spec: SessionSpec) -> Result<Arc<Self>> {
        spec.no_auto_reconnect.get_or_insert(false);
        spec.cto_timeout.get_or_insert(DEFAULT_CTO_TIMEOUT);
        if Duration::try_from(spec.cto_timeout.unwrap()).is_err() {
            return Err(Status::invalid_argument("cto timeout invalid"));
        }

        let transport_spec = spec
            .xcp_transport_spec
            .as_mut()
            .ok_or_else(|| Status::invalid_argument("missing xcp transport spec"))?;
        let transport = Transport::new(core, &meta, transport_spec).await?;
        Ok(Arc::new(Self {
            meta,
            spec,
            transport,
            connect_data: RwLock::new(None),
            cto_guard: Mutex::new(()),
        }))
    }

    pub fn spec(&self) -> SessionSpec {
        self.spec.clone()
    }

    pub fn status(&self) -> SessionStatus {
        let connect_data = self.connect_data.read().unwrap().clone();
        SessionStatus { connect_data }
    }

    pub fn to_proto(&self) -> SessionEntity {
        SessionEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }

    pub async fn read_measurement(
        &self,
        measurement: &A2lMeasurement,
    ) -> Result<(dipstick_proto::wkt::Timestamp, dipstick_proto::wkt::Value)> {
        let address_extension = measurement
            .ecu_address_extension()
            .try_into()
            .map_err(|_| Status::invalid_argument("ecu address extension invalid"))?;
        let length = u32::from(crate::a2l::codec::data_type_len(measurement.datatype()));
        let (timestamp, mut data) = self
            .cto_short_upload(measurement.ecu_address(), address_extension, length)
            .await?;
        let value = crate::a2l::codec::decode_data_type(
            &mut data,
            measurement.datatype(),
            measurement.byte_order(),
        )
        .map_err(|err| Status::internal(format!("measurement data invalid: {err}")))?;

        Ok((timestamp, value))
    }

    pub async fn read_characteristic(
        &self,
        characteristic: &A2lFullCharacteristic,
    ) -> Result<(dipstick_proto::wkt::Timestamp, dipstick_proto::wkt::Value)> {
        let A2lFullCharacteristic {
            characteristic: Some(characteristic),
            record_layout: Some(record_layout),
        } = characteristic
        else {
            return Err(Status::invalid_argument("invalid characteristic"));
        };
        let address_extension = characteristic
            .ecu_address_extension()
            .try_into()
            .map_err(|_| Status::invalid_argument("ecu address extension invalid"))?;
        let length = u32::from(crate::a2l::codec::record_len(record_layout));
        let (timestamp, mut data) = self
            .cto_short_upload(characteristic.address, address_extension, length)
            .await?;

        let value =
            crate::a2l::codec::decode_record(&mut data, record_layout, characteristic.byte_order())
                .map_err(|err| Status::internal(format!("characteristic data invalid: {err}")))?;

        Ok((timestamp, value))
    }

    pub async fn write_characteristic(
        &self,
        characteristic: &A2lFullCharacteristic,
        value: &dipstick_proto::wkt::Value,
    ) -> Result<()> {
        let A2lFullCharacteristic {
            characteristic: Some(characteristic),
            record_layout: Some(record_layout),
        } = characteristic
        else {
            return Err(Status::invalid_argument("invalid characteristic"));
        };
        let address_extension = characteristic
            .ecu_address_extension()
            .try_into()
            .map_err(|_| Status::invalid_argument("ecu address extension invalid"))?;
        let mut data = BytesMut::new();
        crate::a2l::codec::encode_record(
            &mut data,
            record_layout,
            characteristic.byte_order(),
            value,
        )
        .map_err(|err| {
            Status::invalid_argument(format!("unable to encode characteristic data: {err}"))
        })?;
        self.cto_download(characteristic.address, address_extension, data.freeze())
            .await?;
        Ok(())
    }

    async fn cto_short_upload(
        &self,
        address: u32,
        address_extension: u32,
        length: u32,
    ) -> Result<(dipstick_proto::wkt::Timestamp, Bytes)> {
        let req_data = CtoShortUploadReqData {
            address,
            address_extension,
            length,
        };
        let resp = self.cto_command(&req_data.into()).await?;
        let resp_data = match resp.cto_resp_data {
            Some(CtoRespData::ShortUpload(data)) => data,
            Some(CtoRespData::Error(data)) => {
                return Err(Status::internal(format!(
                    "server rejected short upload: {}",
                    data.error().as_str_name()
                )))
            }
            _ => unreachable!(),
        };
        Ok((resp.timestamp.unwrap_or_default(), resp_data.data))
    }

    async fn cto_download(&self, address: u32, address_extension: u32, data: Bytes) -> Result<()> {
        // TODO: need some kind of overarching lock
        self.cto_set_mta(address, address_extension).await?;
        let req_data = CtoDownloadReqData {
            data,
            block_mode_length: 0,
            last: true, // TODO: smart
        };
        let resp = self.cto_command(&req_data.into()).await?;
        match resp.cto_resp_data {
            Some(CtoRespData::Download(_data)) => Ok(()),
            Some(CtoRespData::Error(data)) => Err(Status::internal(format!(
                "server rejected download: {}",
                data.error().as_str_name()
            ))),
            _ => unreachable!(),
        }
    }

    async fn cto_set_mta(&self, address: u32, address_extension: u32) -> Result<()> {
        let req_data = CtoSetMtaReqData {
            address,
            address_extension,
        };
        let resp = self.cto_command(&req_data.into()).await?;
        match resp.cto_resp_data {
            Some(CtoRespData::SetMta(_data)) => Ok(()),
            Some(CtoRespData::Error(data)) => Err(Status::internal(format!(
                "server rejected set mta: {}",
                data.error().as_str_name()
            ))),
            _ => unreachable!(),
        }
    }

    pub async fn cto_command(&self, req: &CtoReq) -> Result<CtoResp> {
        let _guard = self.cto_guard.lock().await;

        if req.pid() != CtoReqPid::Connect {
            self.auto_connect_raw().await?;
        }

        let byte_order = {
            let connect_data = self.connect_data.read().unwrap();
            connect_data
                .as_ref()
                .and_then(|resp| resp.comm_mode_basic.as_ref())
                .map(|comm_mode_basic| comm_mode_basic.byte_order())
                .unwrap_or(ByteOrder::Unspecified)
        };
        self.cto_command_impl(byte_order, req).await
    }

    async fn auto_connect_raw(&self) -> Result<()> {
        if self.connect_data.read().unwrap().is_some() {
            return Ok(());
        }

        tracing::debug!("auto-connecting");
        let req_data = CtoConnectReqData {
            mode: self.spec.auto_connect_mode(),
        };
        let resp = self
            .cto_command_impl(ByteOrder::Unspecified, &req_data.into())
            .await?;
        let connect_data = match resp.cto_resp_data {
            Some(CtoRespData::Connect(data)) => data,
            Some(CtoRespData::Error(data)) => {
                return Err(Status::internal(format!(
                    "server rejected connect: {}",
                    data.error().as_str_name()
                )))
            }
            _ => unreachable!(),
        };

        let mut slot = self.connect_data.write().unwrap();
        *slot = Some(connect_data);
        Ok(())
    }

    async fn cto_command_impl(&self, byte_order: ByteOrder, req: &CtoReq) -> Result<CtoResp> {
        let mut buf = BytesMut::new();
        let req_pid = req.pid();
        crate::protocol::encoder::cto_req(&mut buf, byte_order, req)?;
        let mut packet_stream = self.transport.subscribe();
        self.transport.send(buf.freeze()).await?;
        let resp_fut = crate::transport::get_response(&mut packet_stream, |mut input| {
            crate::protocol::decoder::cto_resp(&mut input, req_pid)
        });
        match tokio::time::timeout(self.spec.cto_timeout.unwrap().try_into().unwrap(), resp_fut)
            .await
        {
            Ok(res) => res,
            Err(_) => Err(Status::deadline_exceeded("cto command timeout")),
        }
    }
}

impl Entity for Session {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for Session {
    type Package = crate::XcpService;
    const KIND: &'static str = "Session";
}
