use state_machine::core_states::GameInternalStateMarker;

#[derive(Debug)]
pub struct Mulligan {}
impl GameInternalStateMarker for Mulligan {}

#[derive(Debug)]
pub struct MulliganWait {}
impl GameInternalStateMarker for MulliganWait {}

#[derive(Debug)]
pub struct Death {}
impl GameInternalStateMarker for Death {}
