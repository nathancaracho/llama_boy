use bit::BitIndex;
use core::GameBoyAdvance;
use core::keypad;
use tokio::sync::mpsc::Receiver;

pub fn emmit_command(gba: &mut GameBoyAdvance, receiver: &mut Receiver<keypad::Keys>) {
    if let Ok(key) = receiver.try_recv() {
        gba.get_key_state_mut().set_bit(key as usize, false);
        for _ in 0..3 {
            gba.frame();
        }
        gba.get_key_state_mut().set_bit(key as usize, true);
    }
}
