use bytes::{Buf, BufMut, Bytes, BytesMut};
use dipstick_proto::xcp::v1::{A2lByteOrder, A2lDataType, A2lRecordLayout};

pub const fn data_type_len(data_type: A2lDataType) -> u16 {
    use A2lDataType as T;
    match data_type {
        T::Unspecified => 0,
        T::Ubyte | T::Sbyte => 1,
        T::Uword | T::Sword | T::F16Ieee => 2,
        T::Ulong | T::Slong | T::F32Ieee => 4,
        T::AUint64 | T::AInt64 | T::F64Ieee => 8,
    }
}

pub fn encode_data_type(
    buf: &mut BytesMut,
    data_type: A2lDataType,
    byte_order: A2lByteOrder,
    value: &dipstick_proto::wkt::Value,
) -> anyhow::Result<()> {
    use {A2lByteOrder as E, A2lDataType as T};

    let f_value = match value.kind {
        Some(dipstick_proto::wkt::value::Kind::BoolValue(true)) => 1.0,
        Some(dipstick_proto::wkt::value::Kind::BoolValue(false)) => 0.0,
        Some(dipstick_proto::wkt::value::Kind::NumberValue(v)) => v,
        _ => anyhow::bail!("invalid value"),
    };

    // collapse byte order.
    // TODO: This doesn't really make sense to me but the BMS reports MSB_LAST even
    //       though it's little endian and I didn't find a way to confirm it one way
    //       or the other.
    let byte_order = match byte_order {
        E::MsbFirst => E::BigEndian,
        E::MsbLast => E::LittleEndian,
        _ => byte_order,
    };
    match (data_type, byte_order) {
        // 8
        (T::Ubyte, E::BigEndian | E::LittleEndian) => buf.put_u8(f_value as _),
        (T::Sbyte, E::BigEndian | E::LittleEndian) => buf.put_i8(f_value as _),
        // 16
        (T::Uword, E::BigEndian) => buf.put_u16(f_value as _),
        (T::Uword, E::LittleEndian) => buf.put_u16_le(f_value as _),
        (T::Sword, E::BigEndian) => buf.put_i16(f_value as _),
        (T::Sword, E::LittleEndian) => buf.put_i16_le(f_value as _),
        // 32
        (T::Ulong, E::BigEndian) => buf.put_u32(f_value as _),
        (T::Ulong, E::LittleEndian) => buf.put_u32_le(f_value as _),
        (T::Slong, E::BigEndian) => buf.put_i32(f_value as _),
        (T::Slong, E::LittleEndian) => buf.put_i32_le(f_value as _),
        // 64
        (T::AUint64, E::BigEndian) => buf.put_u64(f_value as _),
        (T::AUint64, E::LittleEndian) => buf.put_u64_le(f_value as _),
        (T::AInt64, E::BigEndian) => buf.put_i64(f_value as _),
        (T::AInt64, E::LittleEndian) => buf.put_i64_le(f_value as _),
        // f32
        (T::F32Ieee, E::BigEndian) => buf.put_f32(f_value as _),
        (T::F32Ieee, E::LittleEndian) => buf.put_f32_le(f_value as _),
        // f64
        (T::F64Ieee, E::BigEndian) => buf.put_f64(f_value),
        (T::F64Ieee, E::LittleEndian) => buf.put_f64_le(f_value),
        _ => anyhow::bail!("unsupported data type / byte order combination"),
    }
    Ok(())
}

pub fn decode_data_type(
    input: &mut Bytes,
    data_type: A2lDataType,
    byte_order: A2lByteOrder,
) -> anyhow::Result<dipstick_proto::wkt::Value> {
    use {A2lByteOrder as E, A2lDataType as T};

    let required_len = usize::from(data_type_len(data_type));
    anyhow::ensure!(input.remaining() >= required_len, "not enough data");

    // collapse byte order.
    // TODO: This doesn't really make sense to me but the BMS reports MSB_LAST even
    //       though it's little endian and I didn't find a way to confirm it one way
    //       or the other.
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

pub fn record_len(record_layout: &A2lRecordLayout) -> u16 {
    record_layout
        .fnc_values
        .as_ref()
        .map(|fnc| data_type_len(fnc.datatype()))
        .unwrap_or_default()
}

pub fn encode_record(
    buf: &mut BytesMut,
    record_layout: &A2lRecordLayout,
    byte_order: A2lByteOrder,
    value: &dipstick_proto::wkt::Value,
) -> anyhow::Result<()> {
    let data_type = record_layout
        .fnc_values
        .as_ref()
        .ok_or_else(|| anyhow::format_err!("record layout is missing fnc values"))?
        .datatype();
    encode_data_type(buf, data_type, byte_order, value)
}

pub fn decode_record(
    input: &mut Bytes,
    record_layout: &A2lRecordLayout,
    byte_order: A2lByteOrder,
) -> anyhow::Result<dipstick_proto::wkt::Value> {
    let data_type = record_layout
        .fnc_values
        .as_ref()
        .ok_or_else(|| anyhow::format_err!("record layout is missing fnc values"))?
        .datatype();
    decode_data_type(input, data_type, byte_order)
}
