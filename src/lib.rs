pub mod chip8 {
    pub const DISPLAY_HEIGHT: usize = 32;
    pub const DISPLAY_WIDTH: usize = 64;

    const MEMORY_SIZE: usize = 4096;
    const V_SIZE: usize = 16;
    const STACK_SIZE: usize = 16;    
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

        pub fn pixel_at(&self, x: usize, y: usize) -> bool {
            assert!(x < DISPLAY_WIDTH);
            assert!(y < DISPLAY_HEIGHT);

            self.display[DISPLAY_HEIGHT - y - 1][x]
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn constructor() {
            let chip8 = Emulator::new();
            // Program counter
            assert_eq!(
                chip8.program_counter, 0x200,
                "Program counter should be set to 0x200 rather than 0x{:X}",
                chip8.program_counter
            );
            // Memory
            assert_eq!(chip8.memory.len(), 4096);
            assert_eq!(chip8.display.len(), 32);
            assert_eq!(chip8.display[0].len(), 64);
            // V
            assert_eq!(chip8.v.len(), 16);
            for index in 0..16 {
                assert_eq!(chip8.v[index], 0);
            }
            // Stack
            assert_eq!(chip8.stack_pointer, 0);
            assert_eq!(chip8.stack.len(), 16);
            for index in 0..16 {
                assert_eq!(chip8.stack[index], 0);
            }
            // Display
            for x in 0..64 {
                for y in 0..32 {
                    assert_eq!(chip8.pixel_at(x, y), false);
                }
            }
        }
    }
}
