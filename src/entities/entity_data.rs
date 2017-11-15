use std::collections::HashMap;

use game_tags::EGameTags;


#[derive(Debug)]
pub struct EntityData {
    id: u32,
    state: HashMap<EGameTags, u32>,
}

impl EntityData {
    pub fn new(entity_id: u32) -> Result<Self, String> {
    	Ok(Self {
    		id: entity_id,
    		state: hashmap!{EGameTags::EntityId => entity_id},
    	})
    }
}
