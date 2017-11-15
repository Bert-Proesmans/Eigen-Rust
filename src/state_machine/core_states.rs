use std::fmt;

        pub trait StateMarker: fmt::Debug {}
        pub trait GameInternalStateMarker: fmt::Debug {}

        #[derive(Debug)]
        pub struct Invalid {}
        impl StateMarker for Invalid {}

        #[derive(Debug)]
        pub struct AwaitingStart {}
        impl StateMarker for AwaitingStart {}
        impl AwaitingStart {
            pub fn new() -> Self {
                Self {}
            }
        }

        #[derive(Debug)]
        pub struct Finished {}
        impl StateMarker for Finished {}

        #[derive(Debug)]
        pub struct Internal<I> 
        where I: GameInternalStateMarker
        {
            pub game_state: I,
        }
        impl<I> StateMarker for Internal<I>
        where I: GameInternalStateMarker
        {}

        #[derive(Debug)]
        pub struct Waiting {}
        impl StateMarker for Waiting {}

        #[derive(Debug)]
        pub struct Trigger {}
        impl StateMarker for Trigger {}

        #[derive(Debug)]
        pub struct Effect {}
        impl StateMarker for Effect {}
