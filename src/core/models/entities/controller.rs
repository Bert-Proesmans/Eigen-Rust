use std::any::Any;
use std::fmt;

use contracts::models::{ICard, ICharacter, IEntity, IEntityCastable, IEntityData, IPlayable};

use core::models::cardcontainer::CONTROLLER_CARD;
use core::models::entities::EntityData;

use enums::{ECardTypes, EGameTags};
use enums::contracted::{EntityCastError, EntityCreationError};

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
    ) -> Result<(), fmt::Error> {
        write!(f, "CONTROLLER [TODO]")
    }
}

impl Controller {
    pub fn new(
        id: u32,
        name: &'static str,
    ) -> Result<Self, EntityCreationError> {

        let controller_entity_data = try!(EntityData::from_data(id, &CONTROLLER_CARD.card_data).map_err(|x| {
            EntityCreationError::InvalidEntityData(x)
        }));

        Ok(Self {
            data: controller_entity_data,
            card: &*CONTROLLER_CARD as &ICard,
            name: name,

            zones: 0,
            choice: None
        })
    }
}

impl<'e> IEntity<'e> for Controller {
    fn reference_card(&self) -> &'static ICard {
        self.card
    }

    fn _get_data_internal(&self) -> &IEntityData {
        &self.data
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

    fn as_any(&'e self) -> &'e Any {
        self
    }

    fn as_playable(&'e self) -> Option<&'e IPlayable> {
        None
    }

    fn as_character(&'e self) -> Option<&'e ICharacter> {
        None
    }

    fn as_any_mut(&'e mut self) -> &'e mut Any {
        self
    }

    fn as_playable_mut(&'e mut self) -> Option<&'e mut IPlayable> {
        None
    }

    fn as_character_mut(&'e mut self) -> Option<&'e mut ICharacter> {
        None
    }
}

impl IEntityCastable for Controller {
    fn try_into<'e>(e: &'e IEntity) -> Result<&'e Self, EntityCastError> {
        // cast_entity!(e, (ECardTypes::Player), Controller)
        Err(EntityCastError::NoCastProvided)
    }

    fn try_into_mut<'e>(e: &'e mut IEntity) -> Result<&'e mut Self, EntityCastError> {
        // cast_entity_mut!(e, (ECardTypes::Player), Controller)
        Err(EntityCastError::NoCastProvided)
    }
}
