// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/shadow/v1/signal.proto" (package "dipstick.shadow.v1", syntax proto3)
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
import { Value } from "../../../google/protobuf/struct";
import { Timestamp } from "../../../google/protobuf/timestamp";
/**
 * @generated from protobuf message dipstick.shadow.v1.SignalStatus
 */
export interface SignalStatus {
  /**
   * @generated from protobuf field: google.protobuf.Timestamp changed_at = 1;
   */
  changedAt?: Timestamp;
  /**
   * @generated from protobuf field: google.protobuf.Value value = 2;
   */
  value?: Value;
  /**
   * @generated from protobuf field: uint32 update_count = 3;
   */
  updateCount: number;
}
/**
 * @generated from protobuf message dipstick.shadow.v1.SignalEvent
 */
export interface SignalEvent {
  /**
   * @generated from protobuf field: string signal = 1;
   */
  signal: string;
  /**
   * @generated from protobuf field: google.protobuf.Timestamp timestamp = 2;
   */
  timestamp?: Timestamp;
  /**
   * @generated from protobuf field: google.protobuf.Value value = 3;
   */
  value?: Value;
}
/**
 * @generated from protobuf message dipstick.shadow.v1.SignalSpec
 */
export interface SignalSpec {
  /**
   * @generated from protobuf oneof: signal_spec_variant
   */
  signalSpecVariant:
    | {
        oneofKind: "a2LMeasurement";
        /**
         * @generated from protobuf field: dipstick.shadow.v1.A2lMeasurementSignalSpec a2l_measurement = 3 [json_name = "a2lMeasurement"];
         */
        a2LMeasurement: A2lMeasurementSignalSpec;
      }
    | {
        oneofKind: "a2LCharacteristic";
        /**
         * @generated from protobuf field: dipstick.shadow.v1.A2lCharacteristicSignalSpec a2l_characteristic = 5 [json_name = "a2lCharacteristic"];
         */
        a2LCharacteristic: A2lCharacteristicSignalSpec;
      }
    | {
        oneofKind: "deviceSensor";
        /**
         * @generated from protobuf field: dipstick.shadow.v1.DeviceSensorSignalSpec device_sensor = 2;
         */
        deviceSensor: DeviceSensorSignalSpec;
      }
    | {
        oneofKind: "gpio";
        /**
         * @generated from protobuf field: dipstick.shadow.v1.GpioSignalSpec gpio = 4;
         */
        gpio: GpioSignalSpec;
      }
    | {
        oneofKind: undefined;
      };
}
/**
 * @generated from protobuf message dipstick.shadow.v1.A2lMeasurementSignalSpec
 */
export interface A2lMeasurementSignalSpec {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector session = 3;
   */
  session?: EntitySelector;
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector a2l = 1 [json_name = "a2l"];
   */
  a2L?: EntitySelector;
  /**
   * @generated from protobuf field: string measurement_name = 2;
   */
  measurementName: string;
  /**
   * @generated from protobuf field: google.protobuf.Duration poll_interval = 4;
   */
  pollInterval?: Duration;
}
/**
 * @generated from protobuf message dipstick.shadow.v1.A2lCharacteristicSignalSpec
 */
export interface A2lCharacteristicSignalSpec {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector session = 1;
   */
  session?: EntitySelector;
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector a2l = 2 [json_name = "a2l"];
   */
  a2L?: EntitySelector;
  /**
   * @generated from protobuf field: string characteristic_name = 3;
   */
  characteristicName: string;
  /**
   * @generated from protobuf field: google.protobuf.Duration poll_interval = 4;
   */
  pollInterval?: Duration;
}
/**
 * @generated from protobuf message dipstick.shadow.v1.DeviceSensorSignalSpec
 */
export interface DeviceSensorSignalSpec {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector device = 1;
   */
  device?: EntitySelector;
  /**
   * @generated from protobuf field: string sensor = 2;
   */
  sensor: string;
}
/**
 * @generated from protobuf message dipstick.shadow.v1.GpioSignalSpec
 */
