// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/gpio/v1/chip.proto" (package "dipstick.gpio.v1", syntax proto3)
// tslint:disable
import { EntityMeta } from "../../core/v1/entity";
import { PinStatus } from "./pin";
import { PinSpec } from "./pin";
import type { BinaryWriteOptions } from "@protobuf-ts/runtime";
import type { IBinaryWriter } from "@protobuf-ts/runtime";
import { WireType } from "@protobuf-ts/runtime";
import type { BinaryReadOptions } from "@protobuf-ts/runtime";
import type { IBinaryReader } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import type { PartialMessage } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";

/**
 * @generated from protobuf message dipstick.gpio.v1.ChipEntity
 */
export interface ChipEntity {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntityMeta meta = 1;
   */
  meta?: EntityMeta;
  /**
   * @generated from protobuf field: dipstick.gpio.v1.ChipSpec spec = 2;
   */
  spec?: ChipSpec;
  /**
   * @generated from protobuf field: dipstick.gpio.v1.ChipStatus status = 3;
   */
  status?: ChipStatus;
}
/**
 * @generated from protobuf message dipstick.gpio.v1.ChipSpec
 */
export interface ChipSpec {
  /**
   * @generated from protobuf field: map<string, dipstick.gpio.v1.PinSpec> pins = 2;
   */
  pins: {
    [key: string]: PinSpec;
  };
  /**
   * @generated from protobuf oneof: chip_spec_variant
   */
  chipSpecVariant:
    | {
        oneofKind: "linux";
        /**
         * @generated from protobuf field: dipstick.gpio.v1.LinuxChipSpec linux = 3;
         */
        linux: LinuxChipSpec;
      }
    | {
        oneofKind: undefined;
      };
}
/**
 * @generated from protobuf message dipstick.gpio.v1.LinuxChipSpec
 */
export interface LinuxChipSpec {
  /**
   * @generated from protobuf field: string name = 1;
   */
  name: string;
}
/**
 * @generated from protobuf message dipstick.gpio.v1.ChipStatus
 */
