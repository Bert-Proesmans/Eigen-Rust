// extern crate eigen_rust;

// use std::cell::Cell;
// use std::rc::Rc;

// use eigen_rust::state_machine::reactive;
// use eigen_rust::core::state_machine::reactive::Dispatchable;
// use eigen_rust::enums::EEvents;

// struct TestObserver {
//     pub event_processed: Cell<bool>,
// }

// impl TestObserver {
//     fn new() -> Self {
//         Self { event_processed: Cell::new(false) }
//     }
// }

// impl reactive::Observer for TestObserver {
//     fn on_update(&self, event: &EEvents) {
//         println!("Notify called on TestObserver with {:?}", event);
//         match *event {
//             EEvents::Test => self.event_processed.set(true),
//         }
//     }
// }

// // TODO; the following test is actually a unit test.
// // Move this test back into core and build a proper
// // integration test!

// #[test]
// fn test_reactive() {
//     let mut dispatcher: reactive::Dispatcher<TestObserver> = reactive::Dispatcher::new();
//     {
//         let observer_rc = Rc::new(TestObserver::new());
//         dispatcher.register_observer(observer_rc.clone());

//         assert_eq!(dispatcher.num_observers(), 1);

//         dispatcher.dispatch(EEvents::Test);

//         let flag = observer_rc.event_processed.get(); // Implicit dereference of Rc
//         assert_eq!(flag, true);
//     } // observer reference becomes invalid now.

//     dispatcher.dispatch(EEvents::Test);
//     assert_eq!(dispatcher.num_observers(), 0);
// }
