
#[derive(Debug)]
pub enum EExecutionStates {
    Invalid = 0,
    Ready = 1,
    Running = 2,
    Finished = 3,
    Abort = 4,
}

#[derive(Debug)]
pub enum EActivationRequirements {
    None = 0,
    Battlecry,
    Deathrattle,
    Inspire,
    Weapon,
    Spell,
    SecretOrQuest,

    // BOARD_ZONE,
    // HAND_ZONE,
    // DECK_ZONE,
    // SETASIDE_ZONE
}
