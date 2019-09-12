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
        i: u16,
        stack_pointer: u8,
        stack: [u16; STACK_SIZE],
        // TODO: Set up keyboard bindings
        display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
        // TODO: Sound & delay timers
    }

    impl Emulator {
        pub fn new() -> Self {
            let mut memory = [0; MEMORY_SIZE];
            memory[..0x50].clone_from_slice(&SPRITES[..0x50]);
            Emulator {
                program_counter: 0x200,
                memory,
                i: 0,
                v: [0; V_SIZE],
                stack_pointer: 0x0,
                stack: [0; STACK_SIZE],
                display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            }
        }

        /// The instructions are stored big endian and are 16 bits large.
        fn fetch_opcode(&self) -> u16 {
            let first_byte: u8 = self.memory[self.program_counter as usize];
            let second_byte: u8 = self.memory[(self.program_counter + 1) as usize];

            ((first_byte as u16) << 8) | (second_byte as u16)
        }

        pub fn instruction_cycle(&mut self) {
            // Fetch
            let _opcode: u16 = self.fetch_opcode();
            self.program_counter += 2;
            // Decode & execute
        }

        pub fn pixel_at(&self, x: usize, y: usize) -> bool {
            assert!(x < DISPLAY_WIDTH);
            assert!(y < DISPLAY_HEIGHT);

            self.display[DISPLAY_HEIGHT - y - 1][x]
        }

        fn sys_addr(&mut self, nnn: u16) {}

        fn cls(&mut self) {}

        fn ret(&mut self) {}

        fn jp_addr(&mut self, nnn: u16) {}

        fn call_addr(&mut self, nnn: u16) {}

        fn se_vx_byte(&mut self, x: usize, kk: u8) {}

        fn sne_vx_byte(&mut self, x: usize, kk: u8) {}

        fn se_vx_vy(&mut self, x: usize, y: usize) {}

        fn ld_vx_byte(&mut self, x: usize, kk: u8) {}

        fn add_vx_byte(&mut self, x: usize, kk: u8) {}

        fn ld_vx_vy(&mut self, x: usize, y: usize) {}

        fn or_vx_vy(&mut self, x: usize, y: usize) {}

        fn and_vx_vy(&mut self, x: usize, y: usize) {}

        fn xor_vx_vy(&mut self, x: usize, y: usize) {}

        fn add_vx_vy(&mut self, x: usize, y: usize) {}

        fn sub_vx_vy(&mut self, x: usize, y: usize) {}

        fn shr_vx_vy(&mut self, x: usize, y: usize) {}

        fn subn_vx_vy(&mut self, x: usize, y: usize) {}

        fn shl_vx_vy(&mut self, x: usize, y: usize) {}

        fn sne_vx_vy(&mut self, x: usize, y: usize) {}

        fn ld_i_addr(&mut self, nnn: u16) {}

        fn jp_v0_addr(&mut self, nnn: u16) {}

        fn rnd_vx_byte(&mut self, x: usize, kk: u8) {}

        fn drw_vx_vy_nibble(&mut self, x: usize, y: usize) {}

        fn skp_vx(&mut self, x: usize) {}

        fn sknp_vx(&mut self, x: usize) {}

        fn ld_vx_dt(&mut self, x: usize) {}

        fn ld_vx_k(&mut self, x: usize) {}

        fn ld_dt_vx(&mut self, x: usize) {}

        fn ld_st_vx(&mut self, x: usize) {}

        fn add_i_vx(&mut self, x: usize) {}

        fn ld_f_vx(&mut self, x: usize) {}

        fn ld_b_vx(&mut self, x: usize) {}

        fn ld_i_vx(&mut self, x: usize) {}

        fn ld_vx_i(&mut self, x: usize) {}
    }

    // Unit tests
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
            // I
            assert_eq!(chip8.i, 0);
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
