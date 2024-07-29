// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/stack/v1/stack.proto" (package "dipstick.stack.v1", syntax proto3)
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
import { CreateSessionRequest } from "../../xcp/v1/service";
import { CreateA2lRequest } from "../../xcp/v1/service";
import { CreateDeviceRequest as CreateDeviceRequest$ } from "../../spi/v1/service";
import { CreateShadowRequest } from "../../shadow/v1/service";
import { CreateChipRequest } from "../../gpio/v1/service";
import { CreateDeviceRequest } from "../../device/v1/service";
import { CreateBusRequest } from "../../can/v1/service";
import { EntityMeta } from "../../core/v1/entity";
/**
 * @generated from protobuf message dipstick.stack.v1.StackEntity
 */
export interface StackEntity {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntityMeta meta = 1;
   */
  meta?: EntityMeta;
  /**
   * @generated from protobuf field: dipstick.stack.v1.StackSpec spec = 2;
   */
  spec?: StackSpec;
  /**
   * @generated from protobuf field: dipstick.stack.v1.StackStatus status = 3;
   */
  status?: StackStatus;
}
/**
 * @generated from protobuf message dipstick.stack.v1.StackStatus
 */
export interface StackStatus {}
/**
 * @generated from protobuf message dipstick.stack.v1.StackSpec
 */
export interface StackSpec {
  /**
   * @generated from protobuf field: dipstick.stack.v1.CanPackageSpec can = 8;
   */
  can?: CanPackageSpec;
  /**
   * @generated from protobuf field: dipstick.stack.v1.DevicePackageSpec device = 6;
   */
  device?: DevicePackageSpec;
  /**
   * @generated from protobuf field: dipstick.stack.v1.GpioPackageSpec gpio = 4;
   */
  gpio?: GpioPackageSpec;
  /**
   * @generated from protobuf field: dipstick.stack.v1.ShadowPackageSpec shadow = 7;
   */
  shadow?: ShadowPackageSpec;
  /**
   * @generated from protobuf field: dipstick.stack.v1.SpiPackageSpec spi = 3;
   */
  spi?: SpiPackageSpec;
  /**
   * @generated from protobuf field: dipstick.stack.v1.XcpPackageSpec xcp = 5;
   */
  xcp?: XcpPackageSpec;
}
/**
 * @generated from protobuf message dipstick.stack.v1.CanPackageSpec
 */
export interface CanPackageSpec {
  /**
   * @generated from protobuf field: repeated dipstick.can.v1.CreateBusRequest bus = 1;
   */
  bus: CreateBusRequest[];
}
/**
 * @generated from protobuf message dipstick.stack.v1.DevicePackageSpec
 */
export interface DevicePackageSpec {
  /**
   * @generated from protobuf field: repeated dipstick.device.v1.CreateDeviceRequest device = 1;
   */
  device: CreateDeviceRequest[];
}
/**
 * @generated from protobuf message dipstick.stack.v1.GpioPackageSpec
 */
export interface GpioPackageSpec {
  /**
   * @generated from protobuf field: repeated dipstick.gpio.v1.CreateChipRequest chip = 1;
   */
  chip: CreateChipRequest[];
}
/**
 * @generated from protobuf message dipstick.stack.v1.ShadowPackageSpec
 */
export interface ShadowPackageSpec {
  /**
   * @generated from protobuf field: repeated dipstick.shadow.v1.CreateShadowRequest shadow = 2;
   */
  shadow: CreateShadowRequest[];
}
/**
 * @generated from protobuf message dipstick.stack.v1.SpiPackageSpec
 */
export interface SpiPackageSpec {
  /**
   * @generated from protobuf field: repeated dipstick.spi.v1.CreateDeviceRequest device = 1;
   */
  device: CreateDeviceRequest$[];
}
/**
 * @generated from protobuf message dipstick.stack.v1.XcpPackageSpec
 */
