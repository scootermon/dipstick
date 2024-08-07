// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/stack/v1/service.proto" (package "dipstick.stack.v1", syntax proto3)
// tslint:disable
import { EntitySelector } from "../../core/v1/entity";
import { EntityMetaSpec } from "../../core/v1/entity";
import { StackEntity } from "./stack";
import { StackSpec } from "./stack";
import type { BinaryWriteOptions } from "@protobuf-ts/runtime";
import type { IBinaryWriter } from "@protobuf-ts/runtime";
import { WireType } from "@protobuf-ts/runtime";
import type { BinaryReadOptions } from "@protobuf-ts/runtime";
import type { IBinaryReader } from "@protobuf-ts/runtime";
import { UnknownFieldHandler } from "@protobuf-ts/runtime";
import type { PartialMessage } from "@protobuf-ts/runtime";
import { reflectionMergePartial } from "@protobuf-ts/runtime";
import { MessageType } from "@protobuf-ts/runtime";
import { ServiceType } from "@protobuf-ts/runtime-rpc";

/**
 * @generated from protobuf message dipstick.stack.v1.CreateStackRequest
 */
export interface CreateStackRequest {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntityMetaSpec meta = 1;
   */
  meta?: EntityMetaSpec;
  /**
   * @generated from protobuf field: dipstick.stack.v1.StackSpec spec = 2;
   */
  spec?: StackSpec;
}
/**
 * @generated from protobuf message dipstick.stack.v1.CreateStackResponse
 */
export interface CreateStackResponse {
  /**
   * @generated from protobuf field: dipstick.stack.v1.StackEntity stack = 1;
   */
  stack?: StackEntity;
}
/**
 * @generated from protobuf message dipstick.stack.v1.GetStackRequest
 */
export interface GetStackRequest {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector selector = 1;
   */
  selector?: EntitySelector;
}
/**
 * @generated from protobuf message dipstick.stack.v1.GetStackResponse
 */
export interface GetStackResponse {
  /**
   * @generated from protobuf field: dipstick.stack.v1.StackEntity stack = 1;
   */
  stack?: StackEntity;
}
// @generated message type with reflection information, may provide speed optimized methods
class CreateStackRequest$Type extends MessageType<CreateStackRequest> {
  constructor() {
    super("dipstick.stack.v1.CreateStackRequest", [
      { no: 1, name: "meta", kind: "message", T: () => EntityMetaSpec },
      { no: 2, name: "spec", kind: "message", T: () => StackSpec },
    ]);
  }
  create(value?: PartialMessage<CreateStackRequest>): CreateStackRequest {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<CreateStackRequest>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: CreateStackRequest,
  ): CreateStackRequest {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntityMetaSpec meta */ 1:
          message.meta = EntityMetaSpec.internalBinaryRead(
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
    message: CreateStackRequest,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntityMetaSpec meta = 1; */
    if (message.meta)
      EntityMetaSpec.internalBinaryWrite(
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
 * @generated MessageType for protobuf message dipstick.stack.v1.CreateStackRequest
 */
export const CreateStackRequest = new CreateStackRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class CreateStackResponse$Type extends MessageType<CreateStackResponse> {
  constructor() {
    super("dipstick.stack.v1.CreateStackResponse", [
      { no: 1, name: "stack", kind: "message", T: () => StackEntity },
    ]);
  }
  create(value?: PartialMessage<CreateStackResponse>): CreateStackResponse {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<CreateStackResponse>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: CreateStackResponse,
  ): CreateStackResponse {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.stack.v1.StackEntity stack */ 1:
          message.stack = StackEntity.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.stack,
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
    message: CreateStackResponse,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.stack.v1.StackEntity stack = 1; */
    if (message.stack)
      StackEntity.internalBinaryWrite(
        message.stack,
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
 * @generated MessageType for protobuf message dipstick.stack.v1.CreateStackResponse
 */
export const CreateStackResponse = new CreateStackResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetStackRequest$Type extends MessageType<GetStackRequest> {
  constructor() {
    super("dipstick.stack.v1.GetStackRequest", [
      { no: 1, name: "selector", kind: "message", T: () => EntitySelector },
    ]);
  }
  create(value?: PartialMessage<GetStackRequest>): GetStackRequest {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<GetStackRequest>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: GetStackRequest,
  ): GetStackRequest {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntitySelector selector */ 1:
          message.selector = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.selector,
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
    message: GetStackRequest,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntitySelector selector = 1; */
    if (message.selector)
      EntitySelector.internalBinaryWrite(
        message.selector,
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
 * @generated MessageType for protobuf message dipstick.stack.v1.GetStackRequest
 */
export const GetStackRequest = new GetStackRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetStackResponse$Type extends MessageType<GetStackResponse> {
  constructor() {
    super("dipstick.stack.v1.GetStackResponse", [
      { no: 1, name: "stack", kind: "message", T: () => StackEntity },
    ]);
  }
  create(value?: PartialMessage<GetStackResponse>): GetStackResponse {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<GetStackResponse>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: GetStackResponse,
  ): GetStackResponse {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.stack.v1.StackEntity stack */ 1:
          message.stack = StackEntity.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.stack,
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
    message: GetStackResponse,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.stack.v1.StackEntity stack = 1; */
    if (message.stack)
      StackEntity.internalBinaryWrite(
        message.stack,
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
 * @generated MessageType for protobuf message dipstick.stack.v1.GetStackResponse
 */
export const GetStackResponse = new GetStackResponse$Type();
/**
 * @generated ServiceType for protobuf service dipstick.stack.v1.StackService
 */
export const StackService = new ServiceType("dipstick.stack.v1.StackService", [
  {
    name: "CreateStack",
    options: {},
    I: CreateStackRequest,
    O: CreateStackResponse,
  },
  { name: "GetStack", options: {}, I: GetStackRequest, O: GetStackResponse },
]);
