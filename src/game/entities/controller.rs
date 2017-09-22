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
use game::entities::inheritables::hero::Hero;

use enums::{ECardTypes, EGameTags};

#[derive(Debug)]
pub struct Controller {
    data: EntityData,
    card: &'static ICard,
    name: &'static str,

    // TODO
    zones: u32,
    choice: Option<u32>
}

impl fmt::Display for Controller {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "CONTROLLER [TODO]")
    }
}

impl Controller {
    pub fn new(
        id: u32,
        name: &'static str,
    ) -> EntityError::Result<Self> {

        let controller_entity_data: EntityError::Result<_> = EntityData::from_data(id, &CONTROLLER_CARD.card_data)
            .map_err(|e| e.into());
        let controller_entity_data = try!(controller_entity_data);

        Ok(Self {
            data: controller_entity_data,
            card: &*CONTROLLER_CARD as &ICard,
            name: name,

            zones: 0,
            choice: None
        })
    }
}

impl IEntity for Controller {
    fn reference_card(&self) -> &'static ICard {
        self.card
    }

    fn _get_data_internal(&self) -> &IEntityData {
        &self.data
    }

    fn _get_data_internal_mut(&mut self) -> &mut IEntityData {
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

    fn as_any<'e>(&'e self) -> &'e Any {
        self
    }

    fn as_playable<'e>(&'e self) -> Option<&'e IPlayable> {
        None
    }

    fn as_character<'e>(&'e self) -> Option<&'e ICharacter> {
        None
    }

    fn as_any_mut<'e>(&'e mut self) -> &'e mut Any {
        self
    }

    fn as_playable_mut<'e>(&'e mut self) -> Option<&'e mut IPlayable> {
        None
    }

    fn as_character_mut<'e>(&'e mut self) -> Option<&'e mut ICharacter> {
        None
    }
}

impl IEntityCastable for Controller {
    fn try_into<'e>(_e: &'e IEntity) -> CastError::Result<&'e Self> {
        // cast_entity!(e, (ECardTypes::Player), Controller)
        bail!(CastError::ErrorKind::NoCastProvided)
    }

    fn try_into_mut<'e>(_e: &'e mut IEntity) -> CastError::Result<&'e mut Self> {
        // cast_entity_mut!(e, (ECardTypes::Player), Controller)
        bail!(CastError::ErrorKind::NoCastProvided)
    }
}
