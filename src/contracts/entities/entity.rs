use std::any::Any;
use std::fmt::{Debug, Display};

use num_traits::FromPrimitive;

use contracts::cards::card::ICard;
use contracts::entities::character::ICharacter;
use contracts::entities::entity_data::IEntityData;
use contracts::entities::playable::IPlayable;

use enums::{EGameTags, EZones};

pub trait IEntity: Debug + Display {
    // There is no direct reference to the game which holds
    // this entity!

    fn reference_card(&self) -> &'static ICard;
    fn _get_data_internal<'e>(&'e self) -> &'e IEntityData;
    fn _get_data_internal_mut<'e>(&'e mut self) -> &'e mut IEntityData;
    fn tag_value(
        &self,
        tag: EGameTags,
    ) -> u32;

    ////////////////////////
    // Conversion methods //
    // /////////////////////
    fn as_any<'e>(&'e self) -> &'e Any;
    fn as_playable<'e>(&'e self) -> Option<&'e IPlayable>;
    fn as_character<'e>(&'e self) -> Option<&'e ICharacter>;

    fn as_any_mut<'e>(&'e mut self) -> &'e mut Any;
    fn as_playable_mut<'e>(&'e mut self) -> Option<&'e mut IPlayable>;
    fn as_character_mut<'e>(&'e mut self) -> Option<&'e mut ICharacter>;

    ////////////////
    // Properties //
    // /////////////

    fn id(&self) -> u32 {
        self._get_data_internal().id()
    }

    fn native_tag_value(
        &self,
        tag: EGameTags,
    ) -> u32 {
        // Default to 0-value if the tag wasn't present.
        self._get_data_internal().get_tag(tag).unwrap_or(0)
    }

    fn set_native_tag_value(
        &mut self,
        tag: EGameTags,
        val: u32,
    ) -> Option<u32> {
        self._get_data_internal_mut().set_tag(tag, val)
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
