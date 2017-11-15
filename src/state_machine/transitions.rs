use game::GameProcessor;
use state_machine::{core_states, internal_states};

impl From<GameProcessor<core_states::AwaitingStart>>  
for GameProcessor<core_states::Waiting> {
    fn from(old: GameProcessor<core_states::AwaitingStart>) -> Self {
        GameProcessor {
            state: core_states::Waiting {},
            entities: old.entities,
            program_state: old.program_state,
        }
    }
}

impl From<GameProcessor<core_states::AwaitingStart>> 
for GameProcessor<core_states::Internal<internal_states::Mulligan>> {
    fn from(old: GameProcessor<core_states::AwaitingStart>) -> Self {
        GameProcessor {
            state: core_states::Internal {
                game_state: internal_states::Mulligan {}
            },
            entities: old.entities,
            program_state: old.program_state,
        }
    }
}

impl From<GameProcessor<core_states::Internal<internal_states::Mulligan>>> 
for GameProcessor<core_states::Internal<internal_states::MulliganWait>> {
    fn from(old: GameProcessor<core_states::Internal<internal_states::Mulligan>>) -> Self {
        GameProcessor {
            state: core_states::Internal {
                game_state: internal_states::MulliganWait {}
            },
            entities: old.entities,
            program_state: old.program_state,
        }
    }
}
