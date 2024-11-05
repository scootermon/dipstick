import { useGrpcTransport } from "./grpc";
import { SpiServiceClient } from "@/api/dipstick/spi/v1/service.client";
import { useMemo } from "react";

export function useSpiClient() {
  const transport = useGrpcTransport();
  const client = useMemo(() => new SpiServiceClient(transport), [transport]);
  return client;
}
