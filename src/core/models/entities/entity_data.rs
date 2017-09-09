use std::cell::RefCell;
use std::collections::HashMap;

use contracts::models::IEntityData;

use enums::EGameTags;
use enums::contracted::EntityDataCreationError;

#[derive(Debug)]
// Wrapper for mutable acess to tag data.
pub struct EntityData {
    id: u32,
    state: RefCell<HashMap<EGameTags, u32>>
}

impl EntityData {
    pub fn new(entity_id: u32) -> Result<Self, EntityDataCreationError> {
        Ok(Self {
            id: entity_id,
            state: RefCell::new(hashmap!{EGameTags::EntityID => entity_id})
        })
    }

    pub fn from_data(
        entity_id: u32,
        data: &HashMap<EGameTags, u32>,
    ) -> Result<Self, EntityDataCreationError> {

        let mut dictionary_clone = data.clone();
        if let Some(old_value) = dictionary_clone.insert(EGameTags::EntityID, entity_id) {
            if old_value != entity_id {
                return Err(EntityDataCreationError::InvalidEntityID {
                    provided: old_value,
                    requested: entity_id
                });
            }
        }

        Ok(Self {
            id: entity_id,
            state: RefCell::new(data.clone())
        })
    }
}

impl IEntityData for EntityData {
    fn id(&self) -> u32 {
        self.id
    }

    fn set_tag(
        &self,
        tag: EGameTags,
        value: u32,
    ) -> Option<u32> {
        // TODO; Disallow setting of certain tags (like EntityID)!
        let mut dictionary = self.state.borrow_mut();
        dictionary.insert(tag, value)
    }

    fn get_tag(
        &self,
        tag: EGameTags,
    ) -> Option<u32> {
        let dictionary = self.state.borrow();
        dictionary.get(&tag).map(|&v| v) // The returned value is a reference to u32
    }
}
