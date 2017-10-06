use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::hash;

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

/// Represents the internal state of an IEntity
pub trait IEntityData: fmt::Debug + fmt::Display {
    /// Returns the Entity ID defined by this state
    fn id(&self) -> u32;

    /// Store the provided value inside this state for the
    /// provided tag
    fn set_tag(
        &mut self,
        tag: EGameTags,
        value: u32,
    ) -> Option<u32>;

    /// Retrieves the value for the provided tag from this
    /// state
    fn get_tag(
        &self,
        tag: EGameTags,
    ) -> Option<u32>;
}

impl<'dx> hash::Hash for IEntityData + 'dx {
    fn hash<H: hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self.id().hash(state);
    }
}

impl<'dx> cmp::PartialEq for IEntityData + 'dx {
    fn eq(
        &self,
        other: &IEntityData,
    ) -> bool {
        self.id() == other.id()
    }
}

impl<'dx> cmp::Eq for IEntityData + 'dx {}
