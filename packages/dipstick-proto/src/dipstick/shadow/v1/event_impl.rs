use super::{ShadowEvent, ShadowEventVariant, SignalEvent};

impl From<SignalEvent> for ShadowEvent {
    fn from(event: SignalEvent) -> Self {
        Self {
            shadow_event_variant: Some(ShadowEventVariant::Signal(event)),
        }
    }
}
