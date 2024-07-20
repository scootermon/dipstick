use std::sync::{Arc, Mutex};

use bytes::{Buf, Bytes};
use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
use dipstick_proto::xcp::v1::{
    A2lAddrType, A2lByteOrder, A2lCharacteristic, A2lDataType, A2lEntity, A2lFile, A2lFncValues,
    A2lMeasurement, A2lModCommon, A2lModule, A2lProject, A2lRecordLayout, A2lSpec, A2lStatus,
};
use tonic::{Result, Status};

pub struct A2l {
    meta: EntityMeta,
    spec: Mutex<A2lSpec>,
    storage: Mutex<Storage>,
}

impl A2l {
    pub async fn new(core: &Core, meta: EntityMeta, spec: A2lSpec) -> Result<Arc<Self>> {
        let this = Arc::new(Self {
            meta,
            spec: Mutex::new(A2lSpec::default()),
            storage: Mutex::new(Storage::new()),
        });
        Arc::clone(&this).update_spec(core, spec).await?;
        Ok(this)
    }

    async fn update_spec(&self, core: &Core, new_spec: A2lSpec) -> Result<()> {
        let file = new_spec.file.clone().unwrap_or_default();
        let content = core.read_file_to_string(&file).await?;
        let mut log_msgs = Vec::new();
        let a2l_file = a2lfile::load_from_string(
            &content,
            new_spec.a2ml_spec.clone(),
            &mut log_msgs,
            new_spec.strict_parsing(),
        )
        .map(map_file)
        .map_err(|err| Status::invalid_argument(format!("{err}")))?;

        {
            let mut spec = self.spec.lock().unwrap();
            let mut storage = self.storage.lock().unwrap();
            *spec = new_spec;
            storage.a2l_file = a2l_file;
            storage.log_msgs = log_msgs;
        }
        Ok(())
    }

    pub fn spec(&self) -> A2lSpec {
        self.spec.lock().unwrap().clone()
    }

    pub fn status(&self) -> A2lStatus {
        self.storage.lock().unwrap().to_proto()
    }

    pub fn to_proto(&self) -> A2lEntity {
        A2lEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }

    pub fn get_measurement(&self, measurement_name: &str) -> Option<A2lMeasurement> {
        let storage = self.storage.lock().unwrap();
        for project in &storage.a2l_file.project {
            for module in &project.module {
                for meas in &module.measurement {
                    if meas.name == measurement_name {
                        let mut meas = meas.clone();
                        if meas.byte_order() == A2lByteOrder::Unspecified {
                            meas.set_byte_order(
                                module
                                    .mod_common
                                    .as_ref()
                                    .map_or(A2lByteOrder::Unspecified, |common| {
                                        common.byte_order()
                                    }),
                            );
                        }
                        return Some(meas);
                    }
                }
            }
        }
        None
    }
}

impl Entity for A2l {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for A2l {
    const PACKAGE: &'static str = crate::PACKAGE;
    const KIND: &'static str = "A2l";
}

pub fn decode_a2l_data_type(
    input: &mut Bytes,
    data_type: A2lDataType,
    byte_order: A2lByteOrder,
) -> anyhow::Result<dipstick_proto::wkt::Value> {
    use {A2lByteOrder as E, A2lDataType as T};

    let required_bytes = match data_type {
        T::Unspecified => 0,
        T::Ubyte | T::Sbyte => 1,
        T::Uword | T::Sword | T::F16Ieee => 2,
        T::Ulong | T::Slong | T::F32Ieee => 4,
        T::AUint64 | T::AInt64 | T::F64Ieee => 8,
    };
    anyhow::ensure!(input.remaining() >= required_bytes, "not enough bytes");

    // collapse byte order.
    // TODO: This doesn't really make sense to me but the BMS reports MSB_LAST even
    // though it's little endian and I didn't find a way to confirm it one way or
    // the other.
    let byte_order = match byte_order {
        E::MsbFirst => E::BigEndian,
        E::MsbLast => E::LittleEndian,
        _ => byte_order,
    };

    let value = match (data_type, byte_order) {
        // 8
        (T::Ubyte, E::BigEndian | E::LittleEndian) => input.get_u8() as f64,
        (T::Sbyte, E::BigEndian | E::LittleEndian) => input.get_i8() as f64,
        // 16
        (T::Uword, E::BigEndian) => input.get_u16() as f64,
        (T::Uword, E::LittleEndian) => input.get_u16_le() as f64,
        (T::Sword, E::BigEndian) => input.get_i16() as f64,
        (T::Sword, E::LittleEndian) => input.get_i16_le() as f64,
        // 32
        (T::Ulong, E::BigEndian) => input.get_u32() as f64,
        (T::Ulong, E::LittleEndian) => input.get_u32_le() as f64,
        (T::Slong, E::BigEndian) => input.get_i32() as f64,
        (T::Slong, E::LittleEndian) => input.get_i32_le() as f64,
        // 64
        (T::AUint64, E::BigEndian) => input.get_u64() as f64,
        (T::AUint64, E::LittleEndian) => input.get_u64_le() as f64,
        (T::AInt64, E::BigEndian) => input.get_i64() as f64,
        (T::AInt64, E::LittleEndian) => input.get_i64_le() as f64,
        // f32
        (T::F32Ieee, E::BigEndian) => input.get_f32() as f64,
        (T::F32Ieee, E::LittleEndian) => input.get_f32_le() as f64,
        // f64
        (T::F64Ieee, E::BigEndian) => input.get_f64(),
        (T::F64Ieee, E::LittleEndian) => input.get_f64_le(),
        _ => anyhow::bail!("unsupported data type / byte order combination"),
    };
    Ok(dipstick_proto::wkt::Value {
        kind: Some(dipstick_proto::wkt::value::Kind::NumberValue(value)),
    })
}

struct Storage {
    a2l_file: A2lFile,
    log_msgs: Vec<a2lfile::A2lError>,
}

impl Storage {
    fn new() -> Self {
        Self {
            a2l_file: A2lFile::default(),
            log_msgs: Vec::new(),
        }
    }

