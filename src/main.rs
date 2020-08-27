mod emulator;

use std::{
    env,
    fs::File,
    io::Read,
};

use crate::emulator::Emulator;

// TODO: pausing doesn't work, the Event doesn't trigger. remove that functionality

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut emulator = Emulator::new();
    let mut file = File::open(&args[1]).expect("Unable to open the ROM");
    let mut rom = [0u8; 0xE00];
    file.read(&mut rom).unwrap();
    emulator.load(&rom);
    emulator.run();
}
