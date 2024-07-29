// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/can/v1/id.proto" (package "dipstick.can.v1", syntax proto3)
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
/**
 * @generated from protobuf message dipstick.can.v1.Id
 */
export interface Id {
  /**
   * @generated from protobuf field: uint32 id = 1;
   */
  id: number;
  /**
   * @generated from protobuf field: bool extended = 2;
   */
  extended: boolean;
}
// @generated message type with reflection information, may provide speed optimized methods
class Id$Type extends MessageType<Id> {
  constructor() {
    super("dipstick.can.v1.Id", [
      { no: 1, name: "id", kind: "scalar", T: 13 /*ScalarType.UINT32*/ },
      { no: 2, name: "extended", kind: "scalar", T: 8 /*ScalarType.BOOL*/ },
    ]);
  }
  create(value?: PartialMessage<Id>): Id {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.id = 0;
    message.extended = false;
    if (value !== undefined) reflectionMergePartial<Id>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: Id,
  ): Id {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* uint32 id */ 1:
          message.id = reader.uint32();
          break;
        case /* bool extended */ 2:
          message.extended = reader.bool();
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
    message: Id,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* uint32 id = 1; */
    if (message.id !== 0) writer.tag(1, WireType.Varint).uint32(message.id);
    /* bool extended = 2; */
    if (message.extended !== false)
      writer.tag(2, WireType.Varint).bool(message.extended);
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
 * @generated MessageType for protobuf message dipstick.can.v1.Id
 */
export const Id = new Id$Type();
