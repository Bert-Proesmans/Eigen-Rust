use std::any::Any;
use std::fmt;

use contracts::models::{ICard, IEntity, IEntityCastable, IEntityData, IEntityInitializable};

use core::models::entities::{Controller, EntityData};

use enums::{ECardTypes, EGameTags};
use enums::contracted::{EntityCastError, EntityCreationError};

#[derive(Debug)]
pub struct Weapon {
    data: EntityData,
    card: &'static ICard,
}

impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "WEAPON [TODO]")
    }
}

impl Weapon {
    // add code here
}

impl IEntity for Weapon {
    fn reference_card(&self) -> &'static ICard {
        self.card
    }

    fn _get_data_internal(&self) -> &IEntityData {
        &self.data
    }

    fn tag_value(&self, tag: EGameTags) -> u32 {
        let mut tag_value = self.native_tag_value(tag);
        // TODO; process all aura's and other stuff which influence
        // this tag.

        tag_value
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

impl IEntityInitializable for Weapon {
    fn new(entity_id: u32, card: &'static ICard) -> Result<Box<IEntity>, EntityCreationError> {
        let entity_data = try!(EntityData::from_data(entity_id, card._get_data_internal()).map_err(|x| {
            EntityCreationError::InvalidEntityData(x)
        }));

        let obj = Self {
            data: entity_data,
            card: card,
        };

        // Box the object and return it to caller.
        Ok(Box::new(obj))
    }
}

impl IEntityCastable for Weapon {
    fn try_into<'a>(e: &'a IEntity) -> Result<&'a Self, EntityCastError> {
        cast_entity!(e, (ECardTypes::Weapon), Weapon)
    }

    fn try_into_mut<'a>(e: &'a mut IEntity) -> Result<&'a mut Self, EntityCastError> {
        cast_entity_mut!(e, (ECardTypes::Weapon), Weapon)
    }
}
