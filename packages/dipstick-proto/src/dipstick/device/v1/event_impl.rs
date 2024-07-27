use super::{ActionEvent, DeviceEvent, DeviceEventVariant, SensorEvent};

impl From<ActionEvent> for DeviceEvent {
    fn from(event: ActionEvent) -> Self {
        Self {
            device_event_variant: Some(DeviceEventVariant::Action(event)),
        }
    }
}

impl From<SensorEvent> for DeviceEvent {
    fn from(event: SensorEvent) -> Self {
        Self {
            device_event_variant: Some(DeviceEventVariant::Sensor(event)),
        }
    }
}
