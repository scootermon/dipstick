use dipstick_proto::xcp::v1::{
    A2lAddrType, A2lByteOrder, A2lCharacteristic, A2lCompuMethod, A2lCompuTab, A2lCompuVtab,
    A2lConversionType, A2lDataType, A2lFile, A2lFncValues, A2lMeasurement, A2lModCommon, A2lModule,
    A2lProject, A2lRecordLayout, A2lValuePairsStruct,
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
        name: module.name,
        long_identifier: module.long_identifier,
        characteristic: module
            .characteristic
            .into_iter()
            .map(map_characteristic)
            .collect(),
        compu_method: module
            .compu_method
            .into_iter()
            .map(map_compu_method)
            .collect(),
        compu_tab: module.compu_tab.into_iter().map(map_compu_tab).collect(),
        compu_vtab: module.compu_vtab.into_iter().map(map_compu_vtab).collect(),
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
        conversion: map_conversion(c.conversion),
        byte_order: map_byte_order(c.byte_order.map(|order| order.byte_order)) as _,
        ecu_address_extension: c.ecu_address_extension.map(|ext| i32::from(ext.extension)),
    }
}

fn map_compu_method(method: a2lfile::CompuMethod) -> A2lCompuMethod {
    A2lCompuMethod {
        name: method.name,
        long_identifier: method.long_identifier,
        conversion_type: map_conversion_type(method.conversion_type) as _,
        format: method.format,
        unit: method.unit,
        compu_tab_ref: method.compu_tab_ref.map(|v| v.conversion_table),
        ref_unit: method.ref_unit.map(|v| v.unit),
        status_string_ref: method.status_string_ref.map(|v| v.conversion_table),
    }
}

fn map_compu_tab(tab: a2lfile::CompuTab) -> A2lCompuTab {
    A2lCompuTab {
        name: tab.name,
        long_identifier: tab.long_identifier,
        conversion_type: map_conversion_type(tab.conversion_type) as _,
    }
}

fn map_compu_vtab(vtab: a2lfile::CompuVtab) -> A2lCompuVtab {
    A2lCompuVtab {
        name: vtab.name,
        long_identifier: vtab.long_identifier,
        conversion_type: map_conversion_type(vtab.conversion_type) as _,
        value_pairs: vtab
            .value_pairs
            .into_iter()
            .map(map_value_pairs_struct)
            .collect(),
        default_value: vtab.default_value.map(|v| v.display_string),
    }
}

fn map_value_pairs_struct(pair: a2lfile::ValuePairsStruct) -> A2lValuePairsStruct {
    A2lValuePairsStruct {
        in_val: pair.in_val,
        out_val: pair.out_val,
    }
}

fn map_conversion_type(conversion_type: a2lfile::ConversionType) -> A2lConversionType {
    match conversion_type {
        a2lfile::ConversionType::Identical => A2lConversionType::Identical,
        a2lfile::ConversionType::Form => A2lConversionType::Form,
        a2lfile::ConversionType::Linear => A2lConversionType::Linear,
        a2lfile::ConversionType::RatFunc => A2lConversionType::RatFunc,
        a2lfile::ConversionType::TabIntp => A2lConversionType::TabIntp,
        a2lfile::ConversionType::TabNointp => A2lConversionType::TabNointp,
        a2lfile::ConversionType::TabVerb => A2lConversionType::TabVerb,
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
        conversion: map_conversion(meas.conversion),
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

fn map_conversion(c: String) -> Option<String> {
    if c == "NO_COMPU_METHOD" {
        None
    } else {
        Some(c)
    }
}
