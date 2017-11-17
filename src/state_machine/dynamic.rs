use std::marker::PhantomData;

use game::GameTriggerState;
use state_machine::trigger_states;

#[derive(Debug)]
pub enum EExecutionStates {
    Invalid,
    Waiting,
    Running,
    Finished,
    Aborted
}

#[derive(Debug)]
pub struct Method<T, F>
where
    T: trigger_states::TriggerState,
    F: Fn(&mut GameTriggerState<T>) -> EExecutionStates
{
    pub(crate) last_state: EExecutionStates,
    pub(crate) delegate: F,
    phantom: PhantomData<T>,
}
