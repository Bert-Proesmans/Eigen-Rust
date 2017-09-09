use std::fmt::Debug;

use enums::EGameTags;

pub trait IEntityData: Debug {
    fn id(&self) -> u32;

    fn set_tag(
        &self,
        tag: EGameTags,
        value: u32,
    ) -> Option<u32>;

    fn get_tag(
        &self,
        tag: EGameTags,
    ) -> Option<u32>;
}
