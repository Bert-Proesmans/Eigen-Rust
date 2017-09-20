#[derive(Debug)]
pub enum EActivationRequirements {
    NoReq,
    Battlecry,
    Deathrattle,
    Inspire,
    Weapon,
    Spell,
    SecretOrQuest /* BOARD_ZONE,
                   * HAND_ZONE,
                   * DECK_ZONE,
                   * SETASIDE_ZONE */
}

#[derive(Debug)]
pub enum EExecutionStates {
    Invalid,
    Ready,
    Running,
    Interrupt,
    Finished,
    Abort
}
