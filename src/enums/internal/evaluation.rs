#[derive(Debug)]
pub enum EActivationRequirements {
    NoReq = 0,
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
