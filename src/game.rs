use state_machine::core_states::{StateCore, Wait};
use state_machine::wait_states::Start;

#[derive(Debug)]
pub struct GameFactory {}

impl GameFactory {
    pub fn new(_cfg: u32) -> Result<GameMachine<Wait<Start>>, String> {
        let wait_state = Wait::<Start>::default();

        Ok(GameMachine { state: wait_state })
    }
}

#[derive(Debug)]
pub struct GameMachine<S>
where
    S: StateCore,
{
    pub(crate) state: S
}
