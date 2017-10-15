use std::borrow::{Borrow, BorrowMut};
use std::collections::VecDeque;
use std::fmt;
use std::slice::Iter;

use slog;
use slog::Drain;
use slog_stdlog;

use contracts::cards::card::ICard;
use contracts::cards::card_container::ICardContainer;

use contracts::entities::entity::IEntity;
use contracts::entities::entity_castable::IEntityCastable;

use contracts::entities::entity_initializable::IEntityInitializable;
use contracts::state_machine::method::IMethod;
use contracts::state_machine::program::IProgram;
use contracts::state_machine::shared_state::ISharedState;

use contracts::entities::errors as EntityError;

use cards::card_container::CARDS;

use game::config::{GameConfig, MAX_PLAYERS};
use game::entities::controller::Controller;

use game::entities::game::Game;
// use game::entities::inheritables::hero::Hero;
// use game::entities::inheritables::hero_power::HeroPower;

use state_machine::shared_state::SharedState;

use enums::{EExecutionStates, EGameSteps, EGameTags, EZones};

// The Game itself is also an entity. The ID of this entity
// is ALWAYS 1 (1-indexed).
pub const GAME_ENTITY_ID: u32 = 1;

pub mod errors {
    error_chain!{
        errors {
            InvalidStateRequirement(current: String, requested: String) {
                display("Impossible to transition from state {} into state {}", current, requested)
            }
        }

        links {
            InvalidConfig(::game::errors::Error, ::game::errors::ErrorKind);
            EntityCreationFail(::contracts::entities::errors::Error, ::contracts::entities::errors::ErrorKind);
            MissingCardData(::contracts::cards::errors::Error,::contracts::cards::errors::ErrorKind);
        }
    }
}

use self::errors::*;

#[derive(Debug)]
pub struct GameManager<'program> {
    config: GameConfig<'program>,
    logger: slog::Logger,

    entities: Vec<Box<IEntity<'program> + 'program>>,

    next_eid: u32,
    next_oop: u32,

    // Game simulation.
    current_step: EGameSteps,
    method_queue: VecDeque<Box<IMethod>>,
    shared_state: SharedState
}

impl<'px> fmt::Display for GameManager<'px> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "GAMEMANAGER [TODO]")
    }
}

