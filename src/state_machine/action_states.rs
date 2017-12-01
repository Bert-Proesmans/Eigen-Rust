use state_machine::core_states::StateActionable;

#[derive(Debug, Default)]
pub struct StartGame {}
impl StateActionable for StartGame {}

#[derive(Debug, Default)]
pub struct EndTurn {}
impl StateActionable for EndTurn {}

#[derive(Debug, Default)]
pub struct Concede {}
impl StateActionable for Concede {}
