use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use dipstick_core::{Core, Dep};
use dipstick_proto::shadow::v1::{
    A2lCharacteristicSignalSpec, A2lMeasurementSignalSpec, DeviceSensorSignalSpec, GpioSignalSpec,
};
use futures::StreamExt;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_util::sync::{CancellationToken, DropGuard};
use tokio_util::task::TaskTracker;
use tonic::{Result, Status};

use crate::Shadow;

#[must_use]
pub struct Listeners {
    drop_guard: DropGuard,
    tracker: TaskTracker,
}

impl Listeners {
    pub fn builder(core: &Core, shadow: Arc<Shadow>) -> ListenersBuilder<'_> {
        ListenersBuilder::new(core, shadow)
    }

    pub async fn close(self) {
        self.tracker.close();
        drop(self.drop_guard);
        self.tracker.wait().await;
    }
}

pub struct ListenersBuilder<'a> {
    core: &'a Core,
    cancel_token: CancellationToken,
    shadow: Arc<Shadow>,
    gpio_listeners: Vec<GpioChipListener>,
    a2l_listeners: Vec<A2lListener>,
    device_listeners: Vec<DeviceListener>,
}

impl<'a> ListenersBuilder<'a> {
    pub fn new(core: &'a Core, shadow: Arc<Shadow>) -> Self {
        Self {
            core,
            cancel_token: core.new_cancel_token(),
            shadow,
            gpio_listeners: Vec::new(),
            a2l_listeners: Vec::new(),
            device_listeners: Vec::new(),
        }
    }

    pub fn build(self) -> Listeners {
        let tracker = TaskTracker::new();
        for listener in self.gpio_listeners {
            tracker.spawn(listener.run());
        }
        for listener in self.a2l_listeners {
            tracker.spawn(listener.run());
        }
        for listener in self.device_listeners {
            tracker.spawn(listener.run());
        }
        Listeners {
            drop_guard: self.cancel_token.drop_guard(),
            tracker,
        }
    }

    pub fn add_gpio_signal(&mut self, signal_id: String, spec: &mut GpioSignalSpec) -> Result<()> {
        let chip = self.core.select_entity_dep::<dipstick_gpio::Chip>(
            &self.shadow,
            spec.chip.clone().unwrap_or_default(),
        )?;
        let listener = self.get_gpio_chip_listener(chip);
        // TODO: check if chip has pin
        listener.add_mapping(spec.pin.clone(), signal_id)?;
        Ok(())
    }

    fn get_gpio_chip_listener(&mut self, chip: Dep<dipstick_gpio::Chip>) -> &mut GpioChipListener {
        let index = self
            .gpio_listeners
            .iter()
            .position(|listener| Dep::ptr_eq(&listener.chip, &chip));
        match index {
            // necessary to go through index to make the borrow checker happy
            Some(index) => self.gpio_listeners.get_mut(index).unwrap(),
            None => {
                self.gpio_listeners.push(GpioChipListener::new(
                    self.cancel_token.clone(),
                    Arc::clone(&self.shadow),
                    chip,
                ));
                self.gpio_listeners.last_mut().unwrap()
            }
        }
    }

    pub fn add_a2l_characteristic_signal(
        &mut self,
        signal_id: String,
        spec: &mut A2lCharacteristicSignalSpec,
    ) -> Result<()> {
        let listener = A2lListener::new_characteristic(
            self.cancel_token.clone(),
            Arc::clone(&self.shadow),
            self.core,
            signal_id,
            spec,
        )?;
        self.a2l_listeners.push(listener);
        Ok(())
    }

    pub fn add_a2l_measurement_signal(
        &mut self,
        signal_id: String,
        spec: &mut A2lMeasurementSignalSpec,
    ) -> Result<()> {
        let listener = A2lListener::new_measurement(
            self.cancel_token.clone(),
            Arc::clone(&self.shadow),
            self.core,
            signal_id,
            spec,
        )?;
        self.a2l_listeners.push(listener);
        Ok(())
    }

    pub fn add_device_sensor_signal(
        &mut self,
        signal_id: String,
        spec: &mut DeviceSensorSignalSpec,
    ) -> Result<()> {
        let device = self.core.select_entity_dep::<dipstick_device::Device>(
            &self.shadow,
            spec.device.clone().unwrap_or_default(),
        )?;
        let listener = self.get_device_listener(device);
        // TODO: check if device has sensor
        listener.add_sensor_mapping(spec.sensor.clone(), signal_id);
        Ok(())
    }

