use std::default;

use serde::Serialize;

use crate::domain::emulator::pkmn_fire_red::memory::{
    pokemon::{
        list::species_name,
        moves::{Move, get_move_by_id},
    },
    read::{gba_offset, read_u8, read_u16, read_u32},
};

const SUB_ORDER: [[u8; 4]; 24] = [
    [0, 1, 2, 3],
    [0, 1, 3, 2],
    [0, 2, 1, 3],
    [0, 2, 3, 1],
    [0, 3, 1, 2],
    [0, 3, 2, 1],
    [1, 0, 2, 3],
    [1, 0, 3, 2],
    [1, 2, 0, 3],
    [1, 2, 3, 0],
    [1, 3, 0, 2],
    [1, 3, 2, 0],
    [2, 0, 1, 3],
    [2, 0, 3, 1],
    [2, 1, 0, 3],
    [2, 1, 3, 0],
    [2, 3, 0, 1],
    [2, 3, 1, 0],
    [3, 0, 1, 2],
    [3, 0, 2, 1],
    [3, 1, 0, 2],
    [3, 1, 2, 0],
    [3, 2, 0, 1],
    [3, 2, 1, 0],
];
#[derive(Debug, Clone, Copy, Serialize)]
pub struct StatSpread {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spa: u8,
    pub spd: u8,
    pub spe: u8,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct MoveSlot {
    pub move_id: u16,
    pub pp: u8,
    pub pp_used: u8,
    pub move_item: Option<Move>,
}

#[derive(Debug, Serialize)]
pub struct PartyPokemon {
    pub personality: u32,
    pub species_id: u16,
    pub species: String,
    pub level: u8,
    pub status: u32,
    pub hp: u16,
    pub max_hp: u16,
    pub stats: StatSpread,

    pub item: u16,
    pub exp: u32,
    pub friendship: u8,
    pub nature: u8,
    pub ivs: StatSpread,
    pub evs: StatSpread,
    pub ability_1: bool,
    pub pokerus: u8,

