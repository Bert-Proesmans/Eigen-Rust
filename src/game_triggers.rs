use std::fmt;

use game::GameTriggerState;
use entities::core_entities::EntityId;
use state_machine::trigger_states;
use state_machine::dynamic::Method;

use state_machine::dynamic::EExecutionStates;

#[derive(Debug)]
pub enum StoredTrigger {
    Actual(Box<ITrigger>),
    Marker(TriggerMarker),
}

pub trait ITrigger: fmt::Debug {}

#[derive(Debug)]
pub struct Trigger<T, F> 
where
    T: trigger_states::TriggerState,
    F: Fn(&mut GameTriggerState<T>) -> EExecutionStates + fmt::Debug
{
    pub(crate) source: EntityId,
    pub(crate) effect: Method<T, F>,
    pub(crate) number_of_runs: u32,
}

impl<T, F> ITrigger for Trigger<T, F> 
where 
    T: trigger_states::TriggerState,
    F: Fn(&mut GameTriggerState<T>) -> EExecutionStates + fmt::Debug
{}

#[derive(Debug)]
pub struct TriggerMarker {}