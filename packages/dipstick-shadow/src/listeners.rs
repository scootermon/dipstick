use std::collections::HashMap;
use std::sync::Arc;

use dipstick_core::{Core, DependencyHandle, Entity};
use dipstick_proto::shadow::v1::GpioSignalSpec;
use futures::StreamExt;
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
    pub fn builder<'a>(core: &'a Core, shadow: Arc<Shadow>) -> ListenersBuilder<'a> {
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
}

impl<'a> ListenersBuilder<'a> {
    pub fn new(core: &'a Core, shadow: Arc<Shadow>) -> Self {
        Self {
            core,
            cancel_token: CancellationToken::new(),
            shadow,
            gpio_listeners: Vec::new(),
        }
    }

    pub fn build(self) -> Listeners {
        let tracker = TaskTracker::new();
        for listener in self.gpio_listeners {
            tracker.spawn(listener.run());
        }
        Listeners {
            drop_guard: self.cancel_token.drop_guard(),
            tracker,
        }
    }

    pub fn add_gpio_signal(&mut self, signal_id: String, spec: &mut GpioSignalSpec) -> Result<()> {
        let selector = spec.chip.clone().unwrap_or_default();
        let chip = self.core.select_entity::<dipstick_gpio::Chip>(&selector)?;
        let listener = self.get_gpio_chip_listener(chip);
        // TODO: check if chip has pin
        listener.add_mapping(spec.pin.clone(), signal_id)?;
        Ok(())
    }

    fn get_gpio_chip_listener(&mut self, chip: Arc<dipstick_gpio::Chip>) -> &mut GpioChipListener {
        let index = self
            .gpio_listeners
            .iter()
            .position(|listener| Arc::ptr_eq(&listener.chip, &chip));
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
}

struct GpioChipListener {
    cancel_token: CancellationToken,
    shadow: Arc<Shadow>,
    chip: Arc<dipstick_gpio::Chip>,
    _dependency_handle: DependencyHandle,
    /// Maps from pin id to signal id
    mapping: HashMap<String, String>,
}

impl GpioChipListener {
    fn new(
        cancel_token: CancellationToken,
        shadow: Arc<Shadow>,
        chip: Arc<dipstick_gpio::Chip>,
    ) -> Self {
        let dependency_handle = shadow.entity_meta().add_dependency(chip.entity_meta());
        Self {
            cancel_token,
            shadow,
            chip,
            _dependency_handle: dependency_handle,
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

    async fn run(self) {
        // TODO: add option for subscribe to publish current values on subscribe
        let mut stream = self.chip.subscribe(); // TODO: only subscribe to the pins we care about
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
                _ => todo!(), // TODO
            }
        }
    }
}
