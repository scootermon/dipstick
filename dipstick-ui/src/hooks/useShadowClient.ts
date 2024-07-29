import { ShadowServiceClient } from "../api/dipstick/shadow/v1/service.client";
import useGrpcTransport from "./useGrpcTransport";
import { useMemo } from "react";

export default function useShadowClient() {
  const transport = useGrpcTransport();
  const client = useMemo(() => new ShadowServiceClient(transport), [transport]);
  return client;
}
