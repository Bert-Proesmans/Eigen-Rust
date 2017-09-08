#[derive(Debug)]
pub enum EGameSteps {
    /// <summary>The entry point (first executed step) of
    /// the simulated game.</summary>
    OpeningStart,

    /// <summary>The deck shuffle step.</summary>
    OpeningShuffle,

    /// <summary>The where all cards for mulligan are
    /// drawn.</summary>
    OpeningDraw,

    /// <summary>
    /// The step where the mulligan choices are prepared.
    /// Players have to process their mulligan choices
    /// during this step.
    /// Set <see cref="MAIN_BEGIN"/> as next step to end
    /// the mulligan phase and start
    /// actually simulating player/game interactions.
    /// </summary>
    PhaseMulligan,

    /// <summary>The entry point of the simulation
    /// phase.</summary>
    MainBegin,

    /// <summary>The step which resets all information of
    /// game entities related to
    /// turn information.</summary>
    MainReady,

    /// <summary>The step which activates all triggers
    /// listening on 'start of turn' events.
    /// </summary>
    TurnStartTriggers,

    /// <summary>The step where available resources (mana)
    /// are calculated.</summary>
    TurnResource,

    /// <summary>The step where the current player draws
    /// his card(s) for this turn.</summary>
    TurnDraw,

    /// <summary>Intermediate step to process all queued
    /// game operations before the player
    /// can perform actions.</summary>
    TurnStart,

    /// <summary>The step which allows the current player
    /// to give the game <see cref="ITask"/>s
    /// to process.</summary>
    TurnAction,

    /// <summary>The step where an <see cref="ICharacter"/>
    /// attacks another
    /// <see cref="ICharacter"/>.</summary>
    TurnCombat,

    /// <summary>The step to process queued game operations.
    /// Mostly manually called after an operation during
    /// <see cref="TURN_ACTION"/> has
    /// finished processing.
    /// </summary>
    TurnProcess,

    /// <summary>The step which activates triggers waiting
    /// for the 'end turn' event.
    /// </summary>
    TurnEndTriggers,

    /// <summary>The step which changes the current player
    /// and completes entity information.
    /// </summary>
    MainNext,

    /// <summary>The step used to calculate winners/losers
    /// and complete entity information.
    /// </summary>
    FinalWrapup,

    /// <summary>The step which marks the end of the
    /// game.</summary>
    FinalGameover,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EFormats {
    Wild,
    Standard,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Primitive)]
#[repr(u32)]
pub enum EZones {
    Play = 1,
    Deck = 2,
    Hand = 3,
    Graveyard = 4,
    Removedfromgame = 5,
    Setaside = 6,
    Secret = 7,
}

// TODO; Remove integers
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EPlayRequirements {
    ReqMinionTarget = 1,
    ReqFriendlyTarget = 2,
    ReqEnemyTarget = 3,
    ReqDamagedTarget = 4,
    ReqMaxSecrets = 5,
    ReqFrozenTarget = 6,
    ReqChargeTarget = 7,
    ReqTargetMaxAttack = 8,
    ReqNonselfTarget = 9,
    ReqTargetWithRace = 10,
    ReqTargetToPlay = 11,
    ReqNumMinionSlots = 12,
    ReqWeaponEquipped = 13,
    ReqEnoughMana = 14,
    ReqYourTurn = 15,
    ReqNonstealthEnemyTarget = 16,
    ReqHeroTarget = 17,
    ReqSecretZoneCap = 18,
    ReqMinionCapIfTargetAvailable = 19,
    ReqMinionCap = 20,
    ReqTargetAttackedThisTurn = 21,
    ReqTargetIfAvailable = 22,
    ReqMinimumEnemyMinions = 23,
    ReqTargetForCombo = 24,
    ReqNotExhaustedActivate = 25,
    ReqUniqueSecretOrQuest = 26,
    ReqTargetTaunter = 27,
    ReqCanBeAttacked = 28,
    ReqActionPwrIsMasterPwr = 29,
    ReqTargetMagnet = 30,
    ReqAttackGreaterThan0 = 31,
    ReqAttackerNotFrozen = 32,
    ReqHeroOrMinionTarget = 33,
    ReqCanBeTargetedBySpells = 34,
    ReqSubcardIsPlayable = 35,
    ReqTargetForNoCombo = 36,
    ReqNotMinionJustPlayed = 37,
    ReqNotExhaustedHeroPower = 38,
    ReqCanBeTargetedByOpponents = 39,
    ReqAttackerCanAttack = 40,
    ReqTargetMinAttack = 41,
    ReqCanBeTargetedByHeroPowers = 42,
    ReqEnemyTargetNotImmune = 43,
    ReqEntireEntourageNotInPlay = 44,
    ReqMinimumTotalMinions = 45,
    ReqMustTargetTaunter = 46,
    ReqUndamagedTarget = 47,
    ReqCanBeTargetedByBattlecries = 48,
    ReqSteadyShot = 49,
    ReqMinionOrEnemyHero = 50,
    ReqTargetIfAvailableAndDragonInHand = 51,
    ReqLegendaryTarget = 52,
    ReqFriendlyMinionDiedThisTurn = 53,
    ReqFriendlyMinionDiedThisGame = 54,
    ReqEnemyWeaponEquipped = 55,
    ReqTargetIfAvailableAndMinimumFriendlyMinions = 56,
    ReqTargetWithBattlecry = 57,
    ReqTargetWithDeathrattle = 58,
    ReqTargetIfAvailableAndMinimumFriendlySecrets = 59,
    ReqSecretZoneCapForNonSecret = 60,
    ReqTargetExactCost = 61,
    ReqStealthedTarget = 62,
    ReqMinionSlotOrManaCrystalSlot = 63,
    ReqMaxQuests = 64,
    ReqTargetIfAvailabeAndElementalPlayedLastTurn = 65,
    ReqTargetNotVampire = 66,
    ReqTargetNotDamageableOnlyByWeapons = 67,
    ReqNotDisabledHeroPower = 68,
    ReqMustPlayOtherCardFirst = 69,
    ReqHandNotFull = 70,
    ReqDragToPlay = 71,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Primitive)]
