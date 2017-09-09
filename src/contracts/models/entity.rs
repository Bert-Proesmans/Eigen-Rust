use std::any::Any;
use std::fmt::{Debug, Display};

use num_traits::FromPrimitive;

use contracts::models::{ICard, ICharacter, IEntityData, IPlayable};
use enums::{EGameTags, EZones};
use enums::contracted::{EntityCastError, EntityCreationError};


pub trait IEntity<'e>: Debug + Display {
    // There is no direct reference to the game which holds
    // this entity!

    fn reference_card(&'e self) -> &'static ICard;
    fn _get_data_internal(&'e self) -> &'e IEntityData;
    fn tag_value(
        &'e self,
        tag: EGameTags,
    ) -> u32;

    ////////////////////////
    // Conversion methods //
    // /////////////////////
    fn as_any(&'e self) -> &'e Any;
    fn as_playable(&'e self) -> Option<&'e IPlayable>;
    fn as_character(&'e self) -> Option<&'e ICharacter>;

    fn as_any_mut(&'e mut self) -> &'e mut Any;
    fn as_playable_mut(&'e mut self) -> Option<&'e mut IPlayable>;
    fn as_character_mut(&'e mut self) -> Option<&'e mut ICharacter>;

    ////////////////
    // Properties //
    // /////////////

    fn id(&'e self) -> u32 {
        self._get_data_internal().id()
    }

    fn native_tag_value(
        &'e self,
        tag: EGameTags,
    ) -> u32 {
        // Default to 0-value if the tag wasn't present.
        self._get_data_internal().get_tag(tag).unwrap_or(0)
    }

    fn set_native_tag_value(
        &'e mut self,
        tag: EGameTags,
        val: u32,
    ) -> Option<u32> {
        self._get_data_internal().set_tag(tag, val)
    }

    ///////////////////////////////////////////////////////////////
    // Most used tags on each entity have their shortcuts
    // below. //
    // ////////////////////////////////////////////////////////////

    fn zone_id(&'e self) -> Option<EZones> {
        EZones::from_u32(self.native_tag_value(EGameTags::Zone))
    }

    fn controller_id(&'e self) -> u32 {
        self.native_tag_value(EGameTags::Controller)
    }
}

// Kinda similar to the nightly trait TryFrom.
// This trait is used to downcast IEntities into their
// struct type.
pub trait IEntityCastable: Debug + Display {
    fn try_into<'e>(e: &'e IEntity) -> Result<&'e Self, EntityCastError>;
    fn try_into_mut<'e>(e: &'e mut IEntity) -> Result<&'e mut Self, EntityCastError>;
}

pub trait IEntityInitializable: Debug + Display {
    fn new(
        entity_id: u32,
        &'static ICard,
    ) -> Result<Box<IEntity>, EntityCreationError>;
}
