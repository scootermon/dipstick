// use dipstick_proto::core::v1::{EntitySelector, EntitySelectorVariant};

use anyhow::Context;
use dipstick_proto::core::v1::EntitySelectorVariant;

use super::{IdContext, QualifiedId, UniqueId};

pub enum EntitySelector<'a> {
    Id(QualifiedId<'a>),
    UniqueId(UniqueId),
}

impl<'a> EntitySelector<'a> {
    pub fn from_proto(
        proto: &'a dipstick_proto::core::v1::EntitySelector,
        context: &IdContext<'a>,
    ) -> anyhow::Result<Self> {
        match &proto.entity_selector_variant {
            Some(EntitySelectorVariant::Id(id)) => {
                QualifiedId::parse_with_context(&id, context).map(Self::Id)
            }
            Some(EntitySelectorVariant::UniqueId(id)) => UniqueId::new(*id)
                .context("unique id must not be 0")
                .map(Self::UniqueId),
            None => anyhow::bail!("selector variant unspecified"),
        }
    }
}
