
pub trait StateCore {
    fn enter(&self) {
        unimplemented!()
    }

    fn exit(&self) {
        unimplemented!()
    }
}

pub trait StateWaitable {}
pub trait StateActionable {}
pub trait StateTriggerable {}

#[derive(Debug)]
pub struct Wait<T>
where
    T: StateWaitable + Default,
{
    i_type: T
}

impl<T> StateCore for Wait<T>
where
    T: StateWaitable + Default,
{
}

impl<T> Default for Wait<T>
where
    T: StateWaitable + Default,
{
    fn default() -> Self {
        Self { i_type: T::default() }
    }
}


#[derive(Debug)]
pub struct Finished {}

impl StateCore for Finished {}


#[derive(Debug)]
pub struct Action<T>
where
    T: StateActionable + Default,
{
    i_type: T
}

impl<T> StateCore for Action<T>
where
    T: StateActionable + Default,
{
}

impl<T> Default for Action<T>
where
    T: StateActionable + Default,
{
    fn default() -> Self {
        Self { i_type: T::default() }
    }
}


#[derive(Debug)]
pub struct PostAction<T>
where
    T: StateActionable + Default,
{
    i_type: T
}

impl<T> StateCore for PostAction<T>
where
    T: StateActionable + Default,
{
}

impl<T> Default for PostAction<T>
where
    T: StateActionable + Default,
{
    fn default() -> Self {
        Self { i_type: T::default() }
    }
}


#[derive(Debug)]
pub struct Trigger<T>
where
    T: StateTriggerable + Default,
{
    i_type: T
}

impl<T> StateCore for Trigger<T>
where
    T: StateTriggerable + Default,
{
}

impl<T> Default for Trigger<T>
where
    T: StateTriggerable + Default,
{
    fn default() -> Self {
        Self { i_type: T::default() }
    }
}

#[derive(Debug)]
pub struct PostTrigger<T>
where
    T: StateTriggerable + Default,
{
    i_type: T
}

impl<T> StateCore for PostTrigger<T>
where
    T: StateTriggerable + Default,
{
}

impl<T> Default for PostTrigger<T>
where
    T: StateTriggerable + Default,
{
    fn default() -> Self {
        Self { i_type: T::default() }
    }
}


#[derive(Debug)]
pub struct Effect<T>
where
    T: StateTriggerable + Default,
{
    i_type: T
}

impl<T> StateCore for Effect<T>
where
    T: StateTriggerable + Default,
{
}

impl<T> Default for Effect<T>
where
    T: StateTriggerable + Default,
{
    fn default() -> Self {
        Self { i_type: T::default() }
    }
}


#[derive(Debug)]
pub struct Death<T>
where
    T: StateTriggerable + Default,
{
    i_type: T
}

impl<T> StateCore for Death<T>
where
    T: StateTriggerable + Default,
{
}

impl<T> Default for Death<T>
where
    T: StateTriggerable + Default,
{
    fn default() -> Self {
        Self { i_type: T::default() }
    }
}
