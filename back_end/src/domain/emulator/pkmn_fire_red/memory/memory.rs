use serde::Serialize;
use serde_json::{Value, json};

use crate::domain::emulator::pkmn_fire_red::memory::pokemon::party::get_party_at;

pub fn parse_external_memory(buf: &[u8]) -> serde_json::Value {
    use serde_json::json;

    json!({
        "player_party":get_party_at(buf, 0x02024284),
        "enemy_party":get_party_at(buf, 0x0202402C),
    })
}
