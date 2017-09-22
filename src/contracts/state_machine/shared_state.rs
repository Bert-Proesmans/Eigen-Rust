use std::collections::HashSet;
use std::collections::hash_set::Drain;
use std::fmt::{Debug, Display};

use contracts::entities::playable::IPlayable;

/// Holds the data shared by IProgram and IMethod
///
/// This data is accessible and mutable by every IMethod.
/// The purpose of this object is comparable to that of
/// CPU registers. IProgram is comparable to the CPU.
pub trait ISharedState<'a>: Debug + Display {
    /// Returns the amount of registers are shared
    fn register_num(&self) -> u32;

    /// Returns the stored set of IPlayable references
    fn playables(&self) -> &HashSet<&IPlayable>;

    /// Returns the stored set of card database ID's
    fn card_ids(&self) -> &HashSet<u32>;

    /// Returns the integer holding all flags
    fn flags(&self) -> u32;

    /// Appends another IPlayable reference to the stored
    /// vector
    fn add_playable(
        &mut self,
        subj: &'a IPlayable,
    );

    /// Appends another card database ID to the stored
    /// vector
    fn add_card_dbf_id(
        &mut self,
        id: u32,
    );

    /// Overwrites the value of the provided register with
    /// the provided value
    ///
    /// # Panics
    ///
    /// Panics when the value of `register` is greater or
    /// equal than the amount
    /// returned by `register_num`.
    fn set_register(
        &mut self,
        register: u32,
        value: i32,
    );

    /// Enables the flags, which are provided trough
    /// `flags`, within this state
    ///
    /// # Example
    ///
    /// TODO; Example
    ///
    fn enable_flags(
        &mut self,
        flags: u32,
    );

    /// Disables the flags, which are provided through
    /// `flags`, within this state
    ///
    /// # Example
    ///
    /// TODO; Example
    ///
    fn disable_flags(
        &mut self,
        flags: u32,
    );


    /// Reset all internal fields to their default value
    fn clear_all(&mut self);

    /// Clear the set of IPlayables, returning an iterator
    /// over all
    /// removed elements
    fn drain_playables(&mut self) -> Drain<&IPlayable>;

    /// Clear the set of card database ID's, returning an
    /// iterator
    /// over all removed elements
    fn drain_card_ids(&mut self) -> Drain<u32>;

    /// Reset the value of the specified register
    fn clear_register(
        &mut self,
        idx: u32,
    );

    /// Reset the integer holding all flags to it's default
    /// value
    fn reset_flags(&mut self);
}
