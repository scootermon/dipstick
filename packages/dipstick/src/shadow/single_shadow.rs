use super::signal::Signal;
use crate::core::registry::Registry;

pub struct SingleShadow {
    signals: Registry<Signal>,
}
