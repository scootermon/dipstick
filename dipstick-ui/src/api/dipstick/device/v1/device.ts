// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/device/v1/device.proto" (package "dipstick.device.v1", syntax proto3)
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
import { SensorEvent } from "./sensor";
import { ActionEvent } from "./action";
import { Ina2xxSpec } from "./ina2xx";
import { Duration } from "../../../google/protobuf/duration";
import { SensorStatus } from "./sensor";
import { ActionStatus } from "./action";
import { Value } from "../../../google/protobuf/struct";
import { EntityMeta } from "../../core/v1/entity";
/**
 * @generated from protobuf message dipstick.device.v1.DeviceEntity
 */
export interface DeviceEntity {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntityMeta meta = 1;
   */
  meta?: EntityMeta;
  /**
   * @generated from protobuf field: dipstick.device.v1.DeviceSpec spec = 2;
   */
  spec?: DeviceSpec;
  /**
   * @generated from protobuf field: dipstick.device.v1.DeviceStatus status = 3;
   */
  status?: DeviceStatus;
}
/**
 * @generated from protobuf message dipstick.device.v1.DeviceStatus
 */
export interface DeviceStatus {
  /**
   * @generated from protobuf field: map<string, google.protobuf.Value> attrs = 1;
   */
  attrs: {
    [key: string]: Value;
  };
  /**
   * @generated from protobuf field: map<string, dipstick.device.v1.ActionStatus> actions = 2;
   */
  actions: {
    [key: string]: ActionStatus;
  };
  /**
   * @generated from protobuf field: map<string, dipstick.device.v1.SensorStatus> sensors = 3;
   */
  sensors: {
    [key: string]: SensorStatus;
  };
}
/**
 * @generated from protobuf message dipstick.device.v1.DeviceSpec
 */
export interface DeviceSpec {
  /**
   * @generated from protobuf field: google.protobuf.Duration poll_interval = 2;
   */
  pollInterval?: Duration;
  /**
   * @generated from protobuf oneof: device_spec_variant
   */
  deviceSpecVariant:
    | {
        oneofKind: "ina2Xx";
        /**
         * @generated from protobuf field: dipstick.device.v1.Ina2xxSpec ina2xx = 1 [json_name = "ina2xx"];
         */
        ina2Xx: Ina2xxSpec;
      }
    | {
        oneofKind: undefined;
      };
}
/**
 * @generated from protobuf message dipstick.device.v1.DeviceEvent
 */
