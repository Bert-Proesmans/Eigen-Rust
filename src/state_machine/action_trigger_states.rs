use state_machine::core_states::StateTriggerable;
use state_machine::core_states::StateActionable;

#[derive(Debug, Default)]
pub struct StartGame {}
impl StateActionable for StartGame {}
impl StateTriggerable for StartGame {}

#[derive(Debug, Default)]
pub struct EndTurn {}
impl StateActionable for EndTurn {}
impl StateTriggerable for EndTurn {}

#[derive(Debug, Default)]
pub struct Concede {}
impl StateActionable for Concede {}
impl StateTriggerable for Concede {}

#[derive(Debug, Default)]
pub struct StartTurn {}
impl StateTriggerable for StartTurn {}
