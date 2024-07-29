// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/device/v1/action.proto" (package "dipstick.device.v1", syntax proto3)
// tslint:disable
import { Timestamp } from "../../../google/protobuf/timestamp";
import { WireType } from "@protobuf-ts/runtime";
import type { BinaryWriteOptions } from "@protobuf-ts/runtime";
import type { IBinaryWriter } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import type { BinaryReadOptions } from "@protobuf-ts/runtime";
import type { IBinaryReader } from "@protobuf-ts/runtime";
import type { PartialMessage } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";

/**
 * @generated from protobuf message dipstick.device.v1.ActionStatus
 */
export interface ActionStatus {}
/**
 * @generated from protobuf message dipstick.device.v1.ActionEvent
 */
export interface ActionEvent {
  /**
   * @generated from protobuf field: google.protobuf.Timestamp timestamp = 1;
   */
  timestamp?: Timestamp;
  /**
   * @generated from protobuf field: string action = 2;
   */
  action: string;
}
// @generated message type with reflection information, may provide speed optimized methods
class ActionStatus$Type extends MessageType<ActionStatus> {
  constructor() {
    super("dipstick.device.v1.ActionStatus", []);
  }
  create(value?: PartialMessage<ActionStatus>): ActionStatus {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<ActionStatus>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: ActionStatus,
  ): ActionStatus {
    return target ?? this.create();
  }
  internalBinaryWrite(
    message: ActionStatus,
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
 * @generated MessageType for protobuf message dipstick.device.v1.ActionStatus
 */
export const ActionStatus = new ActionStatus$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ActionEvent$Type extends MessageType<ActionEvent> {
  constructor() {
    super("dipstick.device.v1.ActionEvent", [
      { no: 1, name: "timestamp", kind: "message", T: () => Timestamp },
      { no: 2, name: "action", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
    ]);
  }
  create(value?: PartialMessage<ActionEvent>): ActionEvent {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.action = "";
    if (value !== undefined)
      reflectionMergePartial<ActionEvent>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: ActionEvent,
  ): ActionEvent {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* google.protobuf.Timestamp timestamp */ 1:
          message.timestamp = Timestamp.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.timestamp,
          );
          break;
        case /* string action */ 2:
          message.action = reader.string();
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
    message: ActionEvent,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* google.protobuf.Timestamp timestamp = 1; */
    if (message.timestamp)
      Timestamp.internalBinaryWrite(
        message.timestamp,
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* string action = 2; */
    if (message.action !== "")
      writer.tag(2, WireType.LengthDelimited).string(message.action);
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
 * @generated MessageType for protobuf message dipstick.device.v1.ActionEvent
 */
export const ActionEvent = new ActionEvent$Type();
