use std::fmt;

use state_machine::trigger_states::TriggerState;

pub trait CoreState: fmt::Debug {} // Marker

#[derive(Debug)]
pub struct Invalid {}
impl CoreState for Invalid {}

#[derive(Debug)]
pub struct AwaitingStart {}
impl CoreState for AwaitingStart {}
impl AwaitingStart {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct Finished {}
impl CoreState for Finished {}

#[derive(Debug)]
pub struct Input {}
impl CoreState for Input {}

#[derive(Debug)]
pub struct Trigger<I>
where
    I: TriggerState,
{
    pub internal: I
}
impl<I> CoreState for Trigger<I>
where
    I: TriggerState,
{
}

#[derive(Debug)]
pub struct Effect {}
impl CoreState for Effect {}

#[derive(Debug)]
pub struct Death {}
impl CoreState for Death {}

#[derive(Debug)]
pub struct Neutral {}
impl CoreState for Neutral {}
