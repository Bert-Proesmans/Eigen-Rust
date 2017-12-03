use entities::entity::IEntity;
use entities::entity_data::EntityData;

pub type EntityId = u32;
pub const GAME_ENTITY_ID: EntityId = 1;

static DBG_U32: u32 = 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Primitive)]
pub enum EController {
    ControllerOne = 1,
    ControllerTwo = 2
}

impl EController {
    pub fn next(self) -> Self {
        match self {
            EController::ControllerOne => EController::ControllerTwo,
            EController::ControllerTwo => EController::ControllerOne,
        }
    }
}

pub const PLAYER_ONE: EController = EController::ControllerOne;
pub const PLAYER_TWO: EController = EController::ControllerTwo;

#[derive(Debug)]
pub struct Game<'machine> {
    data: EntityData,
    card: &'machine u32
}

impl<'mx> IEntity<'mx> for Game<'mx> {
    fn _get_data_internal(&self) -> &EntityData {
        &self.data
    }

    fn _get_data_internal_mut(&mut self) -> &mut EntityData {
        &mut self.data
    }
}

impl<'mx> Game<'mx> {
    pub fn new() -> Result<Self, String> {
        let e_data = try!(EntityData::new(GAME_ENTITY_ID));

        Ok(Game {
            data: e_data,
            card: &DBG_U32
        })
    }
}

#[derive(Debug)]
pub struct Controller<'machine> {
    data: EntityData,
    card: &'machine u32
}

impl<'mx> IEntity<'mx> for Controller<'mx> {
    fn _get_data_internal(&self) -> &EntityData {
        &self.data
    }

    fn _get_data_internal_mut(&mut self) -> &mut EntityData {
        &mut self.data
    }
}
