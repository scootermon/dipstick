use dipstick_proto::xcp::v1::{
    A2lAddrType, A2lByteOrder, A2lCharacteristic, A2lDataType, A2lFile, A2lFncValues,
    A2lMeasurement, A2lModCommon, A2lModule, A2lProject, A2lRecordLayout,
};

pub fn map_file(a2l: a2lfile::A2lFile) -> A2lFile {
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
