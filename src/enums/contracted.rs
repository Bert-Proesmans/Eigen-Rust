use enums::ECardTypes;

#[derive(Debug)]
pub enum EntityDataCreationError {
    InvalidEntityID { provided: u32, requested: u32 }
}

#[derive(Debug)]
pub enum EntityCreationError {
    InvalidEntityData(EntityDataCreationError),
    InvalidCast(EntityCastError)
}

#[derive(Debug)]
pub enum EntityCastError {
    NoCastProvided, // Implementation explicitly refuses to cast down.
    ErasedType,
    NonMatchingType {
        found: ECardTypes,
        expected: ECardTypes
    }
}

#[derive(Debug)]
pub enum EExecutionStates {
    Invalid = 0,
    Ready = 1,
    Running = 2,
    Finished = 3,
    Abort = 4
}
