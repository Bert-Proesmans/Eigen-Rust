use std::fmt;

pub trait TriggerState: fmt::Debug {} // Marker

#[derive(Debug)]
pub enum ETriggerState {
    StartGame,
    EndGame,
    StartTurn,
    EndTurn
}

#[derive(Debug)]
pub struct StartGame {}
impl TriggerState for StartGame {}

#[derive(Debug)]
pub struct EndGame {}
impl TriggerState for EndGame {}

#[derive(Debug)]
pub struct StartTurn {}
impl TriggerState for StartTurn {}

#[derive(Debug)]
pub struct EndTurn {}
impl TriggerState for EndTurn {}

// **********

#[derive(Debug)]
pub struct ZoneChange {}
impl TriggerState for ZoneChange {}

#[derive(Debug)]
pub struct Concede {}
impl TriggerState for Concede {}

// **********

#[derive(Debug)]
pub struct Shuffle {}
impl TriggerState for Shuffle {}

#[derive(Debug)]
pub struct Mulligan {}
impl TriggerState for Mulligan {}

#[derive(Debug)]
pub struct MulliganWait {}
impl TriggerState for MulliganWait {}

#[derive(Debug)]
pub struct Resource {}
impl TriggerState for Resource {}

#[derive(Debug)]
pub struct Draw {}
impl TriggerState for Draw {}

#[derive(Debug)]
pub struct Death {}
impl TriggerState for Death {}

#[derive(Debug)]
pub struct TagChange {}
impl TriggerState for TagChange {}
