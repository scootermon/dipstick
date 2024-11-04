import {
  AsyncOperationState,
  useGrpcTransport,
  useGrpcUnaryCall,
} from "./grpc";
import { EntityMeta } from "@/api/dipstick/core/v1/entity";
import { LogEvent } from "@/api/dipstick/core/v1/logging";
import { CoreServiceClient } from "@/api/dipstick/core/v1/service.client";
import { useEffect, useMemo, useState } from "react";

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

export function useLogs(opts?: { limit?: number }): LogEvent[] {
  const { limit = 500 } = opts ?? {};

  const client = useCoreClient();
  const [logs, setLogs] = useState<LogEvent[]>([]);

  useEffect(() => {
    const abort = new AbortController();
    const call = client.logSubscribe({}, { signal: abort.signal });
    call.responses.onMessage((msg) => {
      const event = msg.event;
      if (!event) {
        return;
      }
      setLogs((logs) => {
        if (logs.length >= limit) {
          return [...logs.slice(1), event];
        } else {
          return logs;
        }
      });
    });
  }, [client, limit]);
  return logs;
}
