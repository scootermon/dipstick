// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/device/v1/ina2xx.proto" (package "dipstick.device.v1", syntax proto3)
// tslint:disable
import type { BinaryWriteOptions } from "@protobuf-ts/runtime";
import type { IBinaryWriter } from "@protobuf-ts/runtime";
import { WireType } from "@protobuf-ts/runtime";
import type { BinaryReadOptions } from "@protobuf-ts/runtime";
import type { IBinaryReader } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import type { PartialMessage } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";
import { Duration } from "../../../google/protobuf/duration";
import { EntitySelector } from "../../core/v1/entity";
/**
 * @generated from protobuf message dipstick.device.v1.Ina2xxSpec
 */
export interface Ina2xxSpec {
  /**
   * The value of the shunt resistor used for current calculation. No default value.
   *
   * @generated from protobuf field: optional float shunt_resistance = 2;
   */
  shuntResistance?: number;
  /**
   * The maximum current you are expecting. Component will use it to calibrate the sensor. No default value.
   *
   * @generated from protobuf field: optional float max_current = 3;
   */
  maxCurrent?: number;
  /**
   * @generated from protobuf oneof: ina2xx_transport_spec
   */
  ina2XxTransportSpec:
    | {
        oneofKind: "spiDevice";
        /**
         * @generated from protobuf field: dipstick.core.v1.EntitySelector spi_device = 5;
         */
        spiDevice: EntitySelector;
      }
    | {
        oneofKind: undefined;
      };
  /**
   * Selects the range for differential input across shunt resistor. 0 for ±163.84 mV, 1 for ±40.96 mV range. Defaults to 0.
   *
   * @generated from protobuf field: optional uint32 adc_range = 4;
   */
  adcRange?: number;
  /**
   * Defaults to 4120 us. Valid values are 50 us, 84 us, 150 us, 280 us, 540 us, 1052 us, 2074 us, 4120 us.
   *
   * @generated from protobuf field: google.protobuf.Duration bus_voltage_conversion_time = 6;
   */
  busVoltageConversionTime?: Duration;
  /**
   * Defaults to 4120 us. Valid values are 50 us, 84 us, 150 us, 280 us, 540 us, 1052 us, 2074 us, 4120 us.
   *
   * @generated from protobuf field: google.protobuf.Duration shunt_voltage_conversion_time = 7;
   */
  shuntVoltageConversionTime?: Duration;
  /**
   * Defaults to 4120 us. Valid values are 50 us, 84 us, 150 us, 280 us, 540 us, 1052 us, 2074 us, 4120 us.
   *
   * @generated from protobuf field: google.protobuf.Duration temperature_conversion_time = 8;
   */
  temperatureConversionTime?: Duration;
  /**
   * Selects ADC sample averaging count. Defaults to 128. Valid values are 1, 4, 16, 64, 128, 256, 512, 1024.
   *
   * @generated from protobuf field: optional uint32 adc_averaging = 9;
   */
  adcAveraging?: number;
  /**
   * Temperature coefficient (ppm/°C) of the shunt for temperature compensation correction. Only applicable to INA228 and INA229 devices. Zero value means no compensation is done. Defaults to 0.
   *
   * @generated from protobuf field: optional uint32 temperature_coefficient = 10;
   */
  temperatureCoefficient?: number;
}
// @generated message type with reflection information, may provide speed optimized methods
class Ina2xxSpec$Type extends MessageType<Ina2xxSpec> {
  constructor() {
    super("dipstick.device.v1.Ina2xxSpec", [
      {
        no: 2,
        name: "shunt_resistance",
        kind: "scalar",
        opt: true,
        T: 2 /*ScalarType.FLOAT*/,
      },
      {
        no: 3,
        name: "max_current",
        kind: "scalar",
        opt: true,
        T: 2 /*ScalarType.FLOAT*/,
      },
      {
        no: 5,
        name: "spi_device",
        kind: "message",
        oneof: "ina2XxTransportSpec",
        T: () => EntitySelector,
      },
      {
        no: 4,
        name: "adc_range",
        kind: "scalar",
        opt: true,
        T: 13 /*ScalarType.UINT32*/,
      },
      {
        no: 6,
        name: "bus_voltage_conversion_time",
        kind: "message",
        T: () => Duration,
      },
      {
        no: 7,
        name: "shunt_voltage_conversion_time",
        kind: "message",
        T: () => Duration,
      },
      {
        no: 8,
        name: "temperature_conversion_time",
        kind: "message",
        T: () => Duration,
      },
      {
        no: 9,
        name: "adc_averaging",
        kind: "scalar",
        opt: true,
        T: 13 /*ScalarType.UINT32*/,
      },
      {
        no: 10,
        name: "temperature_coefficient",
        kind: "scalar",
        opt: true,
        T: 13 /*ScalarType.UINT32*/,
      },
    ]);
  }
  create(value?: PartialMessage<Ina2xxSpec>): Ina2xxSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.ina2XxTransportSpec = { oneofKind: undefined };
    if (value !== undefined)
      reflectionMergePartial<Ina2xxSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: Ina2xxSpec,
  ): Ina2xxSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* optional float shunt_resistance */ 2:
          message.shuntResistance = reader.float();
          break;
        case /* optional float max_current */ 3:
          message.maxCurrent = reader.float();
          break;
        case /* dipstick.core.v1.EntitySelector spi_device */ 5:
          message.ina2XxTransportSpec = {
            oneofKind: "spiDevice",
            spiDevice: EntitySelector.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
              (message.ina2XxTransportSpec as any).spiDevice,
            ),
          };
          break;
        case /* optional uint32 adc_range */ 4:
          message.adcRange = reader.uint32();
          break;
        case /* google.protobuf.Duration bus_voltage_conversion_time */ 6:
          message.busVoltageConversionTime = Duration.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.busVoltageConversionTime,
          );
          break;
        case /* google.protobuf.Duration shunt_voltage_conversion_time */ 7:
          message.shuntVoltageConversionTime = Duration.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.shuntVoltageConversionTime,
          );
          break;
        case /* google.protobuf.Duration temperature_conversion_time */ 8:
          message.temperatureConversionTime = Duration.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.temperatureConversionTime,
          );
          break;
        case /* optional uint32 adc_averaging */ 9:
          message.adcAveraging = reader.uint32();
          break;
        case /* optional uint32 temperature_coefficient */ 10:
          message.temperatureCoefficient = reader.uint32();
          break;
        default:
          let u = options.readUnknownField;
          if (u === "throw")
            throw new globalThis.Error(
              `Unknown field ${fieldNo} (wire type ${wireType}) for ${this.typeName}`,
            );
          let d = reader.skip(wireType);
          if (u !== false)
            (u === true ? UnknownFieldHandler.onRead : u)(
              this.typeName,
              message,
              fieldNo,
              wireType,
              d,
            );
      }
    }
    return message;
  }
  internalBinaryWrite(
    message: Ina2xxSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* optional float shunt_resistance = 2; */
    if (message.shuntResistance !== undefined)
      writer.tag(2, WireType.Bit32).float(message.shuntResistance);
    /* optional float max_current = 3; */
    if (message.maxCurrent !== undefined)
      writer.tag(3, WireType.Bit32).float(message.maxCurrent);
    /* dipstick.core.v1.EntitySelector spi_device = 5; */
    if (message.ina2XxTransportSpec.oneofKind === "spiDevice")
      EntitySelector.internalBinaryWrite(
        message.ina2XxTransportSpec.spiDevice,
        writer.tag(5, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* optional uint32 adc_range = 4; */
    if (message.adcRange !== undefined)
      writer.tag(4, WireType.Varint).uint32(message.adcRange);
    /* google.protobuf.Duration bus_voltage_conversion_time = 6; */
    if (message.busVoltageConversionTime)
      Duration.internalBinaryWrite(
        message.busVoltageConversionTime,
        writer.tag(6, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* google.protobuf.Duration shunt_voltage_conversion_time = 7; */
    if (message.shuntVoltageConversionTime)
      Duration.internalBinaryWrite(
        message.shuntVoltageConversionTime,
        writer.tag(7, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* google.protobuf.Duration temperature_conversion_time = 8; */
    if (message.temperatureConversionTime)
      Duration.internalBinaryWrite(
        message.temperatureConversionTime,
        writer.tag(8, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* optional uint32 adc_averaging = 9; */
    if (message.adcAveraging !== undefined)
      writer.tag(9, WireType.Varint).uint32(message.adcAveraging);
    /* optional uint32 temperature_coefficient = 10; */
    if (message.temperatureCoefficient !== undefined)
      writer.tag(10, WireType.Varint).uint32(message.temperatureCoefficient);
    let u = options.writeUnknownFields;
    if (u !== false)
      (u == true ? UnknownFieldHandler.onWrite : u)(
        this.typeName,
        message,
        writer,
      );
    return writer;
  }
}
/**
 * @generated MessageType for protobuf message dipstick.device.v1.Ina2xxSpec
 */
export const Ina2xxSpec = new Ina2xxSpec$Type();