pub enum EGameTags {
    IgnoreDamage = 1,
    TagScriptDataNum1 = 2,
    TagScriptDataNum2 = 3,
    TagScriptDataEnt1 = 4,
    TagScriptDataEnt2 = 5,
    MissionEvent = 6,
    Timeout = 7,
    TurnStart = 8,
    TurnTimerSlush = 9,
    // 10 GameTag, seems set to 85 always seconds?
    // 11 MinionTag, Controller, ??? used with Potion of Madness  (11 newController,Charge,333 oldController)
    Premium = 12,
    GoldRewardState = 13,
    Playstate = 17,
    LastAffectedBy = 18,
    Step = 19,
    Turn = 20,
    Fatigue = 22,
    CurrentPlayer = 23,
    FirstPlayer = 24,
    ResourcesUsed = 25,
    Resources = 26,
    HeroEntity = 27,
    Maxhandsize = 28,
    Starthandsize = 29,
    PlayerID = 30,
    TeamID = 31,
    TriggerVisual = 32,
    RecentlyArrived = 33,
    Protected = 34,
    Protecting = 35,
    Defending = 36,
    ProposedDefender = 37,
    Attacking = 38,
    ProposedAttacker = 39,
    Attached = 40,
    Exhausted = 43,
    Damage = 44,
    Health = 45,
    Atk = 47,
    Cost = 48,
    Zone = 49,
    Controller = 50,
    Owner = 51,
    Definition = 52,
    EntityID = 53,
    HistoryProxy = 54,
    CopyDeathrattle = 55,
    CopyDeathrattleIndex = 56,
    Elite = 114,
    Maxresources = 176,
    CardSet = 183,
    CardtextInhand = 184,
    Cardname = 185,
    CardID = 186,
    Durability = 187,
    Silenced = 188,
    Windfury = 189,
    Taunt = 190,
    Stealth = 191,
    Spellpower = 192,
    DivineShield = 194,
    Charge = 197,
    NextStep = 198,
    Class = 199,
    Cardrace = 200,
    Faction = 201,
    Cardtype = 202,
    Rarity = 203,
    State = 204,
    Summoned = 205,
    Freeze = 208,
    Enraged = 212,
    Overload = 215,
    Loyalty = 216,
    Deathrattle = 217,
    Battlecry = 218,
    Secret = 219,
    Combo = 220,
    CantHeal = 221,
    CantDamage = 222,
    CantSetAside = 223,
    CantRemoveFromGame = 224,
    CantReady = 225,
    CantExhaust = 226,
    CantAttack = 227,
    CantTarget = 228,
    CantDestroy = 229,
    CantDiscard = 230,
    CantPlay = 231,
    CantDraw = 232,
    IncomingHealingMultiplier = 233,
    IncomingHealingAdjustment = 234,
    IncomingHealingCap = 235,
    IncomingDamageMultiplier = 236,
    IncomingDamageAdjustment = 237,
    IncomingDamageCap = 238,
    CantBeHealed = 239,
    Immune = 240,
    CantBeSetAside = 241,
    CantBeRemovedFromGame = 242,
    CantBeReadied = 243,
    CantBeExhausted = 244,
    CantBeAttacked = 245,
    CantBeTargeted = 246,
    CantBeDestroyed = 247,
    AttackVisualType = 251,
    CardTextInPlay = 252,
    CantBeSummoningSick = 253,
    Frozen = 260,
    JustPlayed = 261,
    LinkedEntity = 262,
    ZonePosition = 263,
    CantBeFrozen = 264,
    ComboActive = 266,
    CardTarget = 267,
    DevState = 268,
    NumCardsPlayedThisTurn = 269,
    CantBeTargetedByOpponents = 270,
    NumTurnsInPlay = 271,
    NumTurnsLeft = 272,
    OutgoingDamageCap = 273,
    OutgoingDamageAdjustment = 274,
    OutgoingDamageMultiplier = 275,
    OutgoingHealingCap = 276,
    OutgoingHealingAdjustment = 277,
    OutgoingHealingMultiplier = 278,
    IncomingAbilityDamageAdjustment = 279,
    IncomingCombatDamageAdjustment = 280,
    OutgoingAbilityDamageAdjustment = 281,
    OutgoingCombatDamageAdjustment = 282,
    OutgoingAbilityDamageMultiplier = 283,
    OutgoingAbilityDamageCap = 284,
    IncomingAbilityDamageMultiplier = 285,
    IncomingAbilityDamageCap = 286,
    OutgoingCombatDamageMultiplier = 287,
    OutgoingCombatDamageCap = 288,
    IncomingCombatDamageMultiplier = 289,
    IncomingCombatDamageCap = 290,
    CurrentSpellpower = 291,
    Armor = 292,
    Morph = 293,
    IsMorphed = 294,
    TempResources = 295,
    OverloadOwed = 296,
    NumAttacksThisTurn = 297,
    NextAllyBuff = 302,
    Magnet = 303,
    FirstCardPlayedThisTurn = 304,
    MulliganState = 305,
    TauntReady = 306,
    StealthReady = 307,
    ChargeReady = 308,
    CantBeTargetedBySpells = 311,
    Shouldexitcombat = 312,
    Creator = 313,
    CantBeSilenced = 314,
    ParentCard = 316,
    NumMinionsPlayedThisTurn = 317,
    Predamage = 318,
    Collectible = 321,
    // 323 EnchantmentTag, true ... when weapon equipped
    // 324 EnchantmentTag
    TargetingArrowText = 325,
    EnchantmentBirthVisual = 330,
    EnchantmentIdleVisual = 331,
    CantBeTargetedByHeroPowers = 332,
    // 333 MinionTag, turns till effect? Or controller change after turn?
    Weapon = 334,
    InvisibleDeathrattle = 335,
    HealthMinimum = 337,
    TagOneTurnEffect = 338,
    Silence = 339,
    Counter = 340,
    // 341 MinionTag, true/false
    Artistname = 342,
    LocalizationNotes = 344,
    HandRevealed = 348,
    ImmuneToSpellpower = 349,
    AdjacentBuff = 350,
    Flavortext = 351,
    ForcedPlay = 352,
    LowHealthThreshold = 353,
    IgnoreDamageOff = 354,
    GrantCharge = 355,
    SpellpowerDouble = 356,
    HealingDouble = 357,
    NumOptionsPlayedThisTurn = 358,
    NumOptions = 359,
    ToBeDestroyed = 360,
    HealTarget = 361,
    Aura = 362,
    Poisonous = 363,
    HowToEarn = 364,
    HowToEarnGolden = 365,
    HeroPowerDouble = 366,
    AiMustPlay = 367,
    NumMinionsPlayerKilledThisTurn = 368,
    NumMinionsKilledThisTurn = 369,
    AffectedBySpellPower = 370,
    ExtraDeathrattles = 371,
    StartWith1Health = 372,
    ImmuneWhileAttacking = 373,
    MultiplyHeroDamage = 374,
    MultiplyBuffValue = 375,
    CustomKeywordEffect = 376,
    Topdeck = 377,
    CantBeTargetedByBattlecries = 379,
    HeroPower = 380,
    DeathrattleReturnZone = 382,
    SteadyShotCanTarget = 383,
    DisplayedCreator = 385,
    PoweredUp = 386,
    SparePart = 388,
    Forgetful = 389,
    CanSummonMaxplusoneMinion = 390,
    Obfuscated = 391,
    Burning = 392,
    OverloadLocked = 393,
    NumTimesHeroPowerUsedThisGame = 394,
    CurrentHeropowerDamageBonus = 395,
    HeropowerDamage = 396,
    LastCardPlayed = 397,
    NumFriendlyMinionsThatDiedThisTurn = 398,
    NumCardsDrawnThisTurn = 399,
    AiOneShotKill = 400,
    EvilGlow = 401,
    HideStats = 402,
    Inspire = 403,
    ReceivesDoubleSpelldamageBonus = 404,
    HeropowerAdditionalActivations = 405,
    HeropowerActivationsThisTurn = 406,
    Revealed = 410,
    // 411 IMPLEMENTED
    NumFriendlyMinionsThatDiedThisGame = 412,
    CannotAttackHeroes = 413,
    LockAndLoad = 414,
    Discover = 415,
    Shadowform = 416,
    NumFriendlyMinionsThatAttackedThisTurn = 417,
    NumResourcesSpentThisGame = 418,
    ChooseBoth = 419,
    ElectricChargeLevel = 420,
    HeavilyArmored = 421,
    DontShowImmune = 422,
    Ritual = 424,
    Prehealing = 425,
    AppearFunctionallyDead = 426,
    OverloadThisGame = 427,
    // 430 IMPLEMENTED
    SpellsCostHealth = 431,
    HistoryProxyNoBigCard = 432,
    ProxyCthun = 434,
    TransformedFromCard = 435,
    Cthun = 436,
    CastRandomSpells = 437,
    Shifting = 438,
    JadeGolem = 441,
    EmbraceTheShadow = 442,
    ChooseOne = 443,
    ExtraAttacksThisTurn = 444,
    SeenCthun = 445,
    MinionTypeReference = 447,
    Untouchable = 448,
    RedManaCrystals = 449,
    ScoreLabelid1 = 450,
    ScoreValue1 = 451,
    ScoreLabelid2 = 452,
    ScoreValue2 = 453,
    ScoreLabelid3 = 454,
    ScoreValue3 = 455,
    CantBeFatigued = 456,
    Autoattack = 457,
    ArmsDealing = 458,
    PendingEvolutions = 461,
    Quest = 462,
    TagLastKnownCostInHand = 466,
    // 467 IMPLEMENTED
    // 468 IMPLEMENTED
    DefiningEnchantment = 469,
    FinishAttackSpellOnDamage = 470,
    ModularEntityPart1 = 471,
    ModularEntityPart2 = 472,
    ModifyDefinitionAttack = 473,
    ModifyDefinitionHealth = 474,
    ModifyDefinitionCost = 475,
    MultipleClasses = 476,
    AllTargetsRandom = 477,
    // 478, true on quest cards ???
    // 479 IMPLEMENTED
    MultiClassGroup = 480,
    CardCostsHealth = 481,
    GrimyGoons = 482,
    JadeLotus = 483,
    Kabal = 484,
    AdditionalPlayReqs1 = 515,
    AdditionalPlayReqs2 = 516,
    ElementalPoweredUp = 532,
    QuestProgress = 534,
    QuestProgressTotal = 535,
    QuestContributor = 541,
    Adapt = 546,
    IsCurrentTurnAnExtraTurn = 547,
    ExtraTurnsTakenThisGame = 548,
    ShiftingMinion = 549,
    ShiftingWeapon = 550,
    DeathKnight = 554,
    Boss = 556,
    Stampede = 564,
    // 676, true on quests cards ???
    IsVampire = 680,
    Corrupted = 681,
    Lifesteal = 685,
    OverrideEmote0 = 740,
    OverrideEmote1 = 741,
    OverrideEmote2 = 742,
    OverrideEmote3 = 743,
    OverrideEmote4 = 744,
    OverrideEmote5 = 745,
    ScoreFooterid = 751,
    HeroPowerDisabled = 777,
    Valeerashadow = 779,
    Overridecardname = 781,
    Overridecardtextbuilder = 782,
    HiddenChoice = 813,
    Zombeast = 823,
    // more or less guessed gametags
    ExtraBattlecry = 411, // ControllerTag, true/false Bronzebard Extra Battlecry
    NumSpellsPlayedThisTurn = 430, // need to implement it ^^
    NumCardsToDraw = 467,
    MoatLurkerMinion = 468,
    TagLastKnownAtkInHand = 479, // ???
    NumSpellsPlayedThisGame = 1001,
    NumSecretsPlayedThisGame = 1002,
    NumWeaponsPlayedThisGame = 1003,
    LastCardDrawn = 1004,
    LastCardDiscarded = 1005,
    NumElementalPlayedThisTurn = 1006,
    NumElementalPlayedLastTurn = 1007,
    NumMurlocsPlayedThisGame = 1008,
    TagLastKnownPositionOnBoard = 1009, // position aren't changed in graveyard and setaside zone ??? obolet?
}
