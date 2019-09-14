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

    pub fn pixel_at(&self, x: usize, y: usize) -> bool {
        assert!(x < DISPLAY_WIDTH);
        assert!(y < DISPLAY_HEIGHT);

        self.display[DISPLAY_HEIGHT - y - 1][x]
    }

    /// The CHIP-8's fetch, decode, and execute instruction cycle.
    pub fn instruction_cycle(&mut self) {
        let opcode: u16 = self.fetch(self.program_counter as usize);
        self.program_counter += 2;
        self.decode_and_execute(opcode);
    }

    fn fetch(&self, program_counter: usize) -> u16 {
        // The instructions are stored big endian and are 16 bits large
        let first_byte: u8 = self.memory[program_counter];
        let second_byte: u8 = self.memory[program_counter + 1];

        ((first_byte as u16) << 8) | (second_byte as u16)
    }

    fn decode_and_execute(&mut self, opcode: u16) {
        // TODO: Match opcode to instruction
        // TODO: Make sure to log the instruction & the values it's called with
    }

    fn sys_addr(&mut self, nnn: u16) {
        unimplemented!("The instruction [0nnn - SYS addr] has not been implemented.");
    }

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
    use super::Emulator;

    #[test]
    fn test_constructor() {
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

    #[test]
    #[should_panic]
    fn test_sys_addr() {
        let mut chip8 = Emulator::new();

        chip8.sys_addr(0);
    }

    #[test]
    fn test_cls() {
        let mut chip8 = Emulator::new();
        for row in chip8.display.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = true;
            }
        }

        chip8.cls();

        assert!(chip8
            .display
            .iter()
            .all(|&row| row.iter().all(|&pixel| !pixel)));
    }

    #[test]
    fn test_ret() {
        let mut chip8 = Emulator::new();
        // Push old program counter onto stack
        let old_address: u16 = chip8.stack[chip8.stack_pointer as usize];
        chip8.stack[chip8.stack_pointer as usize] = 1;
        chip8.stack_pointer += 1;
        // Set program counter to a different address
        chip8.program_counter = 0xF03D;

        chip8.ret();

        assert_eq!(chip8.stack_pointer, 0);
        assert_eq!(chip8.program_counter, old_address);
    }

    #[test]
    fn test_jp_addr() {
        let mut chip8 = Emulator::new();
        let nnn: u16 = 0xFD0;

        chip8.jp_addr(nnn);

        assert_eq!(chip8.program_counter, nnn);
    }

    #[test]
    fn test_call_addr() {
        let mut chip8 = Emulator::new();
        let nnn: u16 = 0x2E6;

        chip8.call_addr(nnn);

        assert_eq!(chip8.program_counter, nnn);
        assert_eq!(chip8.stack_pointer, 1);
        assert_eq!(chip8.stack[chip8.stack_pointer as usize], 0x200);
    }

    #[test]
    fn test_se_vx_byte_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let kk: u8 = 0x00;

        assert_eq!(chip8.v[x], kk);

        chip8.se_vx_byte(x, kk);

        assert_eq!(chip8.program_counter, 0x202);
    }

    #[test]
    fn test_se_vx_byte_no_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let kk: u8 = 0x04;

        assert_ne!(chip8.v[x], kk);

        chip8.se_vx_byte(x, kk);

        assert_eq!(chip8.program_counter, 0x200);
    }

    #[test]
    fn test_sne_vx_byte_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let kk: u8 = 0x04;

        assert_ne!(chip8.v[x], kk);

        chip8.sne_vx_byte(x, kk);

        assert_eq!(chip8.program_counter, 0x202);
    }

    #[test]
    fn test_sne_vx_byte_no_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let kk: u8 = 0x00;

        assert_eq!(chip8.v[x], kk);

        chip8.sne_vx_byte(x, kk);

        assert_eq!(chip8.program_counter, 0x200);
    }

    #[test]
    fn test_se_vx_vy_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let y: usize = 0x1;

        assert_eq!(chip8.v[x], chip8.v[y]);

        chip8.se_vx_vy(x, y);

        assert_eq!(chip8.program_counter, 0x202);
    }

    #[test]
    fn test_se_vx_vy_no_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let y: usize = 0x1;
        chip8.v[y] = 1;

        assert_ne!(chip8.v[x], chip8.v[y]);

        chip8.se_vx_vy(x, y);

        assert_eq!(chip8.program_counter, 0x200);
    }
}
