#[derive(Debug, Copy, Clone)]
pub enum EActivationRequirements {
    /// No requirement, always activates
    NoReq,
    /// Indicates battlecry effect, when the card entity is
    /// "played"
    Battlecry,
    /// Indicates deathrattle effect, when the card entity
    /// is moved to
    /// the GRAVEYARD zone
    Deathrattle,
    /// Indicates inspire effect, when the owning player
    /// has "played"
    /// his hero power
    Inspire,

    /// Indicates the entity must be hooked as a weapon
    Weapon,
    /// Indicates the entity is executed as a spell
    Spell,
    /// Indicates the entity is executed as a secret or
    /// quest
    SecretOrQuest
}

#[derive(Debug, Copy, Clone)]
pub enum EExecutionStates {
    Invalid,
    Ready,
    Running,
    Interrupt,
    Finished,
    Abort
}

#[derive(Debug, Copy, Clone)]
pub enum EActivationTargets {
    NoTarget,
    Target,
    Hero,
    OpHero,
    Heroes,

    Board,
    OpBoard,
    Boards,

    Hand,
    OpHand,
    Hands,

    TargetSelf,
    Game,

    Controller,
    OpController,
    Controllers,

    /// <summary>
    /// The effect will be triggered by the secrets of the
    /// controller.
    /// </summary>
    Secret,

    /// <summary>
    /// The effect will be triggered by the secrets of all
    /// controllers.
    /// </summary>
    Secrets,

    HandAndBoard,
    OpBoardAndOpHero,

    BoardsHeroes,

    Graveyard,
    GraveyardAndSecret,

    OpGraveyard,
    OpGraveyardAndOpSecret
}
