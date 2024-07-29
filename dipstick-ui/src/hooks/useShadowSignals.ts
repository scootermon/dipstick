import { useEffect, useReducer } from "react";
import { ShadowServiceClient } from "../api/dipstick/shadow/v1/service.client";
import { SignalStatus } from "../api/dipstick/shadow/v1/signal";
import { Value } from "../api/google/protobuf/struct";
import { Timestamp } from "../api/google/protobuf/timestamp";
import useShadowClient from "./useShadowClient";

const nullValue: Value = { kind: { oneofKind: "nullValue", nullValue: 0 } };

export default function useShadowSignals(uniqueId: number) {
  const client = useShadowClient();
  const [state, dispatch] = useReducer(signalsReducer, undefined, () => ({
    signals: {},
  }));

  useEffect(() => {
    const abort = new AbortController();

    const call = client.shadowEvents(
      {
        selector: {
          entitySelectorVariant: {
            oneofKind: "uniqueId",
            uniqueId,
          },
        },
      },
      { abort: abort.signal },
    );
    call.responses.onMessage((resp) => {
      if (resp.event?.shadowEventVariant.oneofKind === "signal") {
        const status = resp.event.shadowEventVariant.signal;
        dispatch({
          type: "updateSignal",
          key: status.signal,
          value: status.value ?? nullValue,
          changedAt: status.timestamp
            ? Timestamp.toDate(status.timestamp)
            : new Date(),
        });
      }
    });

    initializeShadowSignals({
      client,
      uniqueId,
      dispatch,
      abort: abort.signal,
    });

    return () => {
      abort.abort();
    };
  }, [client, uniqueId, dispatch]);

  return state;
}

type SignalState = {
  updateCount: number;
  changedAt: Date | null;
  value: Value;
};

function signalStateFromStatus(status: SignalStatus): SignalState {
  return {
    updateCount: status.updateCount,
    changedAt: status.changedAt ? Timestamp.toDate(status.changedAt) : null,
    value: status.value ?? nullValue,
  };
}

type SignalsState = {
  signals: Record<string, SignalState>;
};

type SignalsAction =
  | {
      type: "fillSignals";
      signals: Record<string, SignalState>;
    }
  | {
      type: "updateSignal";
      key: string;
      value: Value;
      changedAt: Date;
    };

function signalsReducer(
  state: SignalsState,
  action: SignalsAction,
): SignalsState {
  switch (action.type) {
    case "fillSignals":
      return {
        ...state,
        signals: { ...action.signals, ...state.signals },
      };
    case "updateSignal":
      return {
        signals: {
          ...state.signals,
          [action.key]: {
            value: action.value,
            updateCount: (state.signals[action.key]?.updateCount ?? 0) + 1,
            changedAt: action.changedAt,
          },
        },
      };
  }
}

async function initializeShadowSignals({
  client,
  uniqueId,
  dispatch,
  abort,
}: {
  client: ShadowServiceClient;
  uniqueId: number;
  dispatch: React.Dispatch<SignalsAction>;
  abort: AbortSignal;
}) {
  const { response } = await client.getShadow(
    {
      selector: {
        entitySelectorVariant: {
          oneofKind: "uniqueId",
          uniqueId,
        },
      },
    },
    { abort },
  );
  const signalMap = response.shadow?.status?.signals;
  if (signalMap) {
    dispatch({
      type: "fillSignals",
      signals: Object.fromEntries(
        Object.entries(signalMap).map(([key, status]) => [
          key,
          signalStateFromStatus(status),
        ]),
      ),
    });
  }
}
