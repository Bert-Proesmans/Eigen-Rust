use std::collections::HashMap;

use game_tags::EGameTags;


#[derive(Debug)]
pub struct EntityData {
    id: u32,
    state: HashMap<EGameTags, u32>
}

impl EntityData {
    pub fn new(entity_id: u32) -> Result<Self, String> {
        Ok(Self {
            id: entity_id,
            state: hashmap!{EGameTags::EntityId => entity_id}
        })
    }

    pub fn get_tag(
        &self,
        tag: EGameTags,
    ) -> u32 {
        self.state.get(&tag)
    				// Make a copy of the integer.
    				.map(|&v| v)
    				// Return 0 if non existant.
    				.unwrap_or(0)
    }

    pub fn set_tag(
        &mut self,
        tag: EGameTags,
        value: u32,
    ) -> Option<u32> {
        self.state.insert(tag, value)
    }
}
