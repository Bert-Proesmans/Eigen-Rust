use state_machine::core_states::StateWaitable;

#[derive(Debug, Default)]
pub struct Start {}
impl StateWaitable for Start {}

#[derive(Debug, Default)]
pub struct Input {}
impl StateWaitable for Input {}
