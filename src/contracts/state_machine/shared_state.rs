use std::fmt::{Debug, Display};

use contracts::models::IPlayable;
use contracts::state_machine::IMethod;

use enums::contracted::EExecutionStates;

pub trait ISharedState: Debug + Display {
    fn register_num(&self) -> u32;

    fn playables(&self) -> &Vec<&IPlayable>;
    fn card_ids(&self) -> &Vec<&'static str>;
    fn flags(&self) -> u32;

    fn add_playable(
        &mut self,
        subj: &IPlayable,
    );
    fn add_card_id(
        &mut self,
        subj: &'static str,
    );
    fn set_register(
        &mut self,
        register: u32,
        value: i32,
    );
    fn set_flags(
        &mut self,
        subj: u32,
    );

    fn push_method(
        &mut self,
        method: Box<IMethod>,
        fast: bool,
    ) -> Result<EExecutionStates, bool>;

    fn clear_all(&mut self);
    fn drain_playables(&mut self) -> Iterator<Item = &IPlayable>;
    fn drain_card_ids(&mut self) -> Iterator<Item = &'static str>;
    fn clear_register(
        &mut self,
        idx: u32,
    );
    fn clear_flags(&mut self);
}
