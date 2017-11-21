use std::fmt;

pub trait TriggerState: fmt::Debug {} // Marker

#[derive(Debug)]
pub enum ETriggerState {
    StartGame,
    EndGame,
    StartTurn,
    EndTurn
}

#[derive(Debug, Default)]
pub struct StartGame {}
impl TriggerState for StartGame {}

#[derive(Debug, Default)]
pub struct EndGame {}
impl TriggerState for EndGame {}

#[derive(Debug, Default)]
pub struct StartTurn {}
impl TriggerState for StartTurn {}

#[derive(Debug, Default)]
pub struct EndTurn {}
impl TriggerState for EndTurn {}

// **********

#[derive(Debug, Default)]
pub struct ZoneChange {}
impl TriggerState for ZoneChange {}

#[derive(Debug, Default)]
pub struct Concede {}
impl TriggerState for Concede {}

// **********

#[derive(Debug, Default)]
pub struct Shuffle {}
impl TriggerState for Shuffle {}

#[derive(Debug, Default)]
pub struct Mulligan {}
impl TriggerState for Mulligan {}

#[derive(Debug, Default)]
pub struct MulliganWait {}
impl TriggerState for MulliganWait {}

#[derive(Debug, Default)]
pub struct Resource {}
impl TriggerState for Resource {}

#[derive(Debug, Default)]
pub struct Draw {}
impl TriggerState for Draw {}

#[derive(Debug, Default)]
pub struct Death {}
impl TriggerState for Death {}

#[derive(Debug, Default)]
pub struct TagChange {}
impl TriggerState for TagChange {}
