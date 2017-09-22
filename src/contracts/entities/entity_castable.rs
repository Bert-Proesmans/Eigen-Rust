use std::fmt;

use contracts::entities::entity::IEntity;

pub mod errors {
    use enums::ECardTypes;

    error_chain!{
        errors {
            NoCastProvided {
                display("The requested type did not provide a cast method")
            }

            ErasedType {
                display("The underlying type is unknown")
            }

            NonMatchingType(expected: ECardTypes, found: ECardTypes) {
                display("The underlying type `{:?}` did not match the requested type `{:?}`", found, expected)
            }
        }
    }
}

use self::errors::*;

/// This trait is used to downcast IEntities into their
/// struct type
// Kinda similar to the nightly trait TryFrom.
pub trait IEntityCastable: fmt::Debug + fmt::Display {
    /// Tries to convert the provided IEntity object into a
    /// struct type
    ///
    /// The resulting type equals the type which was passed
    /// as generic argument
    /// for `e`.
    fn try_into<'e>(e: &'e IEntity) -> Result<&'e Self>;

    /// Tries to mutably convert the provided IEntity
    /// object into a struct type
    ///
    /// The resulting type equals the type which was passed
    /// as generic argument
    /// for `e`.
    fn try_into_mut<'e>(e: &'e mut IEntity) -> Result<&'e mut Self>;
}