export interface XcpPackageSpec {
  /**
   * @generated from protobuf field: repeated dipstick.xcp.v1.CreateA2lRequest a2l = 1 [json_name = "a2l"];
   */
  a2L: CreateA2lRequest[];
  /**
   * @generated from protobuf field: repeated dipstick.xcp.v1.CreateSessionRequest session = 2;
   */
  session: CreateSessionRequest[];
}
// @generated message type with reflection information, may provide speed optimized methods
class StackEntity$Type extends MessageType<StackEntity> {
  constructor() {
    super("dipstick.stack.v1.StackEntity", [
      { no: 1, name: "meta", kind: "message", T: () => EntityMeta },
      { no: 2, name: "spec", kind: "message", T: () => StackSpec },
      { no: 3, name: "status", kind: "message", T: () => StackStatus },
    ]);
  }
  create(value?: PartialMessage<StackEntity>): StackEntity {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<StackEntity>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: StackEntity,
  ): StackEntity {
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
        case /* dipstick.stack.v1.StackSpec spec */ 2:
          message.spec = StackSpec.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.spec,
          );
          break;
        case /* dipstick.stack.v1.StackStatus status */ 3:
          message.status = StackStatus.internalBinaryRead(
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
    message: StackEntity,
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
    /* dipstick.stack.v1.StackSpec spec = 2; */
    if (message.spec)
      StackSpec.internalBinaryWrite(
        message.spec,
        writer.tag(2, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.stack.v1.StackStatus status = 3; */
    if (message.status)
      StackStatus.internalBinaryWrite(
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
 * @generated MessageType for protobuf message dipstick.stack.v1.StackEntity
 */
export const StackEntity = new StackEntity$Type();
// @generated message type with reflection information, may provide speed optimized methods
class StackStatus$Type extends MessageType<StackStatus> {
  constructor() {
    super("dipstick.stack.v1.StackStatus", []);
  }
  create(value?: PartialMessage<StackStatus>): StackStatus {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<StackStatus>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: StackStatus,
  ): StackStatus {
    return target ?? this.create();
  }
  internalBinaryWrite(
    message: StackStatus,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
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
 * @generated MessageType for protobuf message dipstick.stack.v1.StackStatus
 */
export const StackStatus = new StackStatus$Type();
// @generated message type with reflection information, may provide speed optimized methods
class StackSpec$Type extends MessageType<StackSpec> {
  constructor() {
    super("dipstick.stack.v1.StackSpec", [
      { no: 8, name: "can", kind: "message", T: () => CanPackageSpec },
      { no: 6, name: "device", kind: "message", T: () => DevicePackageSpec },
      { no: 4, name: "gpio", kind: "message", T: () => GpioPackageSpec },
      { no: 7, name: "shadow", kind: "message", T: () => ShadowPackageSpec },
      { no: 3, name: "spi", kind: "message", T: () => SpiPackageSpec },
      { no: 5, name: "xcp", kind: "message", T: () => XcpPackageSpec },
    ]);
  }
  create(value?: PartialMessage<StackSpec>): StackSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<StackSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: StackSpec,
  ): StackSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.stack.v1.CanPackageSpec can */ 8:
          message.can = CanPackageSpec.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.can,
          );
          break;
        case /* dipstick.stack.v1.DevicePackageSpec device */ 6:
          message.device = DevicePackageSpec.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.device,
          );
          break;
        case /* dipstick.stack.v1.GpioPackageSpec gpio */ 4:
          message.gpio = GpioPackageSpec.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.gpio,
          );
          break;
        case /* dipstick.stack.v1.ShadowPackageSpec shadow */ 7:
          message.shadow = ShadowPackageSpec.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.shadow,
          );
          break;
        case /* dipstick.stack.v1.SpiPackageSpec spi */ 3:
          message.spi = SpiPackageSpec.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.spi,
          );
          break;
        case /* dipstick.stack.v1.XcpPackageSpec xcp */ 5:
          message.xcp = XcpPackageSpec.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.xcp,
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
    message: StackSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.stack.v1.CanPackageSpec can = 8; */
    if (message.can)
      CanPackageSpec.internalBinaryWrite(
        message.can,
        writer.tag(8, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.stack.v1.DevicePackageSpec device = 6; */
    if (message.device)
      DevicePackageSpec.internalBinaryWrite(
        message.device,
        writer.tag(6, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.stack.v1.GpioPackageSpec gpio = 4; */
    if (message.gpio)
      GpioPackageSpec.internalBinaryWrite(
        message.gpio,
        writer.tag(4, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.stack.v1.ShadowPackageSpec shadow = 7; */
    if (message.shadow)
      ShadowPackageSpec.internalBinaryWrite(
        message.shadow,
        writer.tag(7, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.stack.v1.SpiPackageSpec spi = 3; */
    if (message.spi)
      SpiPackageSpec.internalBinaryWrite(
        message.spi,
        writer.tag(3, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.stack.v1.XcpPackageSpec xcp = 5; */
    if (message.xcp)
      XcpPackageSpec.internalBinaryWrite(
        message.xcp,
        writer.tag(5, WireType.LengthDelimited).fork(),
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
 * @generated MessageType for protobuf message dipstick.stack.v1.StackSpec
 */
export const StackSpec = new StackSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class CanPackageSpec$Type extends MessageType<CanPackageSpec> {
  constructor() {
    super("dipstick.stack.v1.CanPackageSpec", [
      {
        no: 1,
        name: "bus",
        kind: "message",
        repeat: 1 /*RepeatType.PACKED*/,
        T: () => CreateBusRequest,
      },
    ]);
  }
  create(value?: PartialMessage<CanPackageSpec>): CanPackageSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.bus = [];
    if (value !== undefined)
      reflectionMergePartial<CanPackageSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: CanPackageSpec,
  ): CanPackageSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* repeated dipstick.can.v1.CreateBusRequest bus */ 1:
          message.bus.push(
            CreateBusRequest.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
            ),
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
    message: CanPackageSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* repeated dipstick.can.v1.CreateBusRequest bus = 1; */
    for (let i = 0; i < message.bus.length; i++)
      CreateBusRequest.internalBinaryWrite(
        message.bus[i],
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
 * @generated MessageType for protobuf message dipstick.stack.v1.CanPackageSpec
 */
export const CanPackageSpec = new CanPackageSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class DevicePackageSpec$Type extends MessageType<DevicePackageSpec> {
  constructor() {
    super("dipstick.stack.v1.DevicePackageSpec", [
      {
        no: 1,
        name: "device",
        kind: "message",
        repeat: 1 /*RepeatType.PACKED*/,
        T: () => CreateDeviceRequest,
      },
    ]);
  }
  create(value?: PartialMessage<DevicePackageSpec>): DevicePackageSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.device = [];
    if (value !== undefined)
      reflectionMergePartial<DevicePackageSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: DevicePackageSpec,
  ): DevicePackageSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* repeated dipstick.device.v1.CreateDeviceRequest device */ 1:
          message.device.push(
            CreateDeviceRequest.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
            ),
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
    message: DevicePackageSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* repeated dipstick.device.v1.CreateDeviceRequest device = 1; */
    for (let i = 0; i < message.device.length; i++)
      CreateDeviceRequest.internalBinaryWrite(
        message.device[i],
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
 * @generated MessageType for protobuf message dipstick.stack.v1.DevicePackageSpec
 */
export const DevicePackageSpec = new DevicePackageSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GpioPackageSpec$Type extends MessageType<GpioPackageSpec> {
  constructor() {
    super("dipstick.stack.v1.GpioPackageSpec", [
      {
        no: 1,
        name: "chip",
        kind: "message",
        repeat: 1 /*RepeatType.PACKED*/,
        T: () => CreateChipRequest,
      },
    ]);
  }
  create(value?: PartialMessage<GpioPackageSpec>): GpioPackageSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.chip = [];
    if (value !== undefined)
      reflectionMergePartial<GpioPackageSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: GpioPackageSpec,
  ): GpioPackageSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* repeated dipstick.gpio.v1.CreateChipRequest chip */ 1:
          message.chip.push(
            CreateChipRequest.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
            ),
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
    message: GpioPackageSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* repeated dipstick.gpio.v1.CreateChipRequest chip = 1; */
    for (let i = 0; i < message.chip.length; i++)
      CreateChipRequest.internalBinaryWrite(
        message.chip[i],
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
 * @generated MessageType for protobuf message dipstick.stack.v1.GpioPackageSpec
 */
export const GpioPackageSpec = new GpioPackageSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ShadowPackageSpec$Type extends MessageType<ShadowPackageSpec> {
  constructor() {
    super("dipstick.stack.v1.ShadowPackageSpec", [
      {
        no: 2,
        name: "shadow",
        kind: "message",
        repeat: 1 /*RepeatType.PACKED*/,
        T: () => CreateShadowRequest,
      },
    ]);
  }
  create(value?: PartialMessage<ShadowPackageSpec>): ShadowPackageSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.shadow = [];
    if (value !== undefined)
      reflectionMergePartial<ShadowPackageSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: ShadowPackageSpec,
  ): ShadowPackageSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* repeated dipstick.shadow.v1.CreateShadowRequest shadow */ 2:
          message.shadow.push(
            CreateShadowRequest.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
            ),
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
    message: ShadowPackageSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* repeated dipstick.shadow.v1.CreateShadowRequest shadow = 2; */
    for (let i = 0; i < message.shadow.length; i++)
      CreateShadowRequest.internalBinaryWrite(
        message.shadow[i],
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
 * @generated MessageType for protobuf message dipstick.stack.v1.ShadowPackageSpec
 */
export const ShadowPackageSpec = new ShadowPackageSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class SpiPackageSpec$Type extends MessageType<SpiPackageSpec> {
  constructor() {
    super("dipstick.stack.v1.SpiPackageSpec", [
      {
        no: 1,
        name: "device",
        kind: "message",
        repeat: 1 /*RepeatType.PACKED*/,
        T: () => CreateDeviceRequest$,
      },
    ]);
  }
  create(value?: PartialMessage<SpiPackageSpec>): SpiPackageSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.device = [];
    if (value !== undefined)
      reflectionMergePartial<SpiPackageSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: SpiPackageSpec,
  ): SpiPackageSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* repeated dipstick.spi.v1.CreateDeviceRequest device */ 1:
          message.device.push(
            CreateDeviceRequest$.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
            ),
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
    message: SpiPackageSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* repeated dipstick.spi.v1.CreateDeviceRequest device = 1; */
    for (let i = 0; i < message.device.length; i++)
      CreateDeviceRequest$.internalBinaryWrite(
        message.device[i],
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
 * @generated MessageType for protobuf message dipstick.stack.v1.SpiPackageSpec
 */
export const SpiPackageSpec = new SpiPackageSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class XcpPackageSpec$Type extends MessageType<XcpPackageSpec> {
  constructor() {
    super("dipstick.stack.v1.XcpPackageSpec", [
      {
        no: 1,
        name: "a2l",
        kind: "message",
        jsonName: "a2l",
        repeat: 1 /*RepeatType.PACKED*/,
        T: () => CreateA2lRequest,
      },
      {
        no: 2,
        name: "session",
        kind: "message",
        repeat: 1 /*RepeatType.PACKED*/,
        T: () => CreateSessionRequest,
      },
    ]);
  }
  create(value?: PartialMessage<XcpPackageSpec>): XcpPackageSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.a2L = [];
    message.session = [];
    if (value !== undefined)
      reflectionMergePartial<XcpPackageSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: XcpPackageSpec,
  ): XcpPackageSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* repeated dipstick.xcp.v1.CreateA2lRequest a2l = 1 [json_name = "a2l"];*/ 1:
          message.a2L.push(
            CreateA2lRequest.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
            ),
          );
          break;
        case /* repeated dipstick.xcp.v1.CreateSessionRequest session */ 2:
          message.session.push(
            CreateSessionRequest.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
            ),
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
    message: XcpPackageSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* repeated dipstick.xcp.v1.CreateA2lRequest a2l = 1 [json_name = "a2l"]; */
    for (let i = 0; i < message.a2L.length; i++)
      CreateA2lRequest.internalBinaryWrite(
        message.a2L[i],
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* repeated dipstick.xcp.v1.CreateSessionRequest session = 2; */
    for (let i = 0; i < message.session.length; i++)
      CreateSessionRequest.internalBinaryWrite(
        message.session[i],
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
 * @generated MessageType for protobuf message dipstick.stack.v1.XcpPackageSpec
 */
export const XcpPackageSpec = new XcpPackageSpec$Type();
