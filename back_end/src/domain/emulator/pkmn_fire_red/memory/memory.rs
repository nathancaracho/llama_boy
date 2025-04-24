use serde::Serialize;
use serde_json::{Value, json};
use tracing::info;

use crate::domain::emulator::pkmn_fire_red::memory::{
    pokemon::{battle::BattleType, party::get_party_at},
    read::{read_u8, read_u32},
};

pub fn parse_external_memory(buf: &[u8]) -> serde_json::Value {
    use serde_json::json;

    json!({
        "player_party":get_party_at(buf, 0x02024284),
        "enemy_party":get_party_at(buf, 0x0202402C),
        "battle_type": BattleType::from_raw(read_u32(buf, 0x02022B4C - 0x02000000)).as_str()

    })
}
