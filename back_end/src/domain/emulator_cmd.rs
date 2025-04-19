use core::{
    GameBoyAdvance,
    prelude::{Cartridge, GamepakBuilder, NullAudio},
};

use std::{fs::File, io::Read, thread, time::Duration};

use tokio::sync::broadcast;

pub fn load_local_file(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

fn get_rom() -> Cartridge {
    let rom = load_local_file(
        "/Users/nathancaracho/Documents/projects/rust/llama_boy/external/roms/pkm.gba",
    );
    GamepakBuilder::new()
        .take_buffer(rom.into_boxed_slice())
        .without_backup_to_file()
        .build()
        .unwrap()
}

/// Traduz a frame do GBA (u32) para RGBA (u8)
fn translate_frame_to_u8(input_fb: &[u32], out_fb: &mut [u8]) {
    for (i, &color) in input_fb.iter().enumerate() {
        out_fb[4 * i + 0] = ((color >> 16) & 0xff) as u8;
        out_fb[4 * i + 1] = ((color >> 8) & 0xff) as u8;
        out_fb[4 * i + 2] = (color & 0xff) as u8;
        out_fb[4 * i + 3] = 255;
    }
}

/// Spawna a thread do emulador que envia frames para o canal broadcast
pub fn spawn_emulator_task(tx: broadcast::Sender<Vec<u8>>) -> anyhow::Result<()> {
    thread::spawn(move || {
        let bios = load_local_file(
            "/Users/nathancaracho/Documents/projects/rust/llama_boy/external/bios/gba_bios.bin",
        );
        let gamepak = get_rom();
        let mut gba = GameBoyAdvance::from_local_files();
        let mut frame_buf = vec![0u8; 240 * 160 * 4];

        loop {
            gba.frame();
            translate_frame_to_u8(gba.get_frame_buffer(), &mut frame_buf);

            // Envia pro canal (ignora erro se não tiver ninguém ouvindo)
            let _ = tx.send(frame_buf.clone());

            thread::sleep(Duration::from_millis(16)); // ~60fps
        }
    });

    Ok(())
}
