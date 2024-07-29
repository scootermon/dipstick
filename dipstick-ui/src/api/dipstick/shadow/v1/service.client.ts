// @generated by protobuf-ts 2.9.4
// @generated from protobuf file "dipstick/shadow/v1/service.proto" (package "dipstick.shadow.v1", syntax proto3)
// tslint:disable
import type { RpcTransport } from "@protobuf-ts/runtime-rpc";
import type { ServiceInfo } from "@protobuf-ts/runtime-rpc";
import { ShadowService } from "./service";
import type { ShadowEventsResponse } from "./service";
import type { ShadowEventsRequest } from "./service";
import type { ServerStreamingCall } from "@protobuf-ts/runtime-rpc";
import type { GetShadowResponse } from "./service";
import type { GetShadowRequest } from "./service";
import { stackIntercept } from "@protobuf-ts/runtime-rpc";
import type { CreateShadowResponse } from "./service";
import type { CreateShadowRequest } from "./service";
import type { UnaryCall } from "@protobuf-ts/runtime-rpc";
import type { RpcOptions } from "@protobuf-ts/runtime-rpc";
/**
 * @generated from protobuf service dipstick.shadow.v1.ShadowService
 */
export interface IShadowServiceClient {
  /**
   * @generated from protobuf rpc: CreateShadow(dipstick.shadow.v1.CreateShadowRequest) returns (dipstick.shadow.v1.CreateShadowResponse);
   */
  createShadow(
    input: CreateShadowRequest,
    options?: RpcOptions,
  ): UnaryCall<CreateShadowRequest, CreateShadowResponse>;
  /**
   * @generated from protobuf rpc: GetShadow(dipstick.shadow.v1.GetShadowRequest) returns (dipstick.shadow.v1.GetShadowResponse);
   */
  getShadow(
    input: GetShadowRequest,
    options?: RpcOptions,
  ): UnaryCall<GetShadowRequest, GetShadowResponse>;
  /**
   * @generated from protobuf rpc: ShadowEvents(dipstick.shadow.v1.ShadowEventsRequest) returns (stream dipstick.shadow.v1.ShadowEventsResponse);
   */
  shadowEvents(
    input: ShadowEventsRequest,
    options?: RpcOptions,
  ): ServerStreamingCall<ShadowEventsRequest, ShadowEventsResponse>;
}
/**
 * @generated from protobuf service dipstick.shadow.v1.ShadowService
 */
export class ShadowServiceClient implements IShadowServiceClient, ServiceInfo {
  typeName = ShadowService.typeName;
  methods = ShadowService.methods;
  options = ShadowService.options;
  constructor(private readonly _transport: RpcTransport) {}
  /**
   * @generated from protobuf rpc: CreateShadow(dipstick.shadow.v1.CreateShadowRequest) returns (dipstick.shadow.v1.CreateShadowResponse);
   */
  createShadow(
    input: CreateShadowRequest,
    options?: RpcOptions,
  ): UnaryCall<CreateShadowRequest, CreateShadowResponse> {
    const method = this.methods[0],
      opt = this._transport.mergeOptions(options);
    return stackIntercept<CreateShadowRequest, CreateShadowResponse>(
      "unary",
      this._transport,
      method,
      opt,
      input,
    );
  }
  /**
   * @generated from protobuf rpc: GetShadow(dipstick.shadow.v1.GetShadowRequest) returns (dipstick.shadow.v1.GetShadowResponse);
   */
  getShadow(
    input: GetShadowRequest,
    options?: RpcOptions,
  ): UnaryCall<GetShadowRequest, GetShadowResponse> {
    const method = this.methods[1],
      opt = this._transport.mergeOptions(options);
    return stackIntercept<GetShadowRequest, GetShadowResponse>(
      "unary",
      this._transport,
      method,
      opt,
      input,
    );
  }
  /**
   * @generated from protobuf rpc: ShadowEvents(dipstick.shadow.v1.ShadowEventsRequest) returns (stream dipstick.shadow.v1.ShadowEventsResponse);
   */
  shadowEvents(
    input: ShadowEventsRequest,
    options?: RpcOptions,
  ): ServerStreamingCall<ShadowEventsRequest, ShadowEventsResponse> {
    const method = this.methods[2],
      opt = this._transport.mergeOptions(options);
    return stackIntercept<ShadowEventsRequest, ShadowEventsResponse>(
      "serverStreaming",
      this._transport,
      method,
      opt,
      input,
    );
  }
}