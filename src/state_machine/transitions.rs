use game::GameMachine;
use state_machine::{core_states, wait_states};
use state_machine::action_trigger_states as ats;

// Generic transitions

// Trigger<X> -> Effect<X>
impl<'mx,T> From<GameMachine<'mx, core_states::Trigger<T>>> for GameMachine<'mx, core_states::Effect<T>>
where
    T: core_states::StateTriggerable + Default
{
    fn from(old: GameMachine<'mx, core_states::Trigger<T>>) -> Self {
        GameMachine {
            state: core_states::Effect::<T>::default(),
            entities: old.entities,
        }
    }
}

// Effect<X> -> Trigger<X/Y/Z>
impl<'mx,T,Y> From<GameMachine<'mx, core_states::Effect<T>>> for
		GameMachine<'mx, core_states::Trigger<Y>>
where
	T: core_states::StateTriggerable + Default,
	Y: core_states::StateTriggerable + Default
{
    fn from(old: GameMachine<'mx, core_states::Effect<T>>) -> Self {
    	GameMachine {
    		state: core_states::Trigger::<Y>::default(),
            entities: old.entities,
    	}
    }
}

// PostTrigger<X> -> Effect<X>
impl<'mx,T> From<GameMachine<'mx, core_states::PostTrigger<T>>> for
		GameMachine<'mx, core_states::Effect<T>>
where
	T: core_states::StateTriggerable + Default
{
    fn from(old: GameMachine<'mx, core_states::PostTrigger<T>>) -> Self {
    	GameMachine {
    		state: core_states::Effect::<T>::default(),
            entities: old.entities,
    	}
    }
}

// Effect<X> -> Death<X>
impl<'mx,T> From<GameMachine<'mx, core_states::Effect<T>>> for GameMachine<'mx, core_states::Death<T>>
where
    T: core_states::StateTriggerable + Default
{
    fn from(old: GameMachine<'mx, core_states::Effect<T>>) -> Self {
        GameMachine {
            state: core_states::Death::<T>::default(),
            entities: old.entities,
        }
    }
}

// Explicit transitions

// Wait<Start> -> Action<StartGame>
// impl From<GameMachine<core_states::Wait<wait_states::
// Start>>> for
// GameMachine<core_states::Action<ats::
// StartGame>>
// {
// fn from(old:
// GameMachine<core_states::Wait<wait_states::Start>>) ->
// Self {
//     	GameMachine {
// state:
// core_states::Action::<ats::StartGame>::
// default(),
//     	}
//     }
// }

// DBG; Wait<Start> -> Wait<Input>
impl<'mx> From<GameMachine<'mx, core_states::Wait<wait_states::Start>>>
    for GameMachine<'mx, core_states::Wait<wait_states::Input>> {
    fn from(old: GameMachine<'mx, core_states::Wait<wait_states::Start>>) -> Self {
        GameMachine {
            state: core_states::Wait::<wait_states::Input>::default(),
            entities: old.entities
        }
    }
}

// Wait<Input> -> Action<EndTurn>
impl<'mx> From<GameMachine<'mx, core_states::Wait<wait_states::Input>>>
    for GameMachine<'mx, core_states::Action<ats::EndTurn>> {
    fn from(old: GameMachine<'mx, core_states::Wait<wait_states::Input>>) -> Self {
        GameMachine {
            state: core_states::Action::<ats::EndTurn>::default(),
            entities: old.entities
        }
    }
}


// PostAction<EndTurn> -> WaitInput
impl<'mx> From<GameMachine<'mx, core_states::PostAction<ats::EndTurn>>>
    for GameMachine<'mx, core_states::Wait<wait_states::Input>> {
    fn from(old: GameMachine<'mx, core_states::PostAction<ats::EndTurn>>) -> Self {
        GameMachine {
            state: core_states::Wait::<wait_states::Input>::default(),
            entities: old.entities
        }
    }
}

// Action<EndTurn> -> Effect<EndTurn>
impl<'mx> From<GameMachine<'mx, core_states::Action<ats::EndTurn>>>
    for GameMachine<'mx, core_states::Effect<ats::EndTurn>> {
    fn from(old: GameMachine<'mx, core_states::Action<ats::EndTurn>>) -> Self {
        GameMachine {
            state: core_states::Effect::<ats::EndTurn>::default(),
            entities: old.entities
        }
    }
}

// Death<EndTurn> -> Trigger<StartTurn>
impl<'mx> From<GameMachine<'mx, core_states::Death<ats::EndTurn>>>
    for GameMachine<'mx, core_states::Trigger<ats::StartTurn>> {
    fn from(old: GameMachine<'mx, core_states::Death<ats::EndTurn>>) -> Self {
        GameMachine {
            state: core_states::Trigger::<ats::StartTurn>::default(),
            entities: old.entities
        }
    }
}

// Death<EndTurn> -> PostAction<EndTurn>
impl<'mx> From<GameMachine<'mx, core_states::Death<ats::EndTurn>>>
    for GameMachine<'mx, core_states::PostAction<ats::EndTurn>> {
    fn from(old: GameMachine<'mx, core_states::Death<ats::EndTurn>>) -> Self {
        GameMachine {
            state: core_states::PostAction::<ats::EndTurn>::default(),
            entities: old.entities
        }
    }
}

// Death<StartTurn> -> PostAction<EndTurn>
impl<'mx> From<GameMachine<'mx, core_states::Death<ats::StartTurn>>>
    for GameMachine<'mx, core_states::PostAction<ats::EndTurn>> {
    fn from(old: GameMachine<'mx, core_states::Death<ats::StartTurn>>) -> Self {
        GameMachine {
            state: core_states::PostAction::<ats::EndTurn>::default(),
            entities: old.entities
        }
    }
}