    fn get_device_listener(&mut self, device: Dep<dipstick_device::Device>) -> &mut DeviceListener {
        let index = self
            .device_listeners
            .iter()
            .position(|listener| Dep::ptr_eq(&listener.device, &device));
        match index {
            // necessary to go through index to make the borrow checker happy
            Some(index) => self.device_listeners.get_mut(index).unwrap(),
            None => {
                self.device_listeners.push(DeviceListener::new(
                    self.cancel_token.clone(),
                    Arc::clone(&self.shadow),
                    device,
                ));
                self.device_listeners.last_mut().unwrap()
            }
        }
    }
}

struct GpioChipListener {
    cancel_token: CancellationToken,
    shadow: Arc<Shadow>,
    chip: Dep<dipstick_gpio::Chip>,
    /// Maps from pin id to signal id
    mapping: HashMap<String, String>,
}

impl GpioChipListener {
    fn new(
        cancel_token: CancellationToken,
        shadow: Arc<Shadow>,
        chip: Dep<dipstick_gpio::Chip>,
    ) -> Self {
        Self {
            cancel_token,
            shadow,
            chip,
            mapping: HashMap::new(),
        }
    }

    fn add_mapping(&mut self, pin_id: String, signal_id: String) -> Result<()> {
        if self.mapping.contains_key(&pin_id) {
            return Err(Status::invalid_argument(format!(
                "pin id '{pin_id}' mapped more than once"
            )));
        }
        self.mapping.insert(pin_id, signal_id);
        Ok(())
    }

    fn fetch_all_pins(&self) {
        for (pin_id, signal_id) in &self.mapping {
            let Some(status) = self.chip.pin_status(pin_id) else {
                continue;
            };
            self.shadow.set_signal_value(
                signal_id,
                status.changed_at.unwrap_or_default(),
                status.logical().into(),
            );
        }
    }

    async fn run(self) {
        let mut stream = self.chip.subscribe();
        self.fetch_all_pins();
        loop {
            let item = tokio::select! {
                _ = self.cancel_token.cancelled() => {
                    break;
                }
                item = stream.next() => item,
            };
            match item {
                Some(Ok(event)) => {
                    let Some(signal_id) = self.mapping.get(&event.pin_id) else {
                        continue;
                    };
                    let status = event.status.unwrap_or_default();
                    self.shadow.set_signal_value(
                        signal_id,
                        status.changed_at.unwrap_or_default(),
                        status.logical().into(),
                    );
                }
                Some(Err(BroadcastStreamRecvError::Lagged(n))) => {
                    tracing::error!(n, "gpio chip listener missed events");
                    self.fetch_all_pins();
                }
                None => unreachable!(),
            }
        }
    }
}

struct A2lListener {
    cancel_token: CancellationToken,
    shadow: Arc<Shadow>,
    session: Dep<dipstick_xcp::Session>,
    interval: tokio::time::Interval,
    signal_id: String,
    target: A2lTarget,
}

enum A2lTarget {
    Characteristic(dipstick_proto::xcp::v1::A2lFullCharacteristic),
    Measurement(dipstick_proto::xcp::v1::A2lMeasurement),
}

impl A2lListener {
    fn new_characteristic(
        cancel_token: CancellationToken,
        shadow: Arc<Shadow>,
        core: &Core,
        signal_id: String,
        spec: &mut A2lCharacteristicSignalSpec,
    ) -> Result<Self> {
        let interval = Self::prepare_interval(spec.poll_interval.unwrap_or_default())?;
        let a2l = core.select_entity::<dipstick_xcp::A2l>(spec.a2l.clone().unwrap_or_default())?;
        let target = a2l
            .get_characteristic(&spec.characteristic_name)
            .map(A2lTarget::Characteristic)?;

        let session = core.select_entity_dep::<dipstick_xcp::Session>(
            &shadow,
            spec.session.clone().unwrap_or_default(),
        )?;
        Ok(Self {
            cancel_token,
            shadow,
            session,
            interval,
            signal_id,
            target,
        })
    }

