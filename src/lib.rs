pub mod chip8 {
    const MEMORY_SIZE: usize = 4096;
    const V_SIZE: usize = 16;
    const STACK_SIZE: usize = 16;

    const DISPLAY_WIDTH: usize = 64;
    const DISPLAY_HEIGHT: usize = 32;
    const SPRITES: [u8; 16 * 5] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    pub struct Emulator {
        program_counter: u16,
        memory: [u8; MEMORY_SIZE],
        v: [u8; V_SIZE],
        stack_pointer: u8,
        stack: [u16; STACK_SIZE],
        // TODO: Set up keyboard bindings.
        display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    }

    impl Emulator {
        pub fn new() -> Emulator {
            let mut memory = [0; MEMORY_SIZE];
            memory[..0x50].clone_from_slice(&SPRITES[..0x50]);
            Emulator {
                program_counter: 0x200,
                memory: memory,
                v: [0; V_SIZE],
                stack_pointer: 0x0,
                stack: [0; STACK_SIZE],
                display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            }
        }

        pub fn instruction_cycle(&mut self) {
            // Fetch
            self.program_counter += 2;
            // Decode
            // Execute
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test() {}
    }
}
