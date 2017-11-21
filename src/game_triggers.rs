use std::collections::HashMap;
use std::fmt;

use entities::core_entities::{EController, EntityId};
use game_zones::EZones;
use state_machine::dynamic::MethodTrigger;
use state_machine::trigger_states;

#[derive(Debug)]
pub struct TriggerContainer {
    pub(crate) game_global: Vec<StoredTrigger>,
    pub(crate) zone_specific: HashMap<(EController, EZones), Vec<StoredTrigger>>,
    pub(crate) entity_specific: HashMap<EntityId, Vec<StoredTrigger>>
}

impl TriggerContainer {
    pub fn new() -> Self {
        Self {
            game_global: vec![],
            zone_specific: hashmap!{},
            entity_specific: hashmap!{},
        }
    }

    pub fn store_global_trigger(&mut self, trigger: Box<ITrigger>) {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum StoredTrigger {
    Actual(Box<ITrigger>),
    Marker(TriggerMarker)
}

pub trait ITrigger: fmt::Debug {}

#[derive(Debug, Builder)]
#[builder(public, pattern = "owned")]
pub struct Trigger<T>
where
    T: trigger_states::TriggerState,
{
    pub(crate) source: EntityId,
    pub(crate) effect: MethodTrigger<T>,
    pub(crate) number_of_runs: u32
}

impl<T> ITrigger for Trigger<T>
where
    T: trigger_states::TriggerState
{
}

#[derive(Debug)]
pub struct TriggerMarker {}