impl<'px> GameManager<'px> {
    pub fn new<L: Into<Option<slog::Logger>>>(
        config: GameConfig<'px>,
        logger: L,
    ) -> Result<Self> {
        // Test configuration
        let validated_config: Result<_> = config.validate().map_err(|e| e.into());
        let validated_config = try!(validated_config);

        // Build game entity.
        let game_entity = try!(Game::new());

        let next_eid = GAME_ENTITY_ID + 1;
        let next_oop = 1;

        // Construct logger instance.
        let logger_instance = logger.into().unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!()));

        let obj = Self {
            config: validated_config,
            logger: logger_instance,

            next_eid: next_eid,
            next_oop: next_oop,

            entities: vec![Box::new(game_entity)],

            current_step: EGameSteps::PreGame,
            method_queue: VecDeque::new(),
            shared_state: SharedState::new()
        };


        let obj = try!(obj.build_controllers());
        // let obj = try!(obj.build_heroes());
        // let obj = try!(obj.build_hero_powers());

        Ok(obj)
    }

    pub fn init_entity<T: IEntityInitializable + IEntityCastable>(
        &mut self,
        card: &'px ICard,
    ) -> EntityError::Result<&mut T> {
        let entity_id = self.next_eid();
        let entity = try!(T::new(entity_id, card)) as Box<IEntity>;
        // Consume id and entity object.
        self.entities.push(entity);

        // Since we just created this entity, it's impossible to
        // fail these operations.
        let entity_ref = self.get_entity_mut(entity_id).unwrap();
        let t_ref: EntityError::Result<_> = T::try_into_mut(entity_ref).map_err(|e| e.into());
        let t_ref = try!(t_ref);
        Ok(t_ref)
    }

    pub fn get_entity(
        &self,
        entity_id: u32,
    ) -> Option<&IEntity<'px>> {
        self.entities.get(entity_id as usize).map(|box_ref| box_ref.borrow())
    }

    pub fn get_entity_mut(
        &mut self,
        entity_id: u32,
    ) -> Option<&mut IEntity<'px>> {
        let val = match self.entities.get_mut(entity_id as usize) {
            Some(entity) => {
                let z: &mut IEntity = entity.borrow_mut();
                return Some(z);
            }
            None => None,
        };
        val
    }

    pub fn next_oop(&mut self) -> u32 {
        let val = self.next_oop;
        self.next_oop += 1;
        val
    }

    pub fn process(
        &mut self,
        operation: Box<IMethod>,
    ) -> Result<()> {
        // TODO; Do something with the return value.
        self.fast_execute(operation);

        Ok(())
    }

    /////////////////////
    // Private methods //
    // //////////////////

    fn next_eid(&mut self) -> u32 {
        let val = self.next_eid;
        self.next_eid += 1;
        val
    }

    fn build_controllers(mut self) -> Result<Self> {
        for idx in 0..MAX_PLAYERS {
            let entity_id = self.next_eid();
            let player_idx = 1 + idx; // Player ID is 1-indexed
            let player_name = self.config.player_names[idx as usize];
            let controller: Result<_> = Controller::new(entity_id, player_name).map_err(|e| e.into());
            let mut controller = try!(controller);

            // SET ALL CONTROLLER DEFAULT ENTITY TAGS
            // TODO; These hardcoded values might be better   off moved
            // into GameConfig.

            controller.set_native_tag_value(EGameTags::Maxhandsize, 10);
            controller.set_native_tag_value(EGameTags::Starthandsize, 4);
            controller.set_native_tag_value(EGameTags::PlayerID, player_idx);
            controller.set_native_tag_value(EGameTags::TeamID, player_idx);
            controller.set_native_tag_value(EGameTags::Controller, player_idx);
            controller.set_native_tag_value(EGameTags::Zone, EZones::Play as u32);
            controller.set_native_tag_value(EGameTags::Maxresources, 10);

            self.entities.push(Box::new(controller));
        }

        Ok(self)
    }

    // fn build_heroes(mut self) -> Result<Self> {
    //     if self.config.build_heroes == true {
    //         for idx in 0..MAX_PLAYERS {
    // let player_idx = idx + 1; // Player ID is
    // 1-indexed

    //             // Class is validated by config itself.
    // let hero_class =
    // self.config.player_heroclasses[idx as usize].unwrap();
    // let hero_card: Result<_> =
    // CARDS.hero_cards(hero_class).map_err(|e| e.into());
    // let hero_card =
    // *try!(hero_card).first().unwrap();

    //             // Can't fail.
    // let hero =
    // self.init_entity::<Hero>(hero_card).unwrap();

    //             // SET DEFAULT HERO TAGS
    //
    // hero.set_native_tag_value(EGameTags::Controller,
    // player_idx);
    // hero.set_native_tag_value(EGameTags::Zone,
    // EZones::Play as u32);
    //         }
    //     }

    //     Ok(self)
    // }

    // fn build_hero_powers(mut self) -> Result<Self> {
    //     if self.config.build_hero_powers == true {
    //         for idx in 0..MAX_PLAYERS {
    // let player_idx = idx + 1; // Player ID is
    // 1-indexed

    // let hero_class =
    // self.config.player_heroclasses[idx as usize].unwrap();
    //             let power_card: Result<_> = CARDS.hero_power_cards(hero_class).map_err(|e| e.into());
    // let power_card =
    // *try!(power_card).first().unwrap();


    //             // Can't fail.
    // let hero_power =
    // self.init_entity::<HeroPower>(power_card).unwrap();

    //             // SET DEFAULT HERO_POWER TAGS
    //
    // hero_power.set_native_tag_value(EGameTags::Controller,
    // player_idx);
    //
    // hero_power.set_native_tag_value(EGameTags::Zone,
    // EZones::Play as u32);
    //         }
    //     }

    //     Ok(self)
    // }
}

impl<'px> IProgram<'px> for GameManager<'px> {
    type ProgressStateResult = Result<Self>;

    fn start(self) -> Result<Self> {}

    fn finish(self) -> Result<Self> {}

    fn all_entities(&self) -> Iter<Box<IEntity<'px> + 'px>> {
        // Nightly supports -> impl Iterator<Item....>
        // instead of ->Box<Iterator<Item..>>
        self.entities.iter()
    }

    fn shared_state_mut(&mut self) -> &mut ISharedState<'px> {
        &mut self.shared_state
    }

    /// Process the next queued method.
    fn process_next(&mut self) -> EExecutionStates {
        match self.current_step {
            EGameSteps::PreGame |
            EGameSteps::FinalGameOver => EExecutionStates::Invalid,
            _ => {
                if let Some(mut next_method) = self.method_queue.pop_front() {
                    return next_method.run(self);
                } else {
                    return EExecutionStates::Finished;
                }
            }
        }
    }

    /// Instantly execute the provided method within this
    /// program.
    fn fast_execute(
        &mut self,
        mut method: Box<IMethod>,
    ) -> EExecutionStates {
        method.run(self)
    }

    fn register_method(
        &mut self,
        method: Box<IMethod>,
    ) {
        self.method_queue.push_back(method);
    }

    fn transition_step(
        &mut self,
        _state: EGameSteps,
    ) {

    }
}
