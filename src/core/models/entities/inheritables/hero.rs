use std::any::Any;
use std::fmt;

use contracts::models::{ICard, IEntity, IEntityCastable, IEntityData, IEntityInitializable};

use core::Game;
use core::models::entities::{Controller, EntityData, Weapon};

use enums::{ECardTypes, EGameTags};
use enums::contracted::{EntityCastError, EntityCreationError};

#[derive(Debug)]
pub struct Hero {
    data: EntityData,
    card: &'static ICard,
    weapon_id: Option<u32>,
}

impl fmt::Display for Hero {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "HERO [TODO]")
    }
}

impl Hero {
    pub fn weapon<'a>(&self, game: &'a Game) -> Option<&'a Weapon> {
        self.weapon_id.map(|id| Weapon::try_into(game.get_entity(id).unwrap()).unwrap())
    }

    pub fn set_weapon(&mut self, game: &Game, weapon: &Weapon) -> Result<(), bool> {
        // Should we test for something before storing the weapon
        // id?
        self.weapon_id = Some(weapon.id());
        Ok(())
    }

    pub fn remove_weapon(&mut self, game: &Game) -> Result<(), bool> {
        // Should we also move the weapon into the graveyard +
        // update all tags?
        match self.weapon_id {
            Some(_) => self.weapon_id.take(), // Replace integer, if set, with none.
            None => return Err(false),
        };

        Ok(())
    }

    pub fn controller<'a>(&self, game: &'a Game) -> &'a Controller {
        let controller_entity = game.get_entity(self.controller_id()).expect("Bogus controller ID was set");
        Controller::try_into(controller_entity).unwrap()
    }
}


impl IEntity for Hero {
    fn reference_card(&self) -> &'static ICard {
        self.card
    }

    fn _get_data_internal(&self) -> &IEntityData {
        &self.data
    }

    fn tag_value(&self, tag: EGameTags) -> u32 {
        let tag_value = self.native_tag_value(tag); // MUT
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

impl IEntityInitializable for Hero {
    fn new(entity_id: u32, card: &'static ICard) -> Result<Box<IEntity>, EntityCreationError> {
        let entity_data = try!(EntityData::from_data(entity_id, card._get_data_internal()).map_err(|x| {
            EntityCreationError::InvalidEntityData(x)
        }));

        let obj = Self {
            data: entity_data,
            card: card,
            weapon_id: None,
        };

        // Box the object and return it to caller.
        Ok(Box::new(obj))
    }
}

impl IEntityCastable for Hero {
    fn try_into<'a>(e: &'a IEntity) -> Result<&'a Self, EntityCastError> {
        cast_entity!(e, (ECardTypes::Hero), Hero)
    }

    fn try_into_mut<'a>(e: &'a mut IEntity) -> Result<&'a mut Self, EntityCastError> {
        cast_entity_mut!(e, (ECardTypes::Hero), Hero)
    }
}
