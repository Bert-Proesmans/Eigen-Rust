use std::any::Any;
use std::fmt;

use contracts::models::{ICard, ICharacter, IEntity, IEntityCastable, IEntityData, IEntityInitializable, IPlayable};

use core::models::entities::{Controller, EntityData};

use enums::{ECardTypes, EGameTags};
use enums::contracted::{EntityCastError, EntityCreationError};

#[derive(Debug)]
pub struct HeroPower {
    data: EntityData,
    card: &'static ICard
}

impl fmt::Display for HeroPower {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> Result<(), fmt::Error> {
        write!(f, "HERO_POWER [TODO]")
    }
}

impl HeroPower {
    // add code here
}

impl<'e> IEntity<'e> for HeroPower {
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
        Some(self)
    }

    fn as_character(&'e self) -> Option<&'e ICharacter> {
        None
    }

    fn as_any_mut(&'e mut self) -> &'e mut Any {
        self
    }

    fn as_playable_mut(&'e mut self) -> Option<&'e mut IPlayable> {
        Some(self)
    }

    fn as_character_mut(&'e mut self) -> Option<&'e mut ICharacter> {
        None
    }
}

impl IEntityInitializable for HeroPower {
    fn new(
        entity_id: u32,
        card: &'static ICard,
    ) -> Result<Box<IEntity>, EntityCreationError> {
        let entity_data = try!(EntityData::from_data(entity_id, card._get_data_internal()).map_err(|x| {
            EntityCreationError::InvalidEntityData(x)
        }));

        let obj = Self {
            data: entity_data,
            card: card
        };

        // Box the object and return it to caller.
        Ok(Box::new(obj))
    }
}

impl IEntityCastable for HeroPower {
    fn try_into<'e>(e: &'e IEntity) -> Result<&'e Self, EntityCastError> {
        // cast_entity!(e, (ECardTypes::HeroPower), HeroPower)
        Err(EntityCastError::NoCastProvided)
    }

    fn try_into_mut<'e>(e: &'e mut IEntity) -> Result<&'e mut Self, EntityCastError> {
        // cast_entity_mut!(e, (ECardTypes::HeroPower), HeroPower)
        Err(EntityCastError::NoCastProvided)
    }
}

impl IPlayable for HeroPower {
    // add code here
}