    fn to_proto(&self) -> A2lStatus {
        A2lStatus {
            file: Some(self.a2l_file.clone()),
            warnings: self.log_msgs.iter().map(|err| err.to_string()).collect(),
        }
    }
}

fn map_file(a2l: a2lfile::A2lFile) -> A2lFile {
    let module = a2l.project.module.into_iter().map(map_module).collect();
    let project = A2lProject { module };
    A2lFile {
        project: Some(project),
    }
}

fn map_module(module: a2lfile::Module) -> A2lModule {
    A2lModule {
        characteristic: module
            .characteristic
            .into_iter()
            .map(map_characteristic)
            .collect(),
        measurement: module
            .measurement
            .into_iter()
            .map(map_measurement)
            .collect(),
        mod_common: module.mod_common.map(map_mod_common),
        record_layout: module
            .record_layout
            .into_iter()
            .map(map_record_layout)
            .collect(),
    }
}

fn map_mod_common(mod_common: a2lfile::ModCommon) -> A2lModCommon {
    A2lModCommon {
        comment: mod_common.comment,
        byte_order: map_byte_order(mod_common.byte_order.map(|order| order.byte_order)) as _,
    }
}

fn map_characteristic(c: a2lfile::Characteristic) -> A2lCharacteristic {
    A2lCharacteristic {
        name: c.name,
        long_identifier: c.long_identifier,
        address: c.address,
        deposit: c.deposit,
        conversion: c.conversion,
        byte_order: map_byte_order(c.byte_order.map(|order| order.byte_order)) as _,
        ecu_address_extension: c.ecu_address_extension.map(|ext| i32::from(ext.extension)),
    }
}

fn map_byte_order(byte_order: Option<a2lfile::ByteOrderEnum>) -> A2lByteOrder {
    match byte_order {
        Some(a2lfile::ByteOrderEnum::LittleEndian) => A2lByteOrder::LittleEndian,
        Some(a2lfile::ByteOrderEnum::BigEndian) => A2lByteOrder::BigEndian,
        Some(a2lfile::ByteOrderEnum::MsbLast) => A2lByteOrder::MsbLast,
        Some(a2lfile::ByteOrderEnum::MsbFirst) => A2lByteOrder::MsbFirst,
        Some(a2lfile::ByteOrderEnum::MsbFirstMswLast) => A2lByteOrder::MsbFirstMswLast,
        Some(a2lfile::ByteOrderEnum::MsbLastMswFirst) => A2lByteOrder::MsbLastMswFirst,
        None => A2lByteOrder::Unspecified,
    }
}

fn map_measurement(meas: a2lfile::Measurement) -> A2lMeasurement {
    A2lMeasurement {
        name: meas.name,
        long_identifier: meas.long_identifier,
        datatype: map_data_type(meas.datatype) as _,
        conversion: meas.conversion,
        byte_order: map_byte_order(meas.byte_order.map(|order| order.byte_order)) as _,
        ecu_address: meas.ecu_address.map(|addr| addr.address),
        ecu_address_extension: meas
            .ecu_address_extension
            .map(|ext| i32::from(ext.extension)),
    }
}

fn map_data_type(datatype: a2lfile::DataType) -> A2lDataType {
    match datatype {
        a2lfile::DataType::Ubyte => A2lDataType::Ubyte,
        a2lfile::DataType::Sbyte => A2lDataType::Sbyte,
        a2lfile::DataType::Uword => A2lDataType::Uword,
        a2lfile::DataType::Sword => A2lDataType::Sword,
        a2lfile::DataType::Ulong => A2lDataType::Ulong,
        a2lfile::DataType::Slong => A2lDataType::Slong,
        a2lfile::DataType::AUint64 => A2lDataType::AUint64,
        a2lfile::DataType::AInt64 => A2lDataType::AInt64,
        a2lfile::DataType::Float16Ieee => A2lDataType::F16Ieee,
        a2lfile::DataType::Float32Ieee => A2lDataType::F32Ieee,
        a2lfile::DataType::Float64Ieee => A2lDataType::F64Ieee,
    }
}

fn map_record_layout(record_layout: a2lfile::RecordLayout) -> A2lRecordLayout {
    A2lRecordLayout {
        name: record_layout.name,
        fnc_values: record_layout.fnc_values.map(map_fnc_values),
    }
}

fn map_fnc_values(fnc_values: a2lfile::FncValues) -> A2lFncValues {
    A2lFncValues {
        position: u32::from(fnc_values.position),
        datatype: map_data_type(fnc_values.datatype) as _,
        address_type: map_addr_type(fnc_values.address_type) as _,
    }
}

fn map_addr_type(addr_type: a2lfile::AddrType) -> A2lAddrType {
    match addr_type {
        a2lfile::AddrType::Pbyte => A2lAddrType::Pbyte,
        a2lfile::AddrType::Pword => A2lAddrType::Pword,
        a2lfile::AddrType::Plong => A2lAddrType::Plong,
        a2lfile::AddrType::Plonglong => A2lAddrType::Plonglong,
        a2lfile::AddrType::Direct => A2lAddrType::Direct,
    }
}
