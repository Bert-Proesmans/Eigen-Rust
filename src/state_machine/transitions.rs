use game::GameMachine;
use state_machine::{action_states, core_states, trigger_states, wait_states};

// Generic transitions

// Trigger<X> -> Effect<X>
impl<T> From<GameMachine<core_states::Trigger<T>>> for GameMachine<core_states::Effect<T>>
where
    T: core_states::StateTriggerable
        + Default,
{
    fn from(old: GameMachine<core_states::Trigger<T>>) -> Self {
        GameMachine { state: core_states::Effect::<T>::default() }
    }
}

// Effect<X> -> Trigger<X/Y/Z>
impl<T,Y> From<GameMachine<core_states::Effect<T>>> for
		GameMachine<core_states::Trigger<Y>>
where
	T: core_states::StateTriggerable + Default,
	Y: core_states::StateTriggerable + Default
{
    fn from(old: GameMachine<core_states::Effect<T>>) -> Self {
    	GameMachine {
    		state: core_states::Trigger::<Y>::default(),
    	}
    }
}

// Effect<X> -> PostTrigger<X>
impl<T> From<GameMachine<core_states::Effect<T>>> for
		GameMachine<core_states::PostTrigger<T>>
where
	T: core_states::StateTriggerable + Default
{
    fn from(old: GameMachine<core_states::Effect<T>>) -> Self {
    	GameMachine {
    		state: core_states::PostTrigger::<T>::default(),
    	}
    }
}

// Effect<X> -> Death<X>
impl<T> From<GameMachine<core_states::Effect<T>>> for GameMachine<core_states::Death<T>>
where
    T: core_states::StateTriggerable
        + Default,
{
    fn from(old: GameMachine<core_states::Effect<T>>) -> Self {
        GameMachine { state: core_states::Death::<T>::default() }
    }
}

// Explicit transitions

// Wait<Start> -> Action<StartGame>
// impl From<GameMachine<core_states::Wait<wait_states::
// Start>>> for
// GameMachine<core_states::Action<action_states::
// StartGame>>
// {
// fn from(old:
// GameMachine<core_states::Wait<wait_states::Start>>) ->
// Self {
//     	GameMachine {
// state:
// core_states::Action::<action_states::StartGame>::
// default(),
//     	}
//     }
// }

// DBG; Wait<Start> -> Wait<Input>
impl From<GameMachine<core_states::Wait<wait_states::Start>>> for GameMachine<core_states::Wait<wait_states::Input>> {
    fn from(old: GameMachine<core_states::Wait<wait_states::Start>>) -> Self {
        GameMachine { state: core_states::Wait::<wait_states::Input>::default() }
    }
}

// Wait<Input> -> Action<EndTurn>
impl From<GameMachine<core_states::Wait<wait_states::Input>>>
    for GameMachine<core_states::Action<action_states::EndTurn>> {
    fn from(old: GameMachine<core_states::Wait<wait_states::Input>>) -> Self {
        GameMachine { state: core_states::Action::<action_states::EndTurn>::default() }
    }
}


// PostAction<EndTurn> -> WaitInput
impl From<GameMachine<core_states::PostAction<action_states::EndTurn>>>
    for GameMachine<core_states::Wait<wait_states::Input>> {
    fn from(old: GameMachine<core_states::PostAction<action_states::EndTurn>>) -> Self {
        GameMachine { state: core_states::Wait::<wait_states::Input>::default() }
    }
}

// Action<EndTurn> -> Trigger<EndTurn>
impl From<GameMachine<core_states::Action<action_states::EndTurn>>>
    for GameMachine<core_states::Trigger<trigger_states::EndTurn>> {
    fn from(old: GameMachine<core_states::Action<action_states::EndTurn>>) -> Self {
        GameMachine { state: core_states::Trigger::<trigger_states::EndTurn>::default() }
    }
}

// Death<EndTurn> -> Trigger<StartTurn>
impl From<GameMachine<core_states::Death<trigger_states::EndTurn>>>
    for GameMachine<core_states::Trigger<trigger_states::StartTurn>> {
    fn from(old: GameMachine<core_states::Death<trigger_states::EndTurn>>) -> Self {
        GameMachine { state: core_states::Trigger::<trigger_states::StartTurn>::default() }
    }
}

// Death<StartTurn> -> PostAction<EndTurn>
impl From<GameMachine<core_states::Death<trigger_states::StartTurn>>>
    for GameMachine<core_states::PostAction<action_states::EndTurn>> {
    fn from(old: GameMachine<core_states::Death<trigger_states::StartTurn>>) -> Self {
        GameMachine { state: core_states::PostAction::<action_states::EndTurn>::default() }
    }
}
