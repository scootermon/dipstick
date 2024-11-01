import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { RpcOptions, UnaryCall } from "@protobuf-ts/runtime-rpc";
import { useEffect, useMemo, useState } from "react";

export function useGrpcTransport() {
  const transport = useMemo(
    () =>
      new GrpcWebFetchTransport({
        baseUrl: import.meta.env.VITE_DIPSTICK_BASE_URL,
      }),
    [],
  );
  return transport;
}

export type AsyncOperationState<O> = {
  loading: boolean;
  error: null | unknown;
  value: O;
};

export function useGrpcUnaryCall<I extends object, O extends object>({
  input,
  method,
}: {
  input: I;
  method: (input: I, options: RpcOptions) => UnaryCall<I, O>;
}): AsyncOperationState<O | null> {
  const [state, setState] = useState<AsyncOperationState<O | null>>(() => ({
    loading: false,
    error: null,
    value: null,
  }));
  useEffect(() => {
    const abort = new AbortController();
    const call = method(input, { abort: abort.signal });
    setState((state) => ({ ...state, loading: true }));
    call.then(
      (value) => {
        setState({
          loading: false,
          error: null,
          value: value.response,
        });
      },
      (reason) => {
        setState({
          loading: false,
          error: reason,
          value: null,
        });
      },
    );

    return () => {
      abort.abort();
    };
  }, [method, input]);
  return state;
}