    pub moves: [MoveSlot; 4],
    pub pp_bonuses: u8,
    pub is_battle: bool,
}
const G_BATTLER_PARTY_INDEXES: usize = 0x02023BCE; // EWRAM – FireRed

pub fn mark_active_mon(party: &mut [PartyPokemon], buf: &[u8]) {
    // 1. limpa tudo
    for p in party.iter_mut() {
        p.is_battle = false;
    }

    // 2. lê o slot ativo do jogador  ➜ byte 0
    let idx_player = read_u8(buf, gba_offset(G_BATTLER_PARTY_INDEXES)) as usize;

    // 3. marca só esse
    if idx_player < party.len() {
        party[idx_player].is_battle = true;
    }
}

/// Lê um dos quatro slots de ataque (idx 0-3) a partir do sub-bloco *Attacks*.
fn read_move_slot(buf: &[u8], attacks_base: usize, idx: usize, key: u32) -> MoveSlot {
    assert!(idx < 4, "idx fora do intervalo 0..4");

    /* -------- MOVES (u16) -------- */
    // 0/1 estão no word0 (offset +0), 2/3 no word1 (offset +4)
    let moves_word = read_u32(buf, attacks_base + if idx < 2 { 0 } else { 4 }) ^ key;

    let move_id = if idx & 1 == 0 {
        (moves_word & 0xFFFF) as u16 // metade baixa
    } else {
        (moves_word >> 16) as u16 // metade alta
    };

    /* -------- PP (u8) -------- */
    // PP para os 4 slots fica no word em +8
    let pp_word = read_u32(buf, attacks_base + 8) ^ key;
    let pp = ((pp_word >> (idx * 8)) & 0xFF) as u8;

    /* -------- PP Ups (2 bits) -------- */
    // Cada PP Up ocupa 2 bits; ainda assim só precisamos do valor 0-3
    let ups_word = read_u32(buf, attacks_base + 12) ^ key;
    let pp_used = ((ups_word >> (idx * 8)) & 0x03) as u8;

    MoveSlot {
        move_id,
        pp,
        pp_used,
        move_item: get_move_by_id(move_id),
    }
}

pub fn get_party_at(buf: &[u8], base_addr: usize) -> Vec<PartyPokemon> {
    const SLOT: usize = 100;
    const ENC_START: usize = 0x20;
    const SUB: usize = 12;

    let mut v = Vec::new();
    let base = gba_offset(base_addr);

    let current_species = read_u16(buf, gba_offset(0x0202402C));
    let current_level = read_u8(buf, gba_offset(0x0202402C + 0x1C));
    let current_hp = read_u16(buf, gba_offset(0x0202402C + 0x20));

    for i in 0..6 {
        let off = base + i * SLOT;

        let personality = read_u32(buf, off);
        let ot_id = read_u32(buf, off + 4);
        let key = personality ^ ot_id;
        let order = SUB_ORDER[(personality % 24) as usize];

        let ofs = |sub| off + ENC_START + order.iter().position(|&x| x == sub).unwrap() * SUB;

        let growth = ofs(0);
        let attacks = ofs(1);
        let evs = ofs(2);
        let misc = ofs(3);

        let species = (read_u32(buf, growth) ^ key) as u16;
        if species == 0 {
            continue;
        }

        let word0 = read_u32(buf, growth) ^ key;
        let word1 = read_u32(buf, growth + 4) ^ key;
        let word2 = read_u32(buf, growth + 8) ^ key;

        let item = (word0 >> 16) as u16;
        let exp = word1;
        let pp_bonuses = (word2 & 0xFF) as u8;
        let friendship = ((word2 >> 8) & 0xFF) as u8;

        let moves = [
            read_move_slot(buf, attacks, 0, key),
            read_move_slot(buf, attacks, 1, key),
            read_move_slot(buf, attacks, 2, key),
            read_move_slot(buf, attacks, 3, key),
        ];

        let ev = |k| read_u8(buf, evs + k) ^ (key as u8);
        let evs = StatSpread {
            hp: ev(0),
            atk: ev(1),
            def: ev(2),
            spe: ev(3),
            spa: ev(4),
            spd: ev(5),
        };

        let iv_word = read_u32(buf, misc) ^ key;
        let iv = |shift: u8| -> u8 { ((iv_word >> shift) & 0x1F) as u8 };
        let ivs = StatSpread {
            hp: iv(0),
            atk: iv(5),
            def: iv(10),
            spe: iv(15),
            spa: iv(20),
            spd: iv(25),
        };
        let ability_1 = (iv_word >> 31) != 0;
        let pokerus = read_u8(buf, misc + 8) ^ (key as u8);

        let status = read_u32(buf, off + 0x50);
        let level = read_u8(buf, off + 0x54);
        let hp = read_u16(buf, off + 0x56);
        let maxhp = read_u16(buf, off + 0x58);
        let atk = read_u16(buf, off + 0x5A);
        let def = read_u16(buf, off + 0x5C);
        let spe = read_u16(buf, off + 0x5E);
        let spa = read_u16(buf, off + 0x60);
        let spd = read_u16(buf, off + 0x62);

        let is_battle = species == current_species && level == current_level && hp == current_hp;

        v.push(PartyPokemon {
            personality,
            species_id: species,
            species: species_name(species).to_string(),
            level,
            status,
            hp,
            max_hp: maxhp,
            stats: StatSpread {
                hp: 0,
                atk: (atk as u8),
                def: (def as u8),
                spa: (spa as u8),
                spd: (spd as u8),
                spe: (spe as u8),
            },
            item,
            exp,
            friendship,
            nature: (personality % 25) as u8,
            ivs,
            evs,
            ability_1,
            pokerus,
            moves,
            pp_bonuses,
            is_battle,
        });
    }
    mark_active_mon(&mut v, buf);
    v
}
