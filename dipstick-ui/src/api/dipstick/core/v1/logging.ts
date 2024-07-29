// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/core/v1/logging.proto" (package "dipstick.core.v1", syntax proto3)
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
import { Value } from "../../../google/protobuf/struct";
import { Timestamp } from "../../../google/protobuf/timestamp";
/**
 * Log event
 *
 * @generated from protobuf message dipstick.core.v1.LogEvent
 */
export interface LogEvent {
  /**
   * Timestamp
   *
   * @generated from protobuf field: google.protobuf.Timestamp timestamp = 1;
   */
  timestamp?: Timestamp;
  /**
   * Log level of the event.
   *
   * @generated from protobuf field: dipstick.core.v1.LogLevel level = 2;
   */
  level: LogLevel;
  /**
   * Target module that generated the event.
   *
   * @generated from protobuf field: string target = 3;
   */
  target: string;
  /**
   * Log message.
   *
   * @generated from protobuf field: string message = 4;
   */
  message: string;
  /**
   * Additional fields.
   *
   * @generated from protobuf field: map<string, google.protobuf.Value> fields = 5;
   */
  fields: {
    [key: string]: Value;
  };
  /**
   * Active spans.
   *
   * @generated from protobuf field: repeated dipstick.core.v1.LogSpan spans = 6;
   */
  spans: LogSpan[];
}
/**
 * Log span
 *
 * @generated from protobuf message dipstick.core.v1.LogSpan
 */
export interface LogSpan {
  /**
   * Unique id of the span.
   *
   * @generated from protobuf field: uint64 id = 1;
   */
  id: bigint;
  /**
   * Log level of the span.
   *
   * @generated from protobuf field: dipstick.core.v1.LogLevel level = 5;
   */
  level: LogLevel;
  /**
   * User defined name of the span.
   *
   * @generated from protobuf field: string name = 2;
   */
  name: string;
  /**
   * Target where the span was created.
   *
   * @generated from protobuf field: string target = 4;
   */
  target: string;
  /**
   * Fields.
   *
   * @generated from protobuf field: map<string, google.protobuf.Value> fields = 3;
   */
  fields: {
    [key: string]: Value;
  };
}
/**
 * Log config
 *
 * @generated from protobuf message dipstick.core.v1.LogConfig
 */
export interface LogConfig {
  /**
   * Default log level.
   *
   * @generated from protobuf field: dipstick.core.v1.LogLevel default_level = 1;
   */
  defaultLevel: LogLevel;
  /**
   * Target-specific log levels.
   *
   * @generated from protobuf field: map<string, dipstick.core.v1.LogLevel> target_filters = 2;
   */
  targetFilters: {
    [key: string]: LogLevel;
  };
}
/**
 * Log level
 *
 * @generated from protobuf enum dipstick.core.v1.LogLevel
 */
