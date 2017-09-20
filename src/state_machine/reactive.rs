use std::rc::{Rc, Weak};

use enums::EEvents;

#[derive(Debug)]
pub struct Dispatcher<T>
where
    T: Observer,
{
    observers: Vec<Weak<T>>
}

pub trait Observer {
    fn on_update(
        &self,
        event: &EEvents,
    );
}

pub trait Observable<T>
where
    T: Observer,
{
    // We don't use RefCell since it enlarges the risk of
    // runtime panics heavily.
    // If mutability is required, we opt for INTERIOUR
    // mutability.
    fn register_observer(
        &mut self,
        observer: Rc<T>,
    );
}

impl<T> Observable<T> for Dispatcher<T>
where
    T: Observer,
{
    fn register_observer(
        &mut self,
        observer: Rc<T>,
    ) {
        self.observers.push(Rc::downgrade(&observer));
    }
}

impl<T> Dispatcher<T>
where
    T: Observer,
{
    pub fn new() -> Self {
        Self { observers: Vec::new() }
    }

    pub fn num_observers(&self) -> usize {
        self.observers.len()
    }

    pub fn dispatch(
        &mut self,
        event: EEvents,
    ) {
        let mut cleanup = false;

        for observer_weak in self.observers.iter() {
            if let Some(observer_rc) = observer_weak.upgrade() {
                observer_rc.on_update(&event); // Implicitly dereferenced here.
            } else {
                // All strong pointers to the observer are removed. This
                // means we can unsubscrive the observer without damage.
                cleanup = true;
            }
        }

        if cleanup {
            self.clean_up();
        }
    }

    fn clean_up(&mut self) {
        self.observers.retain(|o| match o.upgrade() {
            None => false,
            _ => true,
        });
    }
}
