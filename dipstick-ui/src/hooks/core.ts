import {
  AsyncOperationState,
  useGrpcTransport,
  useGrpcUnaryCall,
} from "./grpc";
import { EntityMeta } from "@/api/dipstick/core/v1/entity";
import { CoreServiceClient } from "@/api/dipstick/core/v1/service.client";
import { useMemo } from "react";

export function useCoreClient() {
  const transport = useGrpcTransport();
  const client = useMemo(() => new CoreServiceClient(transport), [transport]);
  return client;
}

export function useCoreEntities(): AsyncOperationState<EntityMeta[]> {
  const client = useCoreClient();
  const input = useMemo(
    () => ({ input: {}, method: client.listEntities.bind(client) }),
    [client],
  );
  const state = useGrpcUnaryCall(input);
  return { ...state, value: state.value?.entities ?? [] };
}
