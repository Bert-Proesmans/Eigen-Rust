use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;

use contracts::models::{ICard, ICardContainer, IEntity, IEntityCastable, IEntityData, IEntityInitializable};

use core::{Controller, GameConfig};
use core::models::cardcontainer::{CARDS, GAME_CARD};
use core::models::entities::{EntityData, Hero, HeroPower};
use core::models::gameconfig::MAX_PLAYERS;

use enums::{EGameTags, EZones};
use enums::contracted::EntityCreationError;
use enums::internal::errors::GameCreationError;

// The Game itself is also an entity. The ID of this entity
// is ALWAYS 1 (1-indexed).
const GAME_ENTITY_ID: u32 = 1;

#[derive(Debug)]
pub struct Game {
    data: EntityData,
    card: &'static ICard,
    config: GameConfig,

    entities: HashMap<u32, Box<IEntity>>, // All entities except the game itself

    next_eid: u32,
    next_oop: u32,
}

impl<'a> fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "GAME [TODO]")
    }
}

impl IEntity for Game {
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
        panic!("Game struct does NOT support downcasting from the IEntity trait object");
    }

    fn as_any_mut(&mut self) -> &mut Any {
        panic!("Game struct does NOT support downcasting from the IEntity trait object");
    }
}

impl Game {
    fn next_eid(&mut self) -> u32 {
        let val = self.next_eid;
        self.next_eid += 1;
        val
    }

    pub fn next_oop(&mut self) -> u32 {
        let val = self.next_oop;
        self.next_oop += 1;
        val
    }

    pub fn new(config: GameConfig) -> Result<Self, GameCreationError> {
        // Test configuration
        let validated_config = try!(config.validate().map_err(|x| GameCreationError::InvalidConfig(x)));

        // Setup object data
        let game_entity_data = try!(EntityData::from_data(GAME_ENTITY_ID, &GAME_CARD.card_data).map_err(|x| {
            GameCreationError::InvalidEntityData(x)
        }));

        // Push additional entity information for this game
        game_entity_data.set_tag(EGameTags::Zone, EZones::Play as u32);


        let next_eid = GAME_ENTITY_ID + 1;
        let next_oop = 1;

        let obj = Self {
            data: game_entity_data,
            card: &*GAME_CARD,
            config: validated_config,

            next_eid: next_eid,
            next_oop: next_oop,

            entities: hashmap!{},
        };

        let obj = try!(obj.build_controllers());
        let obj = try!(obj.build_heroes());
        let obj = try!(obj.build_hero_powers());

        Ok(obj)
    }

    fn build_controllers(mut self) -> Result<Self, GameCreationError> {
        for idx in 0..MAX_PLAYERS {
            let entity_id = self.next_eid();
            let player_idx = 1 + idx; // Player ID is 1-indexed
            let player_name = self.config.player_names[idx as usize];
            let mut controller =
                try!(Controller::new(entity_id, player_name).map_err(|x| GameCreationError::InvalidControllerData(x)));

            // SET ALL CONTROLLER DEFAULT ENTITY TAGS
            // TODO; These hardcoded values might be better moved into
            // GameConfig.
            //
            controller.set_native_tag_value(EGameTags::Maxhandsize, 10);
            controller.set_native_tag_value(EGameTags::Starthandsize, 4);
            controller.set_native_tag_value(EGameTags::PlayerID, player_idx);
            controller.set_native_tag_value(EGameTags::TeamID, player_idx);
            controller.set_native_tag_value(EGameTags::Controller, player_idx);
            controller.set_native_tag_value(EGameTags::Zone, EZones::Play as u32);
            controller.set_native_tag_value(EGameTags::Maxresources, 10);

            self.entities.insert(entity_id, Box::new(controller));
        }

        Ok(self)
    }

    fn build_heroes(mut self) -> Result<Self, GameCreationError> {
        if self.config.build_heroes == true {
            for idx in 0..MAX_PLAYERS {
                let player_idx = idx + 1; // Player ID is 1-indexed

                let hero_class = self.config.player_heroclasses[idx as usize].expect("No heroclass set");
                let hero_card = *CARDS.hero_cards(hero_class).first().expect("No hero card found");
                let hero =
                    try!(self.init_entity::<Hero>(hero_card).map_err(|x| GameCreationError::HeroConstructionError(x)));

                // SET DEFAULT HERO TAGS
                hero.set_native_tag_value(EGameTags::Controller, player_idx);
                hero.set_native_tag_value(EGameTags::Zone, EZones::Play as u32);
            }
        }

        Ok(self)
    }

    fn build_hero_powers(mut self) -> Result<Self, GameCreationError> {
        if self.config.build_hero_powers == true {
            for idx in 0..MAX_PLAYERS {
                let player_idx = idx + 1; // Player ID is 1-indexed

                let hero_class = self.config.player_heroclasses[idx as usize].expect("No heroclass set");
                let power_card = *CARDS.hero_power_cards(hero_class).first().expect("No heropower card found");
                let hero_power = try!(self.init_entity::<HeroPower>(power_card).map_err(|x| {
                    GameCreationError::HeroConstructionError(x)
                }));

                // SET DEFAULT HERO_POWER TAGS
                hero_power.set_native_tag_value(EGameTags::Controller, player_idx);
                hero_power.set_native_tag_value(EGameTags::Zone, EZones::Play as u32);
            }
        }

        Ok(self)
    }

    pub fn init_entity<T: IEntityInitializable + IEntityCastable>(
        &mut self,
        card: &'static ICard,
    ) -> Result<&mut T, EntityCreationError> {
        let entity_id = self.next_eid();
        let entity = try!(T::new(entity_id, card));
        // Consume id and entity object.
        self.entities.insert(entity_id, entity);

        let entity_ref = self.get_entity_mut(entity_id).unwrap();
        let t_ref = try!(T::try_into_mut(entity_ref).map_err(|x| EntityCreationError::InvalidCast(x)));
        Ok(t_ref)
    }

    pub fn get_entity<'a>(&'a self, entity_id: u32) -> Option<&'a IEntity> {
        self.entities.get(&entity_id).map(|box_ref| box_ref.borrow())
    }

    pub fn get_entity_mut<'a>(&'a mut self, entity_id: u32) -> Option<&'a mut IEntity> {
        let val = match self.entities.get_mut(&entity_id) {
            Some(entity) => {
                let z: &mut IEntity = entity.borrow_mut();
                return Some(z);
            },
            None => None,
        };
        val
    }
}
