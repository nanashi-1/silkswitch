use crate::values::generic_value::GenericType;

pub struct GenericValueDesc<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub generic_type: GenericType,
    pub pointer: &'a str,
    pub priority: u64,
}

const fn new(
    title: &'static str,
    description: &'static str,
    generic_type: GenericType,
    pointer: &'static str,
    priority: u64,
) -> GenericValueDesc<'static> {
    GenericValueDesc {
        title,
        description,
        generic_type,
        pointer,
        priority,
    }
}

pub const GENERIC_VALUE_DESC: &[GenericValueDesc] = &[
    new(
        "Rosaries",
        "Amount of rosaries Hornet currently holds.",
        GenericType::Int,
        "/playerData/geo",
        1,
    ),
    new(
        "Health",
        "Hornet's current health.",
        GenericType::Int,
        "/playerData/health",
        2,
    ),
    new(
        "Max Health",
        "Hornet's maximum health.",
        GenericType::Int,
        "/playerData/maxHealth",
        2,
    ),
    new(
        "Silk",
        "Maximum amount of silk Hornet can hold.",
        GenericType::Int,
        "/playerData/silk",
        2,
    ),
    new(
        "Has Swift Step",
        "Whether Hornet acquired dash ability.",
        GenericType::Boolean,
        "/playerData/hasDash",
        3,
    ),
    new(
        "Has Drifter's Cloak",
        "Whether Hornet acquired the cloak.",
        GenericType::Boolean,
        "/playerData/hasBrolly",
        3,
    ),
    new(
        "Has Cling Grip",
        "Whether Hornet acquired the ability to jump off walls.",
        GenericType::Boolean,
        "/playerData/hasWalljump",
        3,
    ),
    new(
        "Has Faydown Cloak",
        "Whether Hornet acquired the ability to jump mid-air and survive in the harsh weather of Mount Fay.",
        GenericType::Boolean,
        "/playerData/hasDoubleJump",
        3,
    ),
    new(
        "Has Quill",
        "Whether Hornet owns the Quill for mapping.",
        GenericType::Boolean,
        "/playerData/hasQuill",
        4,
    ),
    new(
        "Has Needle Strike",
        "Whether Hornet unlocked Charge Slash.",
        GenericType::Boolean,
        "/playerData/hasChargeSlash",
        3,
    ),
    new(
        "Has Silk Soar",
        "Whether Hornet acquired super jump.",
        GenericType::Boolean,
        "/playerData/hasSuperJump",
        3,
    ),
    new(
        "Has Killed",
        "Whether Hornet has defeated the target enemy or boss.",
        GenericType::Boolean,
        "/playerData/hasKilled",
        5,
    ),
    new(
        "Is Invincible",
        "Whether Hornet is currently under godmode/invincibility status.",
        GenericType::Boolean,
        "/playerData/isInvincible",
        5,
    ),
    new(
        "Shell Shards",
        "Number of collected Shell Shards.",
        GenericType::Int,
        "/playerData/ShellShards",
        1,
    ),
];
