import { useMemo } from "react";
import useGrpcTransport from "./useGrpcTransport";
import { ShadowServiceClient } from "../api/dipstick/shadow/v1/service.client";

export default function useShadowClient() {
  const transport = useGrpcTransport();
  const client = useMemo(() => new ShadowServiceClient(transport), [transport]);
  return client;
}
