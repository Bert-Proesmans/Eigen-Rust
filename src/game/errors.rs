error_chain! {
    errors {
        NoFormat {
            display("There was no valid format provided")
        }

        StartingPlayerOOB {
            display("The value designating the starting player's index is out of bounds")
        }

        NoName(p_idx: u32) {
            display("No name provided for player with id `{}`", p_idx)
        }

        NoHeroClass(p_idx: u32) {
            display("No hero class provided for player with id `{}`", p_idx)
        }

        NoDeck(p_idx: u32) {
            display("No deck provided for player with id `{}`", p_idx)
        }
    }

    links {
        FailedEntityCreation(::contracts::entities::errors::Error, ::contracts::entities::errors::ErrorKind);
    }
}
