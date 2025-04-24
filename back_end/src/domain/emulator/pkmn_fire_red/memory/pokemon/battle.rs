use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BattleType {
    OutOfBattle,
    VersusTrainer,
    WildPokemon,
    WildPokemonAltEntry,
    SpecialEvent,
    Scripted,
    Unknown(u32),
}

impl BattleType {
    pub fn from_raw(val: u32) -> Self {
        match val {
            0 => Self::OutOfBattle,
            1 => Self::VersusTrainer,
            2 => Self::WildPokemon,
            4 => Self::WildPokemonAltEntry,
            5 => Self::SpecialEvent,
            9 => Self::Scripted,
            other => Self::Unknown(other),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OutOfBattle => "out_of_battle",
            Self::VersusTrainer => "versus_trainer",
            Self::WildPokemon => "wild_pokemon",
            Self::WildPokemonAltEntry => "wild_pokemon_alt_entry",
            Self::SpecialEvent => "special_event",
            Self::Scripted => "scripted",
            Self::Unknown(_) => "unknown",
        }
    }
}
