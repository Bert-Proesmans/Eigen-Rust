use std::fmt::{Debug, Display};
use std::collections::HashMap;

use enums::{EGameTags, EZones};

use core::Card;

pub trait Entity: Debug + Display {
    // The game reference (which holds this entity) is detached.

    fn id(&self) -> u32;
    fn order_of_play_idx(&self) -> u32;

    fn reference_card(&self) -> &'static Card;

    fn zone_id(&self) -> Option<EZones>;
    fn controller_id(&self) -> u32;

    fn native_tag_value(&self, tag: EGameTags) -> u32;
    fn set_native_tag_value(&mut self, tag: EGameTags, val: u32);

    fn tag_value(&self, tag: EGameTags) -> u32;

    // Into Playable trait object.
    fn try_into_playable(&self) -> i32; // TODO
}

#[derive(Debug)]
pub struct EntityData {
    state: HashMap<EGameTags, u32>,
}