export interface ChipStatus {
  /**
   * @generated from protobuf field: map<string, dipstick.gpio.v1.PinStatus> pins = 2;
   */
  pins: {
    [key: string]: PinStatus;
  };
}
// @generated message type with reflection information, may provide speed optimized methods
class ChipEntity$Type extends MessageType<ChipEntity> {
  constructor() {
    super("dipstick.gpio.v1.ChipEntity", [
      { no: 1, name: "meta", kind: "message", T: () => EntityMeta },
      { no: 2, name: "spec", kind: "message", T: () => ChipSpec },
      { no: 3, name: "status", kind: "message", T: () => ChipStatus },
    ]);
  }
  create(value?: PartialMessage<ChipEntity>): ChipEntity {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<ChipEntity>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: ChipEntity,
  ): ChipEntity {
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
        case /* dipstick.gpio.v1.ChipSpec spec */ 2:
          message.spec = ChipSpec.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.spec,
          );
          break;
        case /* dipstick.gpio.v1.ChipStatus status */ 3:
          message.status = ChipStatus.internalBinaryRead(
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
    message: ChipEntity,
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
    /* dipstick.gpio.v1.ChipSpec spec = 2; */
    if (message.spec)
      ChipSpec.internalBinaryWrite(
        message.spec,
        writer.tag(2, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.gpio.v1.ChipStatus status = 3; */
    if (message.status)
      ChipStatus.internalBinaryWrite(
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
 * @generated MessageType for protobuf message dipstick.gpio.v1.ChipEntity
 */
export const ChipEntity = new ChipEntity$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ChipSpec$Type extends MessageType<ChipSpec> {
  constructor() {
    super("dipstick.gpio.v1.ChipSpec", [
      {
        no: 2,
        name: "pins",
        kind: "map",
        K: 9 /*ScalarType.STRING*/,
        V: { kind: "message", T: () => PinSpec },
      },
      {
        no: 3,
        name: "linux",
        kind: "message",
        oneof: "chipSpecVariant",
        T: () => LinuxChipSpec,
      },
    ]);
  }
  create(value?: PartialMessage<ChipSpec>): ChipSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.pins = {};
    message.chipSpecVariant = { oneofKind: undefined };
    if (value !== undefined)
      reflectionMergePartial<ChipSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: ChipSpec,
  ): ChipSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* map<string, dipstick.gpio.v1.PinSpec> pins */ 2:
          this.binaryReadMap2(message.pins, reader, options);
          break;
        case /* dipstick.gpio.v1.LinuxChipSpec linux */ 3:
          message.chipSpecVariant = {
            oneofKind: "linux",
            linux: LinuxChipSpec.internalBinaryRead(
              reader,
              reader.uint32(),
              options,
              (message.chipSpecVariant as any).linux,
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
  private binaryReadMap2(
    map: ChipSpec["pins"],
    reader: IBinaryReader,
    options: BinaryReadOptions,
  ): void {
    let len = reader.uint32(),
      end = reader.pos + len,
      key: keyof ChipSpec["pins"] | undefined,
      val: ChipSpec["pins"][any] | undefined;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case 1:
          key = reader.string();
          break;
        case 2:
          val = PinSpec.internalBinaryRead(reader, reader.uint32(), options);
          break;
        default:
          throw new globalThis.Error(
            "unknown map entry field for field dipstick.gpio.v1.ChipSpec.pins",
          );
      }
    }
    map[key ?? ""] = val ?? PinSpec.create();
  }
  internalBinaryWrite(
    message: ChipSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* map<string, dipstick.gpio.v1.PinSpec> pins = 2; */
    for (let k of globalThis.Object.keys(message.pins)) {
      writer
        .tag(2, WireType.LengthDelimited)
        .fork()
        .tag(1, WireType.LengthDelimited)
        .string(k);
      writer.tag(2, WireType.LengthDelimited).fork();
      PinSpec.internalBinaryWrite(message.pins[k], writer, options);
      writer.join().join();
    }
    /* dipstick.gpio.v1.LinuxChipSpec linux = 3; */
    if (message.chipSpecVariant.oneofKind === "linux")
      LinuxChipSpec.internalBinaryWrite(
        message.chipSpecVariant.linux,
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
 * @generated MessageType for protobuf message dipstick.gpio.v1.ChipSpec
 */
export const ChipSpec = new ChipSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LinuxChipSpec$Type extends MessageType<LinuxChipSpec> {
  constructor() {
    super("dipstick.gpio.v1.LinuxChipSpec", [
      { no: 1, name: "name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
    ]);
  }
  create(value?: PartialMessage<LinuxChipSpec>): LinuxChipSpec {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.name = "";
    if (value !== undefined)
      reflectionMergePartial<LinuxChipSpec>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: LinuxChipSpec,
  ): LinuxChipSpec {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* string name */ 1:
          message.name = reader.string();
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
    message: LinuxChipSpec,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* string name = 1; */
    if (message.name !== "")
      writer.tag(1, WireType.LengthDelimited).string(message.name);
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
 * @generated MessageType for protobuf message dipstick.gpio.v1.LinuxChipSpec
 */
export const LinuxChipSpec = new LinuxChipSpec$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ChipStatus$Type extends MessageType<ChipStatus> {
  constructor() {
    super("dipstick.gpio.v1.ChipStatus", [
      {
        no: 2,
        name: "pins",
        kind: "map",
        K: 9 /*ScalarType.STRING*/,
        V: { kind: "message", T: () => PinStatus },
      },
    ]);
  }
  create(value?: PartialMessage<ChipStatus>): ChipStatus {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.pins = {};
    if (value !== undefined)
      reflectionMergePartial<ChipStatus>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: ChipStatus,
  ): ChipStatus {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* map<string, dipstick.gpio.v1.PinStatus> pins */ 2:
          this.binaryReadMap2(message.pins, reader, options);
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
  private binaryReadMap2(
    map: ChipStatus["pins"],
    reader: IBinaryReader,
    options: BinaryReadOptions,
  ): void {
    let len = reader.uint32(),
      end = reader.pos + len,
      key: keyof ChipStatus["pins"] | undefined,
      val: ChipStatus["pins"][any] | undefined;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case 1:
          key = reader.string();
          break;
        case 2:
          val = PinStatus.internalBinaryRead(reader, reader.uint32(), options);
          break;
        default:
          throw new globalThis.Error(
            "unknown map entry field for field dipstick.gpio.v1.ChipStatus.pins",
          );
      }
    }
    map[key ?? ""] = val ?? PinStatus.create();
  }
  internalBinaryWrite(
    message: ChipStatus,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* map<string, dipstick.gpio.v1.PinStatus> pins = 2; */
    for (let k of globalThis.Object.keys(message.pins)) {
      writer
        .tag(2, WireType.LengthDelimited)
        .fork()
        .tag(1, WireType.LengthDelimited)
        .string(k);
      writer.tag(2, WireType.LengthDelimited).fork();
      PinStatus.internalBinaryWrite(message.pins[k], writer, options);
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
 * @generated MessageType for protobuf message dipstick.gpio.v1.ChipStatus
 */
export const ChipStatus = new ChipStatus$Type();
