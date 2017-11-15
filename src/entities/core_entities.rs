use entities::entity::IEntity;
use entities::entity_data::EntityData;

pub const GAME_ENTITY_ID: u32 = 1;

#[derive(Debug)]
pub struct Game {
    data: EntityData,
    // Reference, the referenced object should live
    // as long as possible!
    card: u32,
}

impl IEntity for Game {}

impl Game {
    pub fn new() -> Result<Self, String> {
    	let e_data = try!(EntityData::new(GAME_ENTITY_ID));

    	Ok(Game {
    		data: e_data,
    		card: 0,
    	})
    }
}
