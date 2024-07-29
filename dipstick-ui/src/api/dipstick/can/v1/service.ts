// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/can/v1/service.proto" (package "dipstick.can.v1", syntax proto3)
// tslint:disable
import { EntitySelector } from "../../core/v1/entity";
import { EntityMetaSpec } from "../../core/v1/entity";
import { BusEntity } from "./bus";
import { BusSpec } from "./bus";
import { Frame } from "./frame";
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
 * @generated from protobuf message dipstick.can.v1.CreateBusRequest
 */
export interface CreateBusRequest {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntityMetaSpec meta = 1;
   */
  meta?: EntityMetaSpec;
  /**
   * @generated from protobuf field: dipstick.can.v1.BusSpec spec = 2;
   */
  spec?: BusSpec;
}
/**
 * @generated from protobuf message dipstick.can.v1.CreateBusResponse
 */
export interface CreateBusResponse {
  /**
   * @generated from protobuf field: dipstick.can.v1.BusEntity bus = 1;
   */
  bus?: BusEntity;
}
/**
 * @generated from protobuf message dipstick.can.v1.GetBusRequest
 */
export interface GetBusRequest {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector bus = 1;
   */
  bus?: EntitySelector;
}
/**
 * @generated from protobuf message dipstick.can.v1.GetBusResponse
 */
export interface GetBusResponse {
  /**
   * @generated from protobuf field: dipstick.can.v1.BusEntity bus = 1;
   */
  bus?: BusEntity;
}
/**
 * @generated from protobuf message dipstick.can.v1.SendFrameRequest
 */
export interface SendFrameRequest {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector bus = 1;
   */
  bus?: EntitySelector;
  /**
   * @generated from protobuf field: dipstick.can.v1.Frame frame = 2;
   */
  frame?: Frame;
}
/**
 * @generated from protobuf message dipstick.can.v1.SendFrameResponse
 */
export interface SendFrameResponse {}
/**
 * @generated from protobuf message dipstick.can.v1.ReceiveFramesRequest
 */
export interface ReceiveFramesRequest {
  /**
   * @generated from protobuf field: dipstick.core.v1.EntitySelector bus = 1;
   */
  bus?: EntitySelector;
}
/**
 * @generated from protobuf message dipstick.can.v1.ReceiveFramesResponse
 */
