
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

use contracts::entities::entity_data::IEntityData;
use contracts::entities::entity_data::errors::*;

use enums::EGameTags;

#[derive(Debug)]
// Wrapper for mutable acess to tag data.
pub struct EntityData {
    id: u32,
    state: HashMap<EGameTags, u32>
}

impl fmt::Display for EntityData {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "ENTITYDATA [TODO]")
    }
}

impl EntityData {
    pub fn new(entity_id: u32) -> Result<Self> {
        Ok(Self {
            id: entity_id,
            state: hashmap!{EGameTags::EntityID => entity_id}
        })
    }

    pub fn from_data(
        entity_id: u32,
        data: &HashMap<EGameTags, u32>,
    ) -> Result<Self> {

        let mut dictionary_clone = data.clone();
        // Make sure to overwrite the entityID value if it was set.
        dictionary_clone.insert(EGameTags::EntityID, entity_id);

        Ok(Self {
            id: entity_id,
            state: dictionary_clone
        })
    }
}

impl IEntityData for EntityData {
    fn id(&self) -> u32 {
        self.id
    }

    fn set_tag(
        &mut self,
        tag: EGameTags,
        value: u32,
    ) -> Option<u32> {
        // TODO; Disallow setting of certain tags (like EntityID)!
        self.state.insert(tag, value)
    }

    fn get_tag(
        &self,
        tag: EGameTags,
    ) -> Option<u32> {
        self.state.get(&tag)
                    // Deref the returned value from the dictionary
                    .map(|&v| v)
    }
}
