import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import { useMemo } from "react";

export default function useGrpcTransport() {
  const transport = useMemo(
    () =>
      new GrpcWebFetchTransport({
        baseUrl: import.meta.env.VITE_DIPSTICK_BASE_URL,
      }),
    [],
  );
  return transport;
}
