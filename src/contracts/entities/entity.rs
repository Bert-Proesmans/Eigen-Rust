use std::any::Any;
use std::cmp;
use std::fmt;
use std::hash;

use num_traits::FromPrimitive;

use contracts::cards::card::ICard;
use contracts::entities::character::ICharacter;
use contracts::entities::entity_data::IEntityData;
use contracts::entities::playable::IPlayable;

use enums::{EGameTags, EZones};

/// Game object which holds a state, which can be
/// manipulated during the game
pub trait IEntity<'entity>: fmt::Debug + fmt::Display {
    // There is no direct reference to the game which holds
    // this entity!

    /// The card used to build this entity object
    fn reference_card(&self) -> &(ICard + 'static);

    /// Returns a borrow of the internal state of this
    /// object
    fn _get_data_internal(&self) -> &(IEntityData + 'entity);

    /// Returns a mutable borrow of the internal state of
    /// this object
    fn _get_data_internal_mut(&mut self) -> &mut (IEntityData + 'entity);

    /// Returns the value of the provided tag
    ///
    /// The value is extracted from the internal state of
    /// this entity object.
    /// Afterwards this value is subjected to various buffs
    /// applied to this entity
    /// or "parent" containers.
    ///
    /// See `self.native_tag_value()` for a value not
    /// influenced by these buffs!
    fn tag_value(
        &self,
        tag: EGameTags,
    ) -> u32;

    /// Sets the value for the provided tag, returning the
    /// old value if present
    ///
    /// The value will be stored into the internal state.
    /// After this happens
    /// any attached triggers will be invoked to notify the
    /// tag value change.
    fn set_tag_value(
        &mut self,
        tag: EGameTags,
        value: u32,
    ) -> Option<u32>;

    ////////////////////////
    // Conversion methods //
    // /////////////////////

    /// Method used for downcasting to actual struct object
    fn as_any(&self) -> &(Any + 'entity);

    /// Return this entity as an IPlayable reference
    fn as_playable(&self) -> Option<&(IPlayable + 'entity)>;

    /// Return this entity as an ICharacter reference
    fn as_character(&self) -> Option<&(ICharacter + 'entity)>;

    /// Method used for mutably downcasting to actual
    /// struct object
    fn as_any_mut(&mut self) -> &mut (Any + 'entity);

    /// Return this entity as a mutable IPlayable reference
    fn as_playable_mut(&mut self) -> Option<&mut (IPlayable + 'entity)>;

    /// Return this entity as a mutable ICharacter reference
    fn as_character_mut(&mut self) -> Option<&mut (ICharacter + 'entity)>;

    ////////////////
    // Properties //
    // /////////////

    /// Returns the ID of this entity (= Entity ID)
    fn id(&self) -> u32 {
        self._get_data_internal().id()
    }

    /// Returns the native value for the provided tag
    ///
    /// The returned value comes directly from the internal
    /// state.
    fn native_tag_value(
        &self,
        tag: EGameTags,
    ) -> u32 {
        // Default to 0-value if the tag wasn't present.
        self._get_data_internal().get_tag(tag).unwrap_or(0)
    }

    /// Sets a native value for the provided tag, returning
    /// the old value if present
    ///
    /// The value goes directly into the internal state.
    fn set_native_tag_value(
        &mut self,
        tag: EGameTags,
        val: u32,
    ) -> Option<u32> {
        self._get_data_internal_mut().set_tag(tag, val)
    }

    ///////////////////////////////////////////////////////////////
    // Most used tags on each entity have their shortcuts
    // below.
    // ////////////////////////////////////////////////////////////

    /// Returns the zone to which this entity belongs
    fn zone_id(&self) -> Option<EZones> {
        EZones::from_u32(self.native_tag_value(EGameTags::Zone))
    }

    /// Returns the Entity ID of the controller
    ///
    /// The controller is the entity object which is in a
    /// sense the parent of this
    /// entity.
    fn controller_id(&self) -> u32 {
        self.native_tag_value(EGameTags::Controller)
    }
}

impl<'ex> hash::Hash for IEntity<'ex> {
    fn hash<H: hash::Hasher>(
        &self,
        state: &mut H,
    ) {
        self._get_data_internal().hash(state);
    }
}

impl<'ex> cmp::PartialEq for IEntity<'ex> {
    fn eq(
        &self,
        other: &IEntity<'ex>,
    ) -> bool {
        self._get_data_internal() == other._get_data_internal()
    }
}

impl<'ex> cmp::Eq for IEntity<'ex> {}
