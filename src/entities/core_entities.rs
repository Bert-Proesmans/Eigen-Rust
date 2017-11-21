use entities::entity::IEntity;
use entities::entity_data::EntityData;

pub type EntityId = u32;
pub const GAME_ENTITY_ID: EntityId = 1;

#[derive(Debug, PartialEq, Eq, Hash, Primitive)]
pub enum EController {
    ControllerOne = 1,
    ControllerTwo = 2
}

pub const PLAYER_ONE: EController = EController::ControllerOne;
pub const PLAYER_TWO: EController = EController::ControllerTwo;

#[derive(Debug)]
pub struct Game {
    data: EntityData,
    // Reference, the referenced object should live
    // as long as possible!
    card: u32
}

impl IEntity for Game {
    fn _get_data_internal(&self) -> &EntityData {
        &self.data
    }

    fn _get_data_internal_mut(&mut self) -> &mut EntityData {
        &mut self.data
    }
}

impl Game {
    pub fn new() -> Result<Self, String> {
        let e_data = try!(EntityData::new(GAME_ENTITY_ID));

        Ok(Game {
            data: e_data,
            card: 0
        })
    }
}

#[derive(Debug)]
pub struct Controller {
    data: EntityData,
    card: u32
}

impl IEntity for Controller {
    fn _get_data_internal(&self) -> &EntityData {
        &self.data
    }

    fn _get_data_internal_mut(&mut self) -> &mut EntityData {
        &mut self.data
    }
}