export interface DeviceEvent {
  /**
   * @generated from protobuf oneof: device_event_variant
   */
  deviceEventVariant:
    | {
        oneofKind: "action";
        /**
         * @generated from protobuf field: dipstick.device.v1.ActionEvent action = 1;
         */
        action: ActionEvent;
      }
    | {
        oneofKind: "sensor";
        /**
         * @generated from protobuf field: dipstick.device.v1.SensorEvent sensor = 2;
         */
        sensor: SensorEvent;
      }
    | {
        oneofKind: undefined;
      };
}
// @generated message type with reflection information, may provide speed optimized methods
class DeviceEntity$Type extends MessageType<DeviceEntity> {
  constructor() {
    super("dipstick.device.v1.DeviceEntity", [
      { no: 1, name: "meta", kind: "message", T: () => EntityMeta },
      { no: 2, name: "spec", kind: "message", T: () => DeviceSpec },
      { no: 3, name: "status", kind: "message", T: () => DeviceStatus },
    ]);
  }
  create(value?: PartialMessage<DeviceEntity>): DeviceEntity {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<DeviceEntity>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: DeviceEntity,
  ): DeviceEntity {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntityMeta meta */ 1:
          message.meta = EntityMeta.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.meta,
          );
          break;
        case /* dipstick.device.v1.DeviceSpec spec */ 2:
          message.spec = DeviceSpec.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.spec,
          );
          break;
        case /* dipstick.device.v1.DeviceStatus status */ 3:
          message.status = DeviceStatus.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.status,
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
    message: DeviceEntity,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntityMeta meta = 1; */
    if (message.meta)
      EntityMeta.internalBinaryWrite(
        message.meta,
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.device.v1.DeviceSpec spec = 2; */
    if (message.spec)
      DeviceSpec.internalBinaryWrite(
        message.spec,
        writer.tag(2, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.device.v1.DeviceStatus status = 3; */
    if (message.status)
      DeviceStatus.internalBinaryWrite(
        message.status,
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
 * @generated MessageType for protobuf message dipstick.device.v1.DeviceEntity
 */
export const DeviceEntity = new DeviceEntity$Type();
// @generated message type with reflection information, may provide speed optimized methods
class DeviceStatus$Type extends MessageType<DeviceStatus> {
  constructor() {
    super("dipstick.device.v1.DeviceStatus", [
      {
        no: 1,
        name: "attrs",
        kind: "map",
        K: 9 /*ScalarType.STRING*/,
        V: { kind: "message", T: () => Value },
      },
      {
        no: 2,
        name: "actions",
        kind: "map",
        K: 9 /*ScalarType.STRING*/,
        V: { kind: "message", T: () => ActionStatus },
      },
      {
        no: 3,
        name: "sensors",
        kind: "map",
        K: 9 /*ScalarType.STRING*/,
        V: { kind: "message", T: () => SensorStatus },
      },
    ]);
  }
  create(value?: PartialMessage<DeviceStatus>): DeviceStatus {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.attrs = {};
    message.actions = {};
    message.sensors = {};
    if (value !== undefined)
      reflectionMergePartial<DeviceStatus>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: DeviceStatus,
  ): DeviceStatus {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* map<string, google.protobuf.Value> attrs */ 1:
          this.binaryReadMap1(message.attrs, reader, options);
          break;
        case /* map<string, dipstick.device.v1.ActionStatus> actions */ 2:
          this.binaryReadMap2(message.actions, reader, options);
          break;
        case /* map<string, dipstick.device.v1.SensorStatus> sensors */ 3:
          this.binaryReadMap3(message.sensors, reader, options);
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
  private binaryReadMap1(
    map: DeviceStatus["attrs"],
    reader: IBinaryReader,
    options: BinaryReadOptions,
  ): void {
    let len = reader.uint32(),
      end = reader.pos + len,
      key: keyof DeviceStatus["attrs"] | undefined,
      val: DeviceStatus["attrs"][any] | undefined;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case 1:
          key = reader.string();
          break;
        case 2:
          val = Value.internalBinaryRead(reader, reader.uint32(), options);
          break;
        default:
          throw new globalThis.Error(
            "unknown map entry field for field dipstick.device.v1.DeviceStatus.attrs",
          );
      }
    }
    map[key ?? ""] = val ?? Value.create();
  }
  private binaryReadMap2(
    map: DeviceStatus["actions"],
    reader: IBinaryReader,
    options: BinaryReadOptions,
  ): void {
    let len = reader.uint32(),
      end = reader.pos + len,
      key: keyof DeviceStatus["actions"] | undefined,
      val: DeviceStatus["actions"][any] | undefined;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case 1:
          key = reader.string();
          break;
        case 2:
          val = ActionStatus.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
          );
          break;
        default:
          throw new globalThis.Error(
            "unknown map entry field for field dipstick.device.v1.DeviceStatus.actions",
          );
      }
    }
    map[key ?? ""] = val ?? ActionStatus.create();
  }
  private binaryReadMap3(
    map: DeviceStatus["sensors"],
    reader: IBinaryReader,
    options: BinaryReadOptions,
  ): void {
    let len = reader.uint32(),
      end = reader.pos + len,
      key: keyof DeviceStatus["sensors"] | undefined,
      val: DeviceStatus["sensors"][any] | undefined;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case 1:
          key = reader.string();
          break;
        case 2:
          val = SensorStatus.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
          );
          break;
        default:
          throw new globalThis.Error(
            "unknown map entry field for field dipstick.device.v1.DeviceStatus.sensors",
          );
      }
    }
    map[key ?? ""] = val ?? SensorStatus.create();
  }
  internalBinaryWrite(
    message: DeviceStatus,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* map<string, google.protobuf.Value> attrs = 1; */
    for (let k of globalThis.Object.keys(message.attrs)) {
      writer
        .tag(1, WireType.LengthDelimited)
        .fork()
        .tag(1, WireType.LengthDelimited)
        .string(k);
      writer.tag(2, WireType.LengthDelimited).fork();
      Value.internalBinaryWrite(message.attrs[k], writer, options);
      writer.join().join();
    }
    /* map<string, dipstick.device.v1.ActionStatus> actions = 2; */
    for (let k of globalThis.Object.keys(message.actions)) {
      writer
        .tag(2, WireType.LengthDelimited)
        .fork()
        .tag(1, WireType.LengthDelimited)
        .string(k);
      writer.tag(2, WireType.LengthDelimited).fork();
      ActionStatus.internalBinaryWrite(message.actions[k], writer, options);
      writer.join().join();
    }
    /* map<string, dipstick.device.v1.SensorStatus> sensors = 3; */
    for (let k of globalThis.Object.keys(message.sensors)) {
      writer
        .tag(3, WireType.LengthDelimited)
        .fork()
        .tag(1, WireType.LengthDelimited)
        .string(k);
      writer.tag(2, WireType.LengthDelimited).fork();
      SensorStatus.internalBinaryWrite(message.sensors[k], writer, options);
      writer.join().join();
    }
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
 * @generated MessageType for protobuf message dipstick.device.v1.DeviceStatus
 */
export const DeviceStatus = new DeviceStatus$Type();
// @generated message type with reflection information, may provide speed optimized methods
class DeviceSpec$Type extends MessageType<DeviceSpec> {
  constructor() {
    super("dipstick.device.v1.DeviceSpec", [
      { no: 2, name: "poll_interval", kind: "message", T: () => Duration },
      {
        no: 1,
        name: "ina2xx",
        kind: "message",
        jsonName: "ina2xx",
        oneof: "deviceSpecVariant",
        T: () => Ina2xxSpec,
      },
    ]);
  }
  create(value?: PartialMessage<DeviceSpec>): DeviceSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.deviceSpecVariant = { oneofKind: undefined };
    if (value !== undefined)
      reflectionMergePartial<DeviceSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: DeviceSpec,
  ): DeviceSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* google.protobuf.Duration poll_interval */ 2:
          message.pollInterval = Duration.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.pollInterval,
          );
          break;
        case /* dipstick.device.v1.Ina2xxSpec ina2xx = 1 [json_name = "ina2xx"];*/ 1:
          message.deviceSpecVariant = {
            oneofKind: "ina2Xx",
            ina2Xx: Ina2xxSpec.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
              (message.deviceSpecVariant as any).ina2Xx,
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
    message: DeviceSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* google.protobuf.Duration poll_interval = 2; */
    if (message.pollInterval)
      Duration.internalBinaryWrite(
        message.pollInterval,
        writer.tag(2, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.device.v1.Ina2xxSpec ina2xx = 1 [json_name = "ina2xx"]; */
    if (message.deviceSpecVariant.oneofKind === "ina2Xx")
      Ina2xxSpec.internalBinaryWrite(
        message.deviceSpecVariant.ina2Xx,
        writer.tag(1, WireType.LengthDelimited).fork(),
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
 * @generated MessageType for protobuf message dipstick.device.v1.DeviceSpec
 */
export const DeviceSpec = new DeviceSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class DeviceEvent$Type extends MessageType<DeviceEvent> {
  constructor() {
    super("dipstick.device.v1.DeviceEvent", [
      {
        no: 1,
        name: "action",
        kind: "message",
        oneof: "deviceEventVariant",
        T: () => ActionEvent,
      },
      {
        no: 2,
        name: "sensor",
        kind: "message",
        oneof: "deviceEventVariant",
        T: () => SensorEvent,
      },
    ]);
  }
  create(value?: PartialMessage<DeviceEvent>): DeviceEvent {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.deviceEventVariant = { oneofKind: undefined };
    if (value !== undefined)
      reflectionMergePartial<DeviceEvent>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: DeviceEvent,
  ): DeviceEvent {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.device.v1.ActionEvent action */ 1:
          message.deviceEventVariant = {
            oneofKind: "action",
            action: ActionEvent.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
              (message.deviceEventVariant as any).action,
            ),
          };
          break;
        case /* dipstick.device.v1.SensorEvent sensor */ 2:
          message.deviceEventVariant = {
            oneofKind: "sensor",
            sensor: SensorEvent.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
              (message.deviceEventVariant as any).sensor,
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
    message: DeviceEvent,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.device.v1.ActionEvent action = 1; */
    if (message.deviceEventVariant.oneofKind === "action")
      ActionEvent.internalBinaryWrite(
        message.deviceEventVariant.action,
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.device.v1.SensorEvent sensor = 2; */
    if (message.deviceEventVariant.oneofKind === "sensor")
      SensorEvent.internalBinaryWrite(
        message.deviceEventVariant.sensor,
        writer.tag(2, WireType.LengthDelimited).fork(),
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
 * @generated MessageType for protobuf message dipstick.device.v1.DeviceEvent
 */
export const DeviceEvent = new DeviceEvent$Type();