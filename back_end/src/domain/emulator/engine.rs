use super::commands::emmit_command;
use super::queries::{EmulatorQuery, emit_frame, emit_ram};
use crate::infrastructure::emulator::builder::from_local_builder;
use core::keypad;
use std::fs::File;

use std::io::Write;
use std::{thread, time::Duration};
use tokio::sync::broadcast;
use tokio::sync::mpsc::{self, Receiver};
pub fn start_emulator_loop(
    sender: broadcast::Sender<EmulatorQuery>,
    mut receiver: Receiver<keypad::Keys>,
) -> anyhow::Result<()> {
    thread::spawn(move || {
        let mut gba = from_local_builder().unwrap();
        let mut frames = 0;
        let mut save_state = 0;
        loop {
            gba.frame();

            emmit_command(&mut gba, &mut receiver);
            emit_frame(&mut gba, &sender);
            if frames == 10 {
                emit_ram(&mut gba, &sender);
                frames = 0;
            }
            if save_state == 1000 {
                match gba.save_state() {
                    Ok(bytes) => {
                        let _ = File::create("llamaboy.savestate")
                            .and_then(|mut f| f.write_all(&bytes));
                        println!("Savestate saved!");
                    }
                    Err(e) => {
                        eprintln!("Failed to save state: {:?}", e);
                    }
                }
                save_state = 0;
            }
            frames += 1;
            save_state += 1;
            thread::sleep(Duration::from_millis(16)); // ~60fps
        }
    });
    Ok(())
}