export interface ReceiveFramesResponse {
  /**
   * @generated from protobuf field: dipstick.can.v1.Frame frame = 1;
   */
  frame?: Frame;
}
// @generated message type with reflection information, may provide speed optimized methods
class CreateBusRequest$Type extends MessageType<CreateBusRequest> {
  constructor() {
    super("dipstick.can.v1.CreateBusRequest", [
      { no: 1, name: "meta", kind: "message", T: () => EntityMetaSpec },
      { no: 2, name: "spec", kind: "message", T: () => BusSpec },
    ]);
  }
  create(value?: PartialMessage<CreateBusRequest>): CreateBusRequest {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<CreateBusRequest>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: CreateBusRequest,
  ): CreateBusRequest {
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
        case /* dipstick.can.v1.BusSpec spec */ 2:
          message.spec = BusSpec.internalBinaryRead(
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
    message: CreateBusRequest,
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
    /* dipstick.can.v1.BusSpec spec = 2; */
    if (message.spec)
      BusSpec.internalBinaryWrite(
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
 * @generated MessageType for protobuf message dipstick.can.v1.CreateBusRequest
 */
export const CreateBusRequest = new CreateBusRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class CreateBusResponse$Type extends MessageType<CreateBusResponse> {
  constructor() {
    super("dipstick.can.v1.CreateBusResponse", [
      { no: 1, name: "bus", kind: "message", T: () => BusEntity },
    ]);
  }
  create(value?: PartialMessage<CreateBusResponse>): CreateBusResponse {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<CreateBusResponse>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: CreateBusResponse,
  ): CreateBusResponse {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.can.v1.BusEntity bus */ 1:
          message.bus = BusEntity.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.bus,
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
    message: CreateBusResponse,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.can.v1.BusEntity bus = 1; */
    if (message.bus)
      BusEntity.internalBinaryWrite(
        message.bus,
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
 * @generated MessageType for protobuf message dipstick.can.v1.CreateBusResponse
 */
export const CreateBusResponse = new CreateBusResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetBusRequest$Type extends MessageType<GetBusRequest> {
  constructor() {
    super("dipstick.can.v1.GetBusRequest", [
      { no: 1, name: "bus", kind: "message", T: () => EntitySelector },
    ]);
  }
  create(value?: PartialMessage<GetBusRequest>): GetBusRequest {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<GetBusRequest>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: GetBusRequest,
  ): GetBusRequest {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntitySelector bus */ 1:
          message.bus = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.bus,
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
    message: GetBusRequest,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntitySelector bus = 1; */
    if (message.bus)
      EntitySelector.internalBinaryWrite(
        message.bus,
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
 * @generated MessageType for protobuf message dipstick.can.v1.GetBusRequest
 */
export const GetBusRequest = new GetBusRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class GetBusResponse$Type extends MessageType<GetBusResponse> {
  constructor() {
    super("dipstick.can.v1.GetBusResponse", [
      { no: 1, name: "bus", kind: "message", T: () => BusEntity },
    ]);
  }
  create(value?: PartialMessage<GetBusResponse>): GetBusResponse {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<GetBusResponse>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: GetBusResponse,
  ): GetBusResponse {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.can.v1.BusEntity bus */ 1:
          message.bus = BusEntity.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.bus,
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
    message: GetBusResponse,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.can.v1.BusEntity bus = 1; */
    if (message.bus)
      BusEntity.internalBinaryWrite(
        message.bus,
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
 * @generated MessageType for protobuf message dipstick.can.v1.GetBusResponse
 */
export const GetBusResponse = new GetBusResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class SendFrameRequest$Type extends MessageType<SendFrameRequest> {
  constructor() {
    super("dipstick.can.v1.SendFrameRequest", [
      { no: 1, name: "bus", kind: "message", T: () => EntitySelector },
      { no: 2, name: "frame", kind: "message", T: () => Frame },
    ]);
  }
  create(value?: PartialMessage<SendFrameRequest>): SendFrameRequest {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<SendFrameRequest>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: SendFrameRequest,
  ): SendFrameRequest {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntitySelector bus */ 1:
          message.bus = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.bus,
          );
          break;
        case /* dipstick.can.v1.Frame frame */ 2:
          message.frame = Frame.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.frame,
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
    message: SendFrameRequest,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntitySelector bus = 1; */
    if (message.bus)
      EntitySelector.internalBinaryWrite(
        message.bus,
        writer.tag(1, WireType.LengthDelimited).fork(),
        options,
      ).join();
    /* dipstick.can.v1.Frame frame = 2; */
    if (message.frame)
      Frame.internalBinaryWrite(
        message.frame,
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
 * @generated MessageType for protobuf message dipstick.can.v1.SendFrameRequest
 */
export const SendFrameRequest = new SendFrameRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class SendFrameResponse$Type extends MessageType<SendFrameResponse> {
  constructor() {
    super("dipstick.can.v1.SendFrameResponse", []);
  }
  create(value?: PartialMessage<SendFrameResponse>): SendFrameResponse {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<SendFrameResponse>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: SendFrameResponse,
  ): SendFrameResponse {
    return target ?? this.create();
  }
  internalBinaryWrite(
    message: SendFrameResponse,
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
 * @generated MessageType for protobuf message dipstick.can.v1.SendFrameResponse
 */
export const SendFrameResponse = new SendFrameResponse$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ReceiveFramesRequest$Type extends MessageType<ReceiveFramesRequest> {
  constructor() {
    super("dipstick.can.v1.ReceiveFramesRequest", [
      { no: 1, name: "bus", kind: "message", T: () => EntitySelector },
    ]);
  }
  create(value?: PartialMessage<ReceiveFramesRequest>): ReceiveFramesRequest {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<ReceiveFramesRequest>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: ReceiveFramesRequest,
  ): ReceiveFramesRequest {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.EntitySelector bus */ 1:
          message.bus = EntitySelector.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.bus,
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
    message: ReceiveFramesRequest,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.EntitySelector bus = 1; */
    if (message.bus)
      EntitySelector.internalBinaryWrite(
        message.bus,
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
 * @generated MessageType for protobuf message dipstick.can.v1.ReceiveFramesRequest
 */
export const ReceiveFramesRequest = new ReceiveFramesRequest$Type();
// @generated message type with reflection information, may provide speed optimized methods
class ReceiveFramesResponse$Type extends MessageType<ReceiveFramesResponse> {
  constructor() {
    super("dipstick.can.v1.ReceiveFramesResponse", [
      { no: 1, name: "frame", kind: "message", T: () => Frame },
    ]);
  }
  create(value?: PartialMessage<ReceiveFramesResponse>): ReceiveFramesResponse {
    const message = globalThis.Object.create(this.messagePrototype!);
    if (value !== undefined)
      reflectionMergePartial<ReceiveFramesResponse>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: ReceiveFramesResponse,
  ): ReceiveFramesResponse {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.can.v1.Frame frame */ 1:
          message.frame = Frame.internalBinaryRead(
            reader,
            reader.uint32(),
            options,
            message.frame,
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
    message: ReceiveFramesResponse,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.can.v1.Frame frame = 1; */
    if (message.frame)
      Frame.internalBinaryWrite(
        message.frame,
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
 * @generated MessageType for protobuf message dipstick.can.v1.ReceiveFramesResponse
 */
export const ReceiveFramesResponse = new ReceiveFramesResponse$Type();
/**
 * @generated ServiceType for protobuf service dipstick.can.v1.CanService
 */
export const CanService = new ServiceType("dipstick.can.v1.CanService", [
  { name: "CreateBus", options: {}, I: CreateBusRequest, O: CreateBusResponse },
  { name: "GetBus", options: {}, I: GetBusRequest, O: GetBusResponse },
  { name: "SendFrame", options: {}, I: SendFrameRequest, O: SendFrameResponse },
  {
    name: "ReceiveFrames",
    serverStreaming: true,
    options: {},
    I: ReceiveFramesRequest,
    O: ReceiveFramesResponse,
  },
]);
