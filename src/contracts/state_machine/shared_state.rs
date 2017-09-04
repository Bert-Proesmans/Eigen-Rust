use std::fmt::{Debug,Display};

// TODO
pub trait SharedState: Debug + Display {

    fn playables(&self) -> i32;

    fn card_ids(&self) -> i32;

    fn registers(&self) -> &[i32; 5];

    fn flags(&self) -> i16;

    fn clear(&mut self);
}
