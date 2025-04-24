use core::GameBoyAdvance;

use tokio::sync::broadcast;

use super::pkmn_fire_red;
#[derive(Clone)]
pub enum EmulatorQuery {
    Frame(Vec<u8>),
    Memory(serde_json::Value),
}

fn translate_frame_to_u8(input_fb: &[u32], out_fb: &mut [u8]) {
    for (i, &color) in input_fb.iter().enumerate() {
        out_fb[4 * i + 0] = ((color >> 16) & 0xff) as u8;
        out_fb[4 * i + 1] = ((color >> 8) & 0xff) as u8;
        out_fb[4 * i + 2] = (color & 0xff) as u8;
        out_fb[4 * i + 3] = 255;
    }
}

pub fn emit_frame(gba: &mut GameBoyAdvance, sender: &broadcast::Sender<EmulatorQuery>) {
    let mut frame_buf = vec![0u8; 240 * 160 * 4];
    translate_frame_to_u8(gba.get_frame_buffer(), &mut frame_buf);
    let _ = sender.send(EmulatorQuery::Frame(frame_buf.clone()));
}

pub fn emit_ram(gba: &mut GameBoyAdvance, sender: &broadcast::Sender<EmulatorQuery>) {
    let eram = gba.get_external_wram();
    let parsed = pkmn_fire_red::memory::memory::parse_external_memory(&eram);
    let _ = sender.send(EmulatorQuery::Memory(parsed));
}
