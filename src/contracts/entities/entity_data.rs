use std::fmt::Debug;

use enums::EGameTags;

pub mod errors {
    error_chain! {
        errors {
            FailedCreation {
                display("Couldn't build EntityData object")
            }
        }
    }
}

pub trait IEntityData: Debug {
    fn id(&self) -> u32;

    fn set_tag(
        &mut self,
        tag: EGameTags,
        value: u32,
    ) -> Option<u32>;

    fn get_tag(
        &self,
        tag: EGameTags,
    ) -> Option<u32>;
}
