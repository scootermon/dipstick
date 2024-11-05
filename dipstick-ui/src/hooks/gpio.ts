import { useGrpcTransport } from "./grpc";
import { GpioServiceClient } from "@/api/dipstick/gpio/v1/service.client";
import { useMemo } from "react";

export function useGpioClient() {
  const transport = useGrpcTransport();
  const client = useMemo(() => new GpioServiceClient(transport), [transport]);
  return client;
}
