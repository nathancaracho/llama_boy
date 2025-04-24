use core::{
    GameBoyAdvance,
    prelude::{Cartridge, GamepakBuilder, NullAudio},
};
use std::{fs::File, io::Read};

fn load_local_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn get_rom() -> Cartridge {
    let rom = load_local_file(
        "/Users/nathancaracho/Documents/projects/rust/llama_boy/external/roms/pkmnv1.gba",
    );
    GamepakBuilder::new()
        .take_buffer(rom.into_boxed_slice())
        .without_backup_to_file()
        .build()
        .unwrap()
}

pub fn from_local_builder() -> anyhow::Result<GameBoyAdvance> {
    let save_state = load_local_file(
        "/Users/nathancaracho/Documents/projects/rust/llama_boy/external/roms/llamaboy.savestate",
    );
    let bios = load_local_file(
        "/Users/nathancaracho/Documents/projects/rust/llama_boy/external/bios/gba_bios.bin",
    );
    let rom = load_local_file(
        "/Users/nathancaracho/Documents/projects/rust/llama_boy/external/roms/pkmnv1.gba",
    );
    let gamepak = get_rom();
    let gba = GameBoyAdvance::from_saved_state(
        &save_state,
        bios.into_boxed_slice(),
        rom.into_boxed_slice(),
        NullAudio::new(),
    )?;
    Ok(gba)
}