export enum LogLevel {
  /**
   * Unspecified
   *
   * @generated from protobuf enum value: LOG_LEVEL_UNSPECIFIED = 0;
   */
  UNSPECIFIED = 0,
  /**
   * Trace
   *
   * @generated from protobuf enum value: LOG_LEVEL_TRACE = 1;
   */
  TRACE = 1,
  /**
   * Debug
   *
   * @generated from protobuf enum value: LOG_LEVEL_DEBUG = 2;
   */
  DEBUG = 2,
  /**
   * Info
   *
   * @generated from protobuf enum value: LOG_LEVEL_INFO = 3;
   */
  INFO = 3,
  /**
   * Warn
   *
   * @generated from protobuf enum value: LOG_LEVEL_WARN = 4;
   */
  WARN = 4,
  /**
   * Error
   *
   * @generated from protobuf enum value: LOG_LEVEL_ERROR = 5;
   */
  ERROR = 5,
  /**
   * Off. Only used for filters
   *
   * @generated from protobuf enum value: LOG_LEVEL_OFF = 6;
   */
  OFF = 6,
}
// @generated message type with reflection information, may provide speed optimized methods
class LogEvent$Type extends MessageType<LogEvent> {
  constructor() {
    super("dipstick.core.v1.LogEvent", [
      { no: 1, name: "timestamp", kind: "message", T: () => Timestamp },
      {
        no: 2,
        name: "level",
        kind: "enum",
        T: () => ["dipstick.core.v1.LogLevel", LogLevel, "LOG_LEVEL_"],
      },
      { no: 3, name: "target", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
      { no: 4, name: "message", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
      {
        no: 5,
        name: "fields",
        kind: "map",
        K: 9 /*ScalarType.STRING*/,
        V: { kind: "message", T: () => Value },
      },
      {
        no: 6,
        name: "spans",
        kind: "message",
        repeat: 1 /*RepeatType.PACKED*/,
        T: () => LogSpan,
      },
    ]);
  }
  create(value?: PartialMessage<LogEvent>): LogEvent {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.level = 0;
    message.target = "";
    message.message = "";
    message.fields = {};
    message.spans = [];
    if (value !== undefined)
      reflectionMergePartial<LogEvent>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: LogEvent,
  ): LogEvent {
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
        case /* dipstick.core.v1.LogLevel level */ 2:
          message.level = reader.int32();
          break;
        case /* string target */ 3:
          message.target = reader.string();
          break;
        case /* string message */ 4:
          message.message = reader.string();
          break;
        case /* map<string, google.protobuf.Value> fields */ 5:
          this.binaryReadMap5(message.fields, reader, options);
          break;
        case /* repeated dipstick.core.v1.LogSpan spans */ 6:
          message.spans.push(
            LogSpan.internalBinaryRead(reader, reader.uint32(), options),
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
  private binaryReadMap5(
    map: LogEvent["fields"],
    reader: IBinaryReader,
    options: BinaryReadOptions,
  ): void {
    let len = reader.uint32(),
      end = reader.pos + len,
      key: keyof LogEvent["fields"] | undefined,
      val: LogEvent["fields"][any] | undefined;
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
            "unknown map entry field for field dipstick.core.v1.LogEvent.fields",
          );
      }
    }
    map[key ?? ""] = val ?? Value.create();
  }
  internalBinaryWrite(
    message: LogEvent,
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
    /* dipstick.core.v1.LogLevel level = 2; */
    if (message.level !== 0)
      writer.tag(2, WireType.Varint).int32(message.level);
    /* string target = 3; */
    if (message.target !== "")
      writer.tag(3, WireType.LengthDelimited).string(message.target);
    /* string message = 4; */
    if (message.message !== "")
      writer.tag(4, WireType.LengthDelimited).string(message.message);
    /* map<string, google.protobuf.Value> fields = 5; */
    for (let k of globalThis.Object.keys(message.fields)) {
      writer
        .tag(5, WireType.LengthDelimited)
        .fork()
        .tag(1, WireType.LengthDelimited)
        .string(k);
      writer.tag(2, WireType.LengthDelimited).fork();
      Value.internalBinaryWrite(message.fields[k], writer, options);
      writer.join().join();
    }
    /* repeated dipstick.core.v1.LogSpan spans = 6; */
    for (let i = 0; i < message.spans.length; i++)
      LogSpan.internalBinaryWrite(
        message.spans[i],
        writer.tag(6, WireType.LengthDelimited).fork(),
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
 * @generated MessageType for protobuf message dipstick.core.v1.LogEvent
 */
export const LogEvent = new LogEvent$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LogSpan$Type extends MessageType<LogSpan> {
  constructor() {
    super("dipstick.core.v1.LogSpan", [
      {
        no: 1,
        name: "id",
        kind: "scalar",
        T: 4 /*ScalarType.UINT64*/,
        L: 0 /*LongType.BIGINT*/,
      },
      {
        no: 5,
        name: "level",
        kind: "enum",
        T: () => ["dipstick.core.v1.LogLevel", LogLevel, "LOG_LEVEL_"],
      },
      { no: 2, name: "name", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
      { no: 4, name: "target", kind: "scalar", T: 9 /*ScalarType.STRING*/ },
      {
        no: 3,
        name: "fields",
        kind: "map",
        K: 9 /*ScalarType.STRING*/,
        V: { kind: "message", T: () => Value },
      },
    ]);
  }
  create(value?: PartialMessage<LogSpan>): LogSpan {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.id = 0n;
    message.level = 0;
    message.name = "";
    message.target = "";
    message.fields = {};
    if (value !== undefined)
      reflectionMergePartial<LogSpan>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: LogSpan,
  ): LogSpan {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* uint64 id */ 1:
          message.id = reader.uint64().toBigInt();
          break;
        case /* dipstick.core.v1.LogLevel level */ 5:
          message.level = reader.int32();
          break;
        case /* string name */ 2:
          message.name = reader.string();
          break;
        case /* string target */ 4:
          message.target = reader.string();
          break;
        case /* map<string, google.protobuf.Value> fields */ 3:
          this.binaryReadMap3(message.fields, reader, options);
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
  private binaryReadMap3(
    map: LogSpan["fields"],
    reader: IBinaryReader,
    options: BinaryReadOptions,
  ): void {
    let len = reader.uint32(),
      end = reader.pos + len,
      key: keyof LogSpan["fields"] | undefined,
      val: LogSpan["fields"][any] | undefined;
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
            "unknown map entry field for field dipstick.core.v1.LogSpan.fields",
          );
      }
    }
    map[key ?? ""] = val ?? Value.create();
  }
  internalBinaryWrite(
    message: LogSpan,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* uint64 id = 1; */
    if (message.id !== 0n) writer.tag(1, WireType.Varint).uint64(message.id);
    /* dipstick.core.v1.LogLevel level = 5; */
    if (message.level !== 0)
      writer.tag(5, WireType.Varint).int32(message.level);
    /* string name = 2; */
    if (message.name !== "")
      writer.tag(2, WireType.LengthDelimited).string(message.name);
    /* string target = 4; */
    if (message.target !== "")
      writer.tag(4, WireType.LengthDelimited).string(message.target);
    /* map<string, google.protobuf.Value> fields = 3; */
    for (let k of globalThis.Object.keys(message.fields)) {
      writer
        .tag(3, WireType.LengthDelimited)
        .fork()
        .tag(1, WireType.LengthDelimited)
        .string(k);
      writer.tag(2, WireType.LengthDelimited).fork();
      Value.internalBinaryWrite(message.fields[k], writer, options);
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
 * @generated MessageType for protobuf message dipstick.core.v1.LogSpan
 */
export const LogSpan = new LogSpan$Type();
// @generated message type with reflection information, may provide speed optimized methods
class LogConfig$Type extends MessageType<LogConfig> {
  constructor() {
    super("dipstick.core.v1.LogConfig", [
      {
        no: 1,
        name: "default_level",
        kind: "enum",
        T: () => ["dipstick.core.v1.LogLevel", LogLevel, "LOG_LEVEL_"],
      },
      {
        no: 2,
        name: "target_filters",
        kind: "map",
        K: 9 /*ScalarType.STRING*/,
        V: {
          kind: "enum",
          T: () => ["dipstick.core.v1.LogLevel", LogLevel, "LOG_LEVEL_"],
        },
      },
    ]);
  }
  create(value?: PartialMessage<LogConfig>): LogConfig {
    const message = globalThis.Object.create(this.messagePrototype!);
    message.defaultLevel = 0;
    message.targetFilters = {};
    if (value !== undefined)
      reflectionMergePartial<LogConfig>(this, message, value);
    return message;
  }
  internalBinaryRead(
    reader: IBinaryReader,
    length: number,
    options: BinaryReadOptions,
    target?: LogConfig,
  ): LogConfig {
    let message = target ?? this.create(),
      end = reader.pos + length;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case /* dipstick.core.v1.LogLevel default_level */ 1:
          message.defaultLevel = reader.int32();
          break;
        case /* map<string, dipstick.core.v1.LogLevel> target_filters */ 2:
          this.binaryReadMap2(message.targetFilters, reader, options);
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
    map: LogConfig["targetFilters"],
    reader: IBinaryReader,
    options: BinaryReadOptions,
  ): void {
    let len = reader.uint32(),
      end = reader.pos + len,
      key: keyof LogConfig["targetFilters"] | undefined,
      val: LogConfig["targetFilters"][any] | undefined;
    while (reader.pos < end) {
      let [fieldNo, wireType] = reader.tag();
      switch (fieldNo) {
        case 1:
          key = reader.string();
          break;
        case 2:
          val = reader.int32();
          break;
        default:
          throw new globalThis.Error(
            "unknown map entry field for field dipstick.core.v1.LogConfig.target_filters",
          );
      }
    }
    map[key ?? ""] = val ?? 0;
  }
  internalBinaryWrite(
    message: LogConfig,
    writer: IBinaryWriter,
    options: BinaryWriteOptions,
  ): IBinaryWriter {
    /* dipstick.core.v1.LogLevel default_level = 1; */
    if (message.defaultLevel !== 0)
      writer.tag(1, WireType.Varint).int32(message.defaultLevel);
    /* map<string, dipstick.core.v1.LogLevel> target_filters = 2; */
    for (let k of globalThis.Object.keys(message.targetFilters))
      writer
        .tag(2, WireType.LengthDelimited)
        .fork()
        .tag(1, WireType.LengthDelimited)
        .string(k)
        .tag(2, WireType.Varint)
        .int32(message.targetFilters[k])
        .join();
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
 * @generated MessageType for protobuf message dipstick.core.v1.LogConfig
 */
export const LogConfig = new LogConfig$Type();