    fn new_measurement(
        cancel_token: CancellationToken,
        shadow: Arc<Shadow>,
        core: &Core,
        signal_id: String,
        spec: &mut A2lMeasurementSignalSpec,
    ) -> Result<Self> {
        let interval = Self::prepare_interval(spec.poll_interval.unwrap_or_default())?;
        let a2l = core.select_entity::<dipstick_xcp::A2l>(spec.a2l.clone().unwrap_or_default())?;
        let target = a2l
            .get_measurement(&spec.measurement_name)
            .map(A2lTarget::Measurement)?;

        let session = core.select_entity_dep::<dipstick_xcp::Session>(
            &shadow,
            spec.session.clone().unwrap_or_default(),
        )?;
        Ok(Self {
            cancel_token,
            shadow,
            session,
            interval,
            signal_id,
            target,
        })
    }

    fn prepare_interval(
        poll_interval: dipstick_proto::wkt::Duration,
    ) -> Result<tokio::time::Interval> {
        let period = Duration::try_from(poll_interval)
            .map_err(|err| Status::invalid_argument(format!("invalid poll interval: {err}")))?;
        if period.is_zero() {
            return Err(Status::invalid_argument("poll interval must be non-zero"));
        }
        let mut interval = tokio::time::interval(period);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        Ok(interval)
    }

    async fn run(mut self) {
        loop {
            tokio::select! {
                _ = self.cancel_token.cancelled() => break,
                _ = self.interval.tick() => {}
            }
            let (timestamp, value) = match self.read_target().await {
                Ok(v) => v,
                Err(err) => {
                    // TODO error counter?
                    tracing::error!(
                        err = &err as &dyn std::error::Error,
                        "failed to read a2l target"
                    );
                    continue;
                }
            };
            self.shadow
                .set_signal_value(&self.signal_id, timestamp, value);
        }
    }

    async fn read_target(
        &self,
    ) -> Result<(dipstick_proto::wkt::Timestamp, dipstick_proto::wkt::Value)> {
        match &self.target {
            A2lTarget::Characteristic(characteristic) => {
                self.session.read_characteristic(characteristic).await
            }
            A2lTarget::Measurement(measurement) => self.session.read_measurement(measurement).await,
        }
    }
}

struct DeviceListener {
    cancel_token: CancellationToken,
    shadow: Arc<Shadow>,
    device: Dep<dipstick_device::Device>,
    /// Maps from sensor to signal id
    sensor_mapping: HashMap<String, String>,
}

impl DeviceListener {
    fn new(
        cancel_token: CancellationToken,
        shadow: Arc<Shadow>,
        device: Dep<dipstick_device::Device>,
    ) -> Self {
        Self {
            cancel_token,
            shadow,
            device,
            sensor_mapping: HashMap::new(),
        }
    }

    fn add_sensor_mapping(&mut self, sensor: String, signal_id: String) {
        self.sensor_mapping.insert(sensor, signal_id);
    }

    async fn handle_event(&mut self, event: dipstick_proto::device::v1::DeviceEvent) {
        use dipstick_proto::device::v1::DeviceEventVariant;
        if let Some(DeviceEventVariant::Sensor(event)) = event.device_event_variant {
            let signal_id = match self.sensor_mapping.get(&event.sensor) {
                Some(signal_id) => signal_id,
                None => return,
            };
            self.shadow.set_signal_value(
                signal_id,
                event.timestamp.unwrap_or_default(),
                event.value.unwrap_or_default(),
            );
        }
    }

    fn fetch_all_sensor_values(&self) {
        for (sensor, signal_id) in &self.sensor_mapping {
            let Some(status) = self.device.sensor_status(sensor) else {
                continue;
            };
            self.shadow.set_signal_value(
                signal_id,
                status.timestamp.unwrap_or_default(),
                status.value.unwrap_or_default(),
            );
        }
    }

    async fn run(mut self) {
        let mut stream = self.device.subscribe();
        self.fetch_all_sensor_values();
        loop {
            let item = tokio::select! {
                _ = self.cancel_token.cancelled() => break,
                item = stream.next() => item,
            };
            match item {
                Some(Ok(event)) => self.handle_event(event).await,
                Some(Err(BroadcastStreamRecvError::Lagged(n))) => {
                    tracing::error!(n, "device sensor listener missed events");
                    self.fetch_all_sensor_values();
                }
                None => unreachable!(),
            }
        }
    }
}
