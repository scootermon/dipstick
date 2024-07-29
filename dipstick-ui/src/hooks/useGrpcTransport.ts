import { useMemo } from "react";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

export default function useGrpcTransport() {
  const transport = useMemo(
    () =>
      new GrpcWebFetchTransport({
        baseUrl: "http://10.90.40.13:50051",
      }),
    [],
  );
  return transport;
}
