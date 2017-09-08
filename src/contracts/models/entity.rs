use std::any::Any;
use std::fmt::{Debug, Display};

use num_traits::FromPrimitive;

use contracts::models::{ICard, IEntityData};
use enums::{EGameTags, EZones};
use enums::contracted::{EntityCastError, EntityCreationError};


pub trait IEntity: Debug + Display {
    // There is no direct reference to the game which holds
    // this entity!

    fn reference_card(&self) -> &'static ICard;
    fn _get_data_internal<'a>(&'a self) -> &'a IEntityData;
    fn tag_value(&self, tag: EGameTags) -> u32;
    fn as_any<'a>(&'a self) -> &'a Any;
    fn as_any_mut<'a>(&'a mut self) -> &'a mut Any;

    fn id(&self) -> u32 {
        self._get_data_internal().id()
    }

    fn native_tag_value(&self, tag: EGameTags) -> u32 {
        // Default to 0-value if the tag wasn't present.
        self._get_data_internal().get_tag(tag).unwrap_or(0)
    }

    fn set_native_tag_value(&mut self, tag: EGameTags, val: u32) -> Option<u32> {
        self._get_data_internal().set_tag(tag, val)
    }

    ///////////////////////////////////////////////////////////////
    // Most used tags on each entity have their shortcuts
    // below. //
    // ////////////////////////////////////////////////////////////

    fn zone_id(&self) -> Option<EZones> {
        EZones::from_u32(self.native_tag_value(EGameTags::Zone))
    }

    fn controller_id(&self) -> u32 {
        self.native_tag_value(EGameTags::Controller)
    }
}

// Kinda similar to the nightly trait TryFrom.
// DON'T use the downcast method in cooperation with the
// Any type!
pub trait IEntityCastable: Debug + Display {
    fn try_into<'a>(e: &'a IEntity) -> Result<&'a Self, EntityCastError>;
    fn try_into_mut<'a>(e: &'a mut IEntity) -> Result<&'a mut Self, EntityCastError>;
}

pub trait IEntityInitializable: Debug + Display {
    fn new(entity_id: u32, &'static ICard) -> Result<Box<IEntity>, EntityCreationError>;
}