export interface GpioSignalSpec {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector chip = 1;
   */
  chip?: EntitySelector;
  /**
   * @generated from protobuf field: string pin = 2;
   */
  pin: string;
}
// @generated message type with reflection information, may provide speed optimized methods
class SignalStatus$Type extends MessageType<SignalStatus> {
  constructor() {
    super("dipstick.shadow.v1.SignalStatus", [
      { no: 1, name: "changed_at", kind: "message", T: () => Timestamp },
      { no: 2, name: "value", kind: "message", T: () => Value },
      {
        no: 3,
        name: "update_count",
        kind: "scalar",
        T: 13 /*ScalarType.UINT32*/,
      },
    ]);
  }
  create(value?: PartialMessage<SignalStatus>): SignalStatus {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.updateCount = 0;
    if (value !== undefined)
      reflectionMergePartial<SignalStatus>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: SignalStatus,
  ): SignalStatus {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* google.protobuf.Timestamp changed_at */ 1:
          message.changedAt = Timestamp.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.changedAt,
          );
          break;
        case /* google.protobuf.Value value */ 2:
          message.value = Value.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.value,
          );
          break;
        case /* uint32 update_count */ 3:
          message.updateCount = reader.uint32();
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
    message: SignalStatus,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* google.protobuf.Timestamp changed_at = 1; */
    if (message.changedAt)
      Timestamp.internalBinaryWrite(
        message.changedAt,
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* google.protobuf.Value value = 2; */
    if (message.value)
      Value.internalBinaryWrite(
        message.value,
        writer.tag(2, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* uint32 update_count = 3; */
    if (message.updateCount !== 0)
      writer.tag(3, WireType.Varint).uint32(message.updateCount);
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
 * @generated MessageType for protobuf message dipstick.shadow.v1.SignalStatus
 */
export const SignalStatus = new SignalStatus$Type();
// @generated message type with reflection information, may provide speed optimized methods
class SignalEvent$Type extends MessageType<SignalEvent> {
  constructor() {
    super("dipstick.shadow.v1.SignalEvent", [
      { no: 1, name: "signal", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
      { no: 2, name: "timestamp", kind: "message", T: () => Timestamp },
      { no: 3, name: "value", kind: "message", T: () => Value },
    ]);
  }
  create(value?: PartialMessage<SignalEvent>): SignalEvent {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.signal = "";
    if (value !== undefined)
      reflectionMergePartial<SignalEvent>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: SignalEvent,
  ): SignalEvent {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* string signal */ 1:
          message.signal = reader.string();
          break;
        case /* google.protobuf.Timestamp timestamp */ 2:
          message.timestamp = Timestamp.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.timestamp,
          );
          break;
        case /* google.protobuf.Value value */ 3:
          message.value = Value.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.value,
          );
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
    message: SignalEvent,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* string signal = 1; */
    if (message.signal !== "")
      writer.tag(1, WireType.LengthDelimited).string(message.signal);
    /* google.protobuf.Timestamp timestamp = 2; */
    if (message.timestamp)
      Timestamp.internalBinaryWrite(
        message.timestamp,
        writer.tag(2, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* google.protobuf.Value value = 3; */
    if (message.value)
      Value.internalBinaryWrite(
        message.value,
        writer.tag(3, WireType.LengthDelimited).fork(),
        options,
      ).join();
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
 * @generated MessageType for protobuf message dipstick.shadow.v1.SignalEvent
 */
export const SignalEvent = new SignalEvent$Type();
// @generated message type with reflection information, may provide speed optimized methods
class SignalSpec$Type extends MessageType<SignalSpec> {
  constructor() {
    super("dipstick.shadow.v1.SignalSpec", [
      {
        no: 3,
        name: "a2l_measurement",
        kind: "message",
        jsonName: "a2lMeasurement",
        oneof: "signalSpecVariant",
        T: () => A2lMeasurementSignalSpec,
      },
      {
        no: 5,
        name: "a2l_characteristic",
        kind: "message",
        jsonName: "a2lCharacteristic",
        oneof: "signalSpecVariant",
        T: () => A2lCharacteristicSignalSpec,
      },
      {
        no: 2,
        name: "device_sensor",
        kind: "message",
        oneof: "signalSpecVariant",
        T: () => DeviceSensorSignalSpec,
      },
      {
        no: 4,
        name: "gpio",
        kind: "message",
        oneof: "signalSpecVariant",
        T: () => GpioSignalSpec,
      },
    ]);
  }
  create(value?: PartialMessage<SignalSpec>): SignalSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.signalSpecVariant = { oneofKind: undefined };
    if (value !== undefined)
      reflectionMergePartial<SignalSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: SignalSpec,
  ): SignalSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.shadow.v1.A2lMeasurementSignalSpec a2l_measurement = 3 [json_name = "a2lMeasurement"];*/ 3:
          message.signalSpecVariant = {
            oneofKind: "a2LMeasurement",
            a2LMeasurement: A2lMeasurementSignalSpec.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
              (message.signalSpecVariant as any).a2LMeasurement,
            ),
          };
          break;
        case /* dipstick.shadow.v1.A2lCharacteristicSignalSpec a2l_characteristic = 5 [json_name = "a2lCharacteristic"];*/ 5:
          message.signalSpecVariant = {
            oneofKind: "a2LCharacteristic",
            a2LCharacteristic: A2lCharacteristicSignalSpec.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
              (message.signalSpecVariant as any).a2LCharacteristic,
            ),
          };
          break;
        case /* dipstick.shadow.v1.DeviceSensorSignalSpec device_sensor */ 2:
          message.signalSpecVariant = {
            oneofKind: "deviceSensor",
            deviceSensor: DeviceSensorSignalSpec.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
              (message.signalSpecVariant as any).deviceSensor,
            ),
          };
          break;
        case /* dipstick.shadow.v1.GpioSignalSpec gpio */ 4:
          message.signalSpecVariant = {
            oneofKind: "gpio",
            gpio: GpioSignalSpec.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
              (message.signalSpecVariant as any).gpio,
            ),
          };
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
    message: SignalSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.shadow.v1.A2lMeasurementSignalSpec a2l_measurement = 3 [json_name = "a2lMeasurement"]; */
    if (message.signalSpecVariant.oneofKind === "a2LMeasurement")
      A2lMeasurementSignalSpec.internalBinaryWrite(
        message.signalSpecVariant.a2LMeasurement,
        writer.tag(3, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.shadow.v1.A2lCharacteristicSignalSpec a2l_characteristic = 5 [json_name = "a2lCharacteristic"]; */
    if (message.signalSpecVariant.oneofKind === "a2LCharacteristic")
      A2lCharacteristicSignalSpec.internalBinaryWrite(
        message.signalSpecVariant.a2LCharacteristic,
        writer.tag(5, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.shadow.v1.DeviceSensorSignalSpec device_sensor = 2; */
    if (message.signalSpecVariant.oneofKind === "deviceSensor")
      DeviceSensorSignalSpec.internalBinaryWrite(
        message.signalSpecVariant.deviceSensor,
        writer.tag(2, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.shadow.v1.GpioSignalSpec gpio = 4; */
    if (message.signalSpecVariant.oneofKind === "gpio")
      GpioSignalSpec.internalBinaryWrite(
        message.signalSpecVariant.gpio,
        writer.tag(4, WireType.LengthDelimited).fork(),
        options,
      ).join();
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
 * @generated MessageType for protobuf message dipstick.shadow.v1.SignalSpec
 */
export const SignalSpec = new SignalSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class A2lMeasurementSignalSpec$Type extends MessageType<A2lMeasurementSignalSpec> {
  constructor() {
    super("dipstick.shadow.v1.A2lMeasurementSignalSpec", [
      { no: 3, name: "session", kind: "message", T: () => EntitySelector },
      {
        no: 1,
        name: "a2l",
        kind: "message",
        jsonName: "a2l",
        T: () => EntitySelector,
      },
      {
        no: 2,
        name: "measurement_name",
        kind: "scalar",
        T: 9 /*ScalarType.STRING*/,
      },
      { no: 4, name: "poll_interval", kind: "message", T: () => Duration },
    ]);
  }
  create(
    value?: PartialMessage<A2lMeasurementSignalSpec>,
  ): A2lMeasurementSignalSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.measurementName = "";
    if (value !== undefined)
      reflectionMergePartial<A2lMeasurementSignalSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: A2lMeasurementSignalSpec,
  ): A2lMeasurementSignalSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntitySelector session */ 3:
          message.session = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.session,
          );
          break;
        case /* dipstick.core.v1.EntitySelector a2l = 1 [json_name = "a2l"];*/ 1:
          message.a2L = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.a2L,
          );
          break;
        case /* string measurement_name */ 2:
          message.measurementName = reader.string();
          break;
        case /* google.protobuf.Duration poll_interval */ 4:
          message.pollInterval = Duration.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.pollInterval,
          );
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
    message: A2lMeasurementSignalSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntitySelector session = 3; */
    if (message.session)
      EntitySelector.internalBinaryWrite(
        message.session,
        writer.tag(3, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.core.v1.EntitySelector a2l = 1 [json_name = "a2l"]; */
    if (message.a2L)
      EntitySelector.internalBinaryWrite(
        message.a2L,
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* string measurement_name = 2; */
    if (message.measurementName !== "")
      writer.tag(2, WireType.LengthDelimited).string(message.measurementName);
    /* google.protobuf.Duration poll_interval = 4; */
    if (message.pollInterval)
      Duration.internalBinaryWrite(
        message.pollInterval,
        writer.tag(4, WireType.LengthDelimited).fork(),
        options,
      ).join();
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
 * @generated MessageType for protobuf message dipstick.shadow.v1.A2lMeasurementSignalSpec
 */
export const A2lMeasurementSignalSpec = new A2lMeasurementSignalSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class A2lCharacteristicSignalSpec$Type extends MessageType<A2lCharacteristicSignalSpec> {
  constructor() {
    super("dipstick.shadow.v1.A2lCharacteristicSignalSpec", [
      { no: 1, name: "session", kind: "message", T: () => EntitySelector },
      {
        no: 2,
        name: "a2l",
        kind: "message",
        jsonName: "a2l",
        T: () => EntitySelector,
      },
      {
        no: 3,
        name: "characteristic_name",
        kind: "scalar",
        T: 9 /*ScalarType.STRING*/,
      },
      { no: 4, name: "poll_interval", kind: "message", T: () => Duration },
    ]);
  }
  create(
    value?: PartialMessage<A2lCharacteristicSignalSpec>,
  ): A2lCharacteristicSignalSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.characteristicName = "";
    if (value !== undefined)
      reflectionMergePartial<A2lCharacteristicSignalSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: A2lCharacteristicSignalSpec,
  ): A2lCharacteristicSignalSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntitySelector session */ 1:
          message.session = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.session,
          );
          break;
        case /* dipstick.core.v1.EntitySelector a2l = 2 [json_name = "a2l"];*/ 2:
          message.a2L = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.a2L,
          );
          break;
        case /* string characteristic_name */ 3:
          message.characteristicName = reader.string();
          break;
        case /* google.protobuf.Duration poll_interval */ 4:
          message.pollInterval = Duration.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.pollInterval,
          );
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
    message: A2lCharacteristicSignalSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntitySelector session = 1; */
    if (message.session)
      EntitySelector.internalBinaryWrite(
        message.session,
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.core.v1.EntitySelector a2l = 2 [json_name = "a2l"]; */
    if (message.a2L)
      EntitySelector.internalBinaryWrite(
        message.a2L,
        writer.tag(2, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* string characteristic_name = 3; */
    if (message.characteristicName !== "")
      writer
        .tag(3, WireType.LengthDelimited)
        .string(message.characteristicName);
    /* google.protobuf.Duration poll_interval = 4; */
    if (message.pollInterval)
      Duration.internalBinaryWrite(
        message.pollInterval,
        writer.tag(4, WireType.LengthDelimited).fork(),
        options,
      ).join();
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
 * @generated MessageType for protobuf message dipstick.shadow.v1.A2lCharacteristicSignalSpec
 */
export const A2lCharacteristicSignalSpec =
  new A2lCharacteristicSignalSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class DeviceSensorSignalSpec$Type extends MessageType<DeviceSensorSignalSpec> {
  constructor() {
    super("dipstick.shadow.v1.DeviceSensorSignalSpec", [
      { no: 1, name: "device", kind: "message", T: () => EntitySelector },
      { no: 2, name: "sensor", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
    ]);
  }
  create(
    value?: PartialMessage<DeviceSensorSignalSpec>,
  ): DeviceSensorSignalSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.sensor = "";
    if (value !== undefined)
      reflectionMergePartial<DeviceSensorSignalSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: DeviceSensorSignalSpec,
  ): DeviceSensorSignalSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntitySelector device */ 1:
          message.device = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.device,
          );
          break;
        case /* string sensor */ 2:
          message.sensor = reader.string();
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
    message: DeviceSensorSignalSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntitySelector device = 1; */
    if (message.device)
      EntitySelector.internalBinaryWrite(
        message.device,
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* string sensor = 2; */
    if (message.sensor !== "")
      writer.tag(2, WireType.LengthDelimited).string(message.sensor);
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
 * @generated MessageType for protobuf message dipstick.shadow.v1.DeviceSensorSignalSpec
 */
export const DeviceSensorSignalSpec = new DeviceSensorSignalSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GpioSignalSpec$Type extends MessageType<GpioSignalSpec> {
  constructor() {
    super("dipstick.shadow.v1.GpioSignalSpec", [
      { no: 1, name: "chip", kind: "message", T: () => EntitySelector },
      { no: 2, name: "pin", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
    ]);
  }
  create(value?: PartialMessage<GpioSignalSpec>): GpioSignalSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.pin = "";
    if (value !== undefined)
      reflectionMergePartial<GpioSignalSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: GpioSignalSpec,
  ): GpioSignalSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntitySelector chip */ 1:
          message.chip = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.chip,
          );
          break;
        case /* string pin */ 2:
          message.pin = reader.string();
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
    message: GpioSignalSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntitySelector chip = 1; */
    if (message.chip)
      EntitySelector.internalBinaryWrite(
        message.chip,
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* string pin = 2; */
    if (message.pin !== "")
      writer.tag(2, WireType.LengthDelimited).string(message.pin);
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
 * @generated MessageType for protobuf message dipstick.shadow.v1.GpioSignalSpec
 */
export const GpioSignalSpec = new GpioSignalSpec$Type();
