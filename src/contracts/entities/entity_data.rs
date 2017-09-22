use std::fmt;
use std::collections::HashMap;
use std::hash;
use std::cmp;

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

impl hash::Hash for IEntityData {
    fn hash<H: hash::Hasher>(&self, state:&mut H) {
        self.id().hash(state);
    }
}

impl cmp::PartialEq for IEntityData {
    fn eq(&self, other: &IEntityData) -> bool {
        self.id() == other.id()
    }
}

impl cmp::Eq for IEntityData {}
