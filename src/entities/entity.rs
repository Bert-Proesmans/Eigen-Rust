use std::fmt;

use entities::entity_data::EntityData;
use game_tags::EGameTags;

pub trait IEntity<'machine>: fmt::Debug {
    fn _get_data_internal(&self) -> &EntityData;
    fn _get_data_internal_mut(&mut self) -> &mut EntityData;

    fn tag_value(
        &self,
        tag: EGameTags,
    ) -> u32 {
        self._get_data_internal().get_tag(tag)
    }

    fn set_tag_value(
        &mut self,
        tag: EGameTags,
        value: u32,
    ) -> Option<u32> {
        self._get_data_internal_mut().set_tag(tag, value)
    }
}
