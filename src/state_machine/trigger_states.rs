use state_machine::core_states::StateTriggerable;

#[derive(Debug, Default)]
pub struct EndTurn {}
impl StateTriggerable for EndTurn {}

#[derive(Debug, Default)]
pub struct StartTurn {}
impl StateTriggerable for StartTurn {}
