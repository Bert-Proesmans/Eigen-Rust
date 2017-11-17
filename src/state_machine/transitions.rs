use game::GameProcessor;
use state_machine::{core_states, trigger_states};

// Source: trigger_states::Effect

impl From<GameProcessor<core_states::Effect>> for GameProcessor<core_states::Death> {
    fn from(old: GameProcessor<core_states::Effect>) -> Self {
        GameProcessor {
            state: core_states::Death {},
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

// Source: trigger_states::Death

impl From<GameProcessor<core_states::Death>> for GameProcessor<core_states::Trigger<trigger_states::Death>> {
    fn from(old: GameProcessor<core_states::Death>) -> Self {
        GameProcessor {
            state: core_states::Trigger { internal: trigger_states::Death {} },
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

impl From<GameProcessor<core_states::Death>> for GameProcessor<core_states::Trigger<trigger_states::EndGame>> {
    fn from(old: GameProcessor<core_states::Death>) -> Self {
        GameProcessor {
            state: core_states::Trigger { internal: trigger_states::EndGame {} },
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

impl From<GameProcessor<core_states::Death>> for GameProcessor<core_states::Neutral> {
    fn from(old: GameProcessor<core_states::Death>) -> Self {
        GameProcessor {
            state: core_states::Neutral {},
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

// Source: trigger_states::Neutral

impl From<GameProcessor<core_states::Neutral>> for GameProcessor<core_states::Input> {
    fn from(old: GameProcessor<core_states::Neutral>) -> Self {
        GameProcessor {
            state: core_states::Input {},
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

// Source: core_states::AwaitingStart

impl From<GameProcessor<core_states::AwaitingStart>>
    for GameProcessor<core_states::Trigger<trigger_states::StartGame>> {
    fn from(old: GameProcessor<core_states::AwaitingStart>) -> Self {
        GameProcessor {
            state: core_states::Trigger { internal: trigger_states::StartGame {} },
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

// Source: trigger_states::StartGame

impl From<GameProcessor<core_states::Trigger<trigger_states::StartGame>>> for GameProcessor<core_states::Effect> {
    fn from(old: GameProcessor<core_states::Trigger<trigger_states::StartGame>>) -> Self {
        GameProcessor {
            state: core_states::Effect {},
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

impl From<GameProcessor<core_states::Trigger<trigger_states::StartGame>>> for GameProcessor<core_states::Input> {
    fn from(old: GameProcessor<core_states::Trigger<trigger_states::StartGame>>) -> Self {
        GameProcessor {
            state: core_states::Input {},
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

impl From<GameProcessor<core_states::Trigger<trigger_states::StartGame>>>
    for GameProcessor<core_states::Trigger<trigger_states::MulliganWait>> {
    fn from(old: GameProcessor<core_states::Trigger<trigger_states::StartGame>>) -> Self {
        GameProcessor {
            state: core_states::Trigger { internal: trigger_states::MulliganWait {} },
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

// Source: trigger_states::Input

impl From<GameProcessor<core_states::Input>> for GameProcessor<core_states::Trigger<trigger_states::Concede>> {
    fn from(old: GameProcessor<core_states::Input>) -> Self {
        GameProcessor {
            state: core_states::Trigger { internal: trigger_states::Concede {} },
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

impl From<GameProcessor<core_states::Input>> for GameProcessor<core_states::Trigger<trigger_states::EndTurn>> {
    fn from(old: GameProcessor<core_states::Input>) -> Self {
        GameProcessor {
            state: core_states::Trigger { internal: trigger_states::EndTurn {} },
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

// Source: trigger_states::Concede

impl From<GameProcessor<core_states::Trigger<trigger_states::Concede>>> for GameProcessor<core_states::Effect> {
    fn from(old: GameProcessor<core_states::Trigger<trigger_states::Concede>>) -> Self {
        GameProcessor {
            state: core_states::Effect {},
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

// Source: trigger_states::EndTurn

impl From<GameProcessor<core_states::Trigger<trigger_states::EndTurn>>> for GameProcessor<core_states::Effect> {
    fn from(old: GameProcessor<core_states::Trigger<trigger_states::EndTurn>>) -> Self {
        GameProcessor {
            state: core_states::Effect {},
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}

// Source: trigger_states::Death

impl From<GameProcessor<core_states::Trigger<trigger_states::Death>>> for GameProcessor<core_states::Effect> {
    fn from(old: GameProcessor<core_states::Trigger<trigger_states::Death>>) -> Self {
        GameProcessor {
            state: core_states::Effect {},
            entities: old.entities,
            program_state: old.program_state,
            zones: old.zones,
            triggers: old.triggers
        }
    }
}
