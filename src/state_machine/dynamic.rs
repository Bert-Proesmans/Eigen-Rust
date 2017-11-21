use game::GameTriggerState;
use state_machine::trigger_states;

#[derive(Debug, Clone)]
pub enum EExecutionStates {
    Invalid,
    Waiting,
    Running,
    Aborted,
    Finished,
}

type GameTriggerDelegate<T> = fn(GameTriggerState<T>) -> Result<GameTriggerState<T>, EExecutionStates>;

#[derive(Debug, Clone)]
pub struct MethodTrigger<T>
where
    T: trigger_states::TriggerState,
{
    pub(crate) last_state: EExecutionStates,
    pub(crate) delegate: GameTriggerDelegate<T>,
}

impl<T> MethodTrigger<T> 
where
    T: trigger_states::TriggerState,
{
    pub fn new(delegate: GameTriggerDelegate<T>) -> Self {
        Self {
            last_state: EExecutionStates::Waiting,
            delegate: delegate,
        }
    }
}
