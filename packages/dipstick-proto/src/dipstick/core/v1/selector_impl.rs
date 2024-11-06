use std::fmt::Display;

use super::{EntitySelector, EntitySelectorVariant};

impl EntitySelector {
    pub const NONE: Self = Self {
        entity_selector_variant: None,
    };
}

impl Display for EntitySelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.entity_selector_variant {
            Some(v) => v.fmt(f),
            None => f.write_str("NONE"),
        }
    }
}

impl Display for EntitySelectorVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UniqueId(v) => v.fmt(f),
            Self::Key(v) => v.fmt(f),
        }
    }
}
