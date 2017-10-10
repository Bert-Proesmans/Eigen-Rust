use std::any::Any;
use std::fmt;

use contracts::cards::card::ICard;
use contracts::entities::character::ICharacter;
use contracts::entities::entity::IEntity;
use contracts::entities::entity_castable::IEntityCastable;
use contracts::entities::entity_data::IEntityData;
use contracts::entities::playable::IPlayable;

use contracts::entities::entity_castable::errors as CastError;
use contracts::entities::errors as EntityError;

use cards::card_container::CONTROLLER_CARD;
use game::entities::entity_data::EntityData;

use enums::{ECardTypes, EGameTags};

#[derive(Debug)]
pub struct Controller<'controller> {
    data: EntityData,
    card: &'controller (ICard + 'controller),
    name: &'controller str,

    // TODO
    zones: u32,
    choice: Option<u32>
}

impl<'cx> fmt::Display for Controller<'cx> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "CONTROLLER [TODO]")
    }
}

impl<'cx> Controller<'cx> {
    pub fn new(
        id: u32,
        name: &'cx str,
    ) -> EntityError::Result<Self> {

        let controller_entity_data: EntityError::Result<_> = EntityData::from_data(id, &CONTROLLER_CARD.card_data)
            .map_err(|e| e.into());
        let controller_entity_data = try!(controller_entity_data);

        Ok(Self {
            data: controller_entity_data,
            card: &*CONTROLLER_CARD, // Coerce 'static into 'cx
            name: name,

            zones: 0,
            choice: None
        })
    }
}

impl<'cx> IEntity<'cx> for Controller<'cx> {
    fn reference_card(&self) -> &(ICard + 'cx) {
        self.card
    }

    fn _get_data_internal(&self) -> &(IEntityData + 'cx) {
        &self.data
    }

    fn _get_data_internal_mut(&mut self) -> &mut (IEntityData + 'cx) {
        &mut self.data
    }

    fn tag_value(
        &self,
        tag: EGameTags,
    ) -> u32 {
        let mut tag_value = self.native_tag_value(tag);
        // TODO; process all aura's and other stuff which influence
        // this tag.

        tag_value
    }

    fn set_tag_value(
        &mut self,
        _tag: EGameTags,
        _value: u32,
    ) -> Option<u32> {
        None
    }

    fn as_any(&self) -> &(Any + 'cx) {
        self
    }

    fn as_playable(&self) -> Option<&IPlayable<'cx>> {
        None
    }

    fn as_character(&self) -> Option<&ICharacter<'cx>> {
        None
    }

    fn as_any_mut(&mut self) -> &mut (Any + 'cx) {
        self
    }

    fn as_playable_mut(&mut self) -> Option<&mut IPlayable<'cx>> {
        None
    }

    fn as_character_mut(&mut self) -> Option<&mut ICharacter<'cx>> {
        None
    }
}

impl<'cx> IEntityCastable for Controller<'cx> {
    fn try_into<'e>(_e: &'e IEntity) -> CastError::Result<&'e Self> {
        // cast_entity!(e, (ECardTypes::Player), Controller)
        bail!(CastError::ErrorKind::NoCastProvided)
    }

    fn try_into_mut<'e>(_e: &'e mut IEntity) -> CastError::Result<&'e mut Self> {
        // cast_entity_mut!(e, (ECardTypes::Player), Controller)
        bail!(CastError::ErrorKind::NoCastProvided)
    }
}
