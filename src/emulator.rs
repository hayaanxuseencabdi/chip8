use rand::{rngs::ThreadRng, Rng};

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
    keyboard: [bool; 16],
    bitmap: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    sound_timer: u8,
    delay_timer: u8,
    rng: ThreadRng,
}

impl Emulator {
    pub fn new() -> Self {
        let mut memory = [0; MEMORY_SIZE];
        memory[..0x50].copy_from_slice(&SPRITES);

        Emulator {
            program_counter: 0x200,
            memory,
            i: 0,
            v: [0; V_SIZE],
            stack_pointer: 0x0,
            stack: [0; STACK_SIZE],
            keyboard: [false; 16],
            bitmap: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            sound_timer: 0,
            delay_timer: 0,
            rng: rand::thread_rng(),
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> bool {
        assert!(x < DISPLAY_WIDTH);
        assert!(y < DISPLAY_HEIGHT);

        // self.bitmap[DISPLAY_HEIGHT - y - 1][x]
        self.bitmap[y][x]
    }

    pub fn load(&mut self, rom: &[u8; 0xE00]) {
        self.memory[0x200..].copy_from_slice(rom);
    }

    /// The CHIP-8's fetch, decode, and execute instruction cycle.
    pub fn instruction_cycle(&mut self) {
        let opcode: u16 = self.fetch(self.program_counter);
        self.program_counter += 2;
        self.decode_and_execute(opcode);
        // self.dump_debug_info();
    }

    pub fn key_press(&mut self, key: usize) {
        self.keyboard[key] = true;
    }

    pub fn key_release(&mut self, key: usize) {
        assert!(
            self.keyboard[key],
            "Attempting to release a key that wasn't registered as pressed"
        );
        self.keyboard[key] = false;
    }

    fn fetch(&self, program_counter: u16) -> u16 {
        let pc = program_counter as usize;
        // The instructions are stored big endian and are 16 bits large
        let first_byte: u8 = self.memory[pc];
        let second_byte: u8 = self.memory[pc + 1];

        ((first_byte as u16) << 8) | (second_byte as u16)
    }

    fn decode_and_execute(&mut self, opcode: u16) {
        match opcode & 0xF000 {
            0x000 => match opcode & 0x0FFF {
                0x00E0 => {
                    // 00E0 - CLS
                    self.cls();
                }
                0x00EE => {
                    // 00EE - RET
                    self.ret();
                }
                _ => {
                    // 0nnn - SYS addr
                    let nnn = opcode & 0x0FFF;
                    self.sys_addr(nnn);
                }
            },
            0x1000 => {
                // 1nnn - JP addr
                let nnn = opcode & 0x0FFF;
                self.jp_addr(nnn);
            }
            0x2000 => {
                // 2nnn - CALL addr
                let nnn = opcode & 0x0FFF;
                self.call_addr(nnn);
            }
            0x3000 => {
                // 3xkk - SE Vx, byte
                let x = (opcode & 0x0F00) as usize >> 8;
                let kk = (opcode & 0x00FF) as u8;
                self.se_vx_byte(x, kk);
            }
            0x4000 => {
                // 4xkk - SNE Vx, byte
                let x = (opcode & 0x0F00) as usize >> 8;
                let kk = (opcode & 0x00FF) as u8;
                self.sne_vx_byte(x, kk);
            }
            0x5000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        // 5xy0 - SE Vx, Vy
                        let x = (opcode & 0x0F00) as usize >> 8;
                        let y = (opcode & 0x00F0) as usize >> 4;
                        self.se_vx_vy(x, y);
                    }
                    _ => {
                        panic!("Unrecognised opcode: {}", opcode);
                    }
                }
            }
            0x6000 => {
                // 6xkk - LD Vx, byte
                let x = (opcode & 0x0F00) as usize >> 8;
                let kk = (opcode & 0x00FF) as u8;
                self.ld_vx_byte(x, kk);
            }
            0x7000 => {
                // 7xkk - ADD Vx, byte
                let x = (opcode & 0x0F00) as usize >> 8;
                let kk = (opcode & 0x00FF) as u8;
                self.add_vx_byte(x, kk);
            }
            0x8000 => {
                let x = (opcode & 0x0F00) as usize >> 8;
                let y = (opcode & 0x00F0) as usize >> 4;
                match opcode & 0x000F {
                    0x0000 => {
                        // 8xy0 - LD Vx, Vy
                        self.ld_vx_vy(x, y);
                    }
                    0x0001 => {
                        // 8xy1 - OR Vx, Vy
                        self.or_vx_vy(x, y);
                    }
                    0x0002 => {
                        // 8xy2 - AND Vx, Vy
                        self.and_vx_vy(x, y);
                    }
                    0x0003 => {
                        // 8xy3 - XOR Vx, Vy
                        self.xor_vx_vy(x, y);
                    }
                    0x0004 => {
                        // 8xy4 - ADD Vx, Vy
                        self.add_vx_vy(x, y);
                    }
                    0x0005 => {
                        // 8xy5 - SUB Vx, Vy
                        self.sub_vx_vy(x, y);
                    }
                    0x0006 => {
                        // 8xy6 - SHR Vx {, Vy}
                        self.shr_vx(x);
                    }
                    0x0007 => {
                        // 8xy7 - SUBN Vx, Vy
                        self.subn_vx_vy(x, y);
                    }
                    0x000E => {
                        // 8xyE - SHL Vx {, Vy}
                        self.shl_vx(x);
                    }
                    _ => {
                        panic!("Unrecognised opcode: {}", opcode);
                    }
                }
            }
            0x9000 => {
                match opcode & 0x000F {
                    0x0000 => {
                        // 9xy0 - SNE Vx, Vy
                        let x = (opcode & 0x0F00) as usize >> 8;
                        let y = (opcode & 0x00F0) as usize >> 4;
                        self.sne_vx_vy(x, y);
                    }
                    _ => {
                        panic!("Unrecognised opcode: {}", opcode);
                    }
                }
            }
            0xA000 => {
                // Annn - LD I, addr
                let nnn = opcode & 0x0FFF;
                self.ld_i_addr(nnn);
            }
            0xB000 => {
                // Bnnn - JP V0, addr
                let nnn = opcode & 0x0FFF;
                self.jp_v0_addr(nnn);
            }
            0xC000 => {
                // Cxkk - RND Vx, byte
                let x = (opcode & 0x0F00) as usize >> 8;
                let kk = (opcode & 0x00FF) as u8;
                self.rnd_vx_byte(x, kk);
            }
            0xD000 => {
                // Dxyn - DRW Vx, Vy, nibble
                let x = (opcode & 0x0F00) as usize >> 8;
                let y = (opcode & 0x00F0) as usize >> 4;
                let nibble = (opcode & 0x000F) as u8;
                self.drw_vx_vy_nibble(x, y, nibble);
            }
            0xE000 => {
                let x = (opcode & 0x0F00) as usize >> 8;
                match opcode & 0x00FF {
                    0x009E => {
                        // Ex9E - SKP Vx
                        self.skp_vx(x);
                    }
                    0x00A1 => {
                        // ExA1 - SKNP Vx
                        self.sknp_vx(x);
                    }
                    _ => {
                        panic!("Unrecognised opcode: {}", opcode);
                    }
                }
            }
            0xF000 => {
                let x = (opcode & 0x0F00) as usize >> 8;
                match opcode & 0x00FF {
                    0x0007 => {
                        // Fx07 - LD Vx, DT
                        self.ld_vx_dt(x);
                    }
                    0x000A => {
                        // Fx0A - LD Vx, K
                        self.ld_vx_k(x);
                    }
                    0x0015 => {
                        // Fx15 - LD DT, Vx}
                        self.ld_dt_vx(x);
                    }
                    0x0018 => {
                        // Fx18 - LD ST, Vx}
                        self.ld_st_vx(x);
                    }
                    0x001E => {
                        // Fx1E - ADD I, Vx}
                        self.add_i_vx(x);
                    }
                    0x0029 => {
                        // Fx29 - LD F, Vx}
                        self.ld_f_vx(x);
                    }
                    0x0033 => {
                        // Fx33 - LD B, Vx}
                        self.ld_b_vx(x);
                    }
                    0x0055 => {
                        // Fx55 - LD [I], Vx}
                        self.ld_i_vx(x);
                    }
                    0x0065 => {
                        // Fx65 - LD Vx, [I]}
                        self.ld_vx_i(x);
                    }
                    _ => {
                        panic!("Unrecognised opcode: {}", opcode);
                    }
                }
            }
            _ => {
                panic!("Unrecognised opcode: {}", opcode);
            }
        };
    }

    fn dump_debug_info(&self) {
        println!(
            "Program counter: {} -> {:#06X}",
            self.program_counter - 2,
            self.fetch(self.program_counter - 2)
        );
        for i in 0..V_SIZE {
            println!("V[{:#X}]: {}", i, self.v[i]);
        }

        println!();
        println!();
    }

    fn sys_addr(&mut self, _nnn: u16) {
        unimplemented!("The instruction [0nnn - SYS addr] has not been implemented.");
    }

    fn cls(&mut self) {
        for row in self.bitmap.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = false;
            }
        }
    }

    fn ret(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer as usize];
    }

    fn jp_addr(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    fn call_addr(&mut self, nnn: u16) {
        self.stack[self.stack_pointer as usize] = self.program_counter;
        self.stack_pointer += 1;
        self.program_counter = nnn;
    }

    fn se_vx_byte(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.program_counter += 2;
        }
    }

    fn sne_vx_byte(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.program_counter += 2;
        }
    }

    fn se_vx_vy(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.program_counter += 2;
        }
    }

    fn ld_vx_byte(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    fn add_vx_byte(&mut self, x: usize, kk: u8) {
        self.v[x] = self.v[x].overflowing_add(kk).0;
    }

    fn ld_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    fn or_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    fn and_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    fn xor_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    fn add_vx_vy(&mut self, x: usize, y: usize) {
        let (sum, overflow) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = sum;
        self.v[0xF] = overflow as u8;
    }

    fn sub_vx_vy(&mut self, x: usize, y: usize) {
        let (difference, underflow) = self.v[x].overflowing_sub(self.v[y]);
        self.v[x] = difference;
        self.v[0xF] = !underflow as u8;
    }

    fn shr_vx(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 0b0000_0001;
        self.v[x] >>= 1;
    }

    fn subn_vx_vy(&mut self, x: usize, y: usize) {
        let (difference, underflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = difference;
        self.v[0xF] = !underflow as u8;
    }

    fn shl_vx(&mut self, x: usize) {
        self.v[0xF] = (self.v[x] & 0b1000_0000) >> 7;
        self.v[x] <<= 1;
    }

    fn sne_vx_vy(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.program_counter += 2;
        }
    }

    fn ld_i_addr(&mut self, nnn: u16) {
        self.i = nnn;
    }

    fn jp_v0_addr(&mut self, nnn: u16) {
        self.program_counter = nnn + (self.v[0x0] as u16);
    }

    fn rnd_vx_byte(&mut self, x: usize, kk: u8) {
        let random_byte = self.rng.gen_range(0, 256) as u8;
        self.v[x] = kk & random_byte;
    }

    fn drw_vx_vy_nibble(&mut self, x: usize, y: usize, nibble: u8) {
        self.v[0xF] = 0;

        for byte_index in 0..nibble {
            let byte = self.memory[(self.i + byte_index as u16) as usize];
            for pixel_index in 0..8 {
                let row = (self.v[y] + byte_index) as usize % 32;
                let col = (self.v[x] + pixel_index) as usize % 64;
                let bit = byte & (1 << (7 - pixel_index)) != 0;
                self.v[0xF] |= (self.bitmap[row][col] && bit) as u8;
                self.bitmap[row][col] ^= bit;
            }
        }
    }

    fn skp_vx(&mut self, x: usize) {
        self.program_counter += if self.keyboard[x] { 2 } else { 0 }
    }

    fn sknp_vx(&mut self, x: usize) {
        self.program_counter += if self.keyboard[x] { 0 } else { 2 }
    }

    fn ld_vx_dt(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }

    fn ld_vx_k(&mut self, x: usize) {
        // If no key is pressed at the moment, decrement the PC by two to stay at the same instruction.
        self.program_counter -= if self.keyboard.iter().any(|key| *key) {
            for (key, &key_is_pressed) in self.keyboard.iter().enumerate() {
                if key_is_pressed {
                    self.v[x] = key as u8;
                }
            }
            0
        } else {
            2
        };
    }

    fn ld_dt_vx(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }

    fn ld_st_vx(&mut self, x: usize) {
        self.sound_timer = self.v[x];
    }

    fn add_i_vx(&mut self, x: usize) {
        self.i += self.v[x] as u16;
    }

    fn ld_f_vx(&mut self, x: usize) {
        self.i = (5 * self.v[x]) as u16;
    }

    fn ld_b_vx(&mut self, x: usize) {
        let i = self.i as usize;
        self.memory[i] = self.v[x] / 100;
        self.memory[i + 1] = (self.v[x] / 10) % 10;
        self.memory[i + 2] = self.v[x] % 10;
    }

    fn ld_i_vx(&mut self, x: usize) {
        for index in 0x0..x + 1 {
            self.memory[self.i as usize + index] = self.v[index];
        }
    }

    fn ld_vx_i(&mut self, x: usize) {
        for index in 0..x + 1 {
            self.v[index] = self.memory[self.i as usize + index];
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::Emulator;

    #[test]
    fn new() {
        let chip8 = Emulator::new();

        // Program counter
        assert_eq!(
            chip8.program_counter, 0x200,
            "Program counter should be set to 0x200 rather than 0x{:X}",
            chip8.program_counter
        );
        // Memory
        assert_eq!(chip8.memory.len(), 4096);
        assert_eq!(chip8.bitmap.len(), 32);
        assert_eq!(chip8.bitmap[0].len(), 64);
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
    fn sys_addr() {
        let mut chip8 = Emulator::new();

        chip8.sys_addr(0);
    }

    #[test]
    fn cls() {
        let mut chip8 = Emulator::new();
        for row in chip8.bitmap.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = true;
            }
        }

        chip8.cls();

        assert!(chip8
            .bitmap
            .iter()
            .all(|&row| row.iter().all(|&pixel| !pixel)));
    }

    #[test]
    fn ret() {
        let mut chip8 = Emulator::new();
        // Simulate entering a subroutine
        chip8.stack[chip8.stack_pointer as usize] = chip8.program_counter;
        let old_address: u16 = chip8.stack[chip8.stack_pointer as usize];
        chip8.stack_pointer += 1;
        // Set program counter to a different address
        chip8.program_counter = 0xF03D;

        chip8.ret();

        assert_eq!(chip8.stack_pointer, 0);
        assert_eq!(chip8.program_counter, old_address);
    }

    #[test]
    fn jp_addr() {
        let mut chip8 = Emulator::new();
        let nnn: u16 = 0xFD0;

        chip8.jp_addr(nnn);

        assert_eq!(chip8.program_counter, nnn);
    }

    #[test]
    fn call_addr() {
        let mut chip8 = Emulator::new();
        let nnn: u16 = 0x2E6;

        chip8.call_addr(nnn);

        assert_eq!(chip8.program_counter, nnn);
        assert_eq!(chip8.stack_pointer, 1);
        assert_eq!(chip8.stack[(chip8.stack_pointer - 1) as usize], 0x200);
    }

    #[test]
    fn se_vx_byte_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let kk: u8 = 0x00;

        assert_eq!(chip8.v[x], kk);

        chip8.se_vx_byte(x, kk);

        assert_eq!(chip8.program_counter, 0x202);
    }

    #[test]
    fn se_vx_byte_no_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let kk: u8 = 0x04;

        assert_ne!(chip8.v[x], kk);

        chip8.se_vx_byte(x, kk);

        assert_eq!(chip8.program_counter, 0x200);
    }

    #[test]
    fn sne_vx_byte_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let kk: u8 = 0x04;

        assert_ne!(chip8.v[x], kk);

        chip8.sne_vx_byte(x, kk);

        assert_eq!(chip8.program_counter, 0x202);
    }

    #[test]
    fn sne_vx_byte_no_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let kk: u8 = 0x00;

        assert_eq!(chip8.v[x], kk);

        chip8.sne_vx_byte(x, kk);

        assert_eq!(chip8.program_counter, 0x200);
    }

    #[test]
    fn se_vx_vy_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let y: usize = 0x1;

        assert_eq!(chip8.v[x], chip8.v[y]);

        chip8.se_vx_vy(x, y);

        assert_eq!(chip8.program_counter, 0x202);
    }

    #[test]
    fn se_vx_vy_no_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0x0;
        let y: usize = 0x1;
        chip8.v[y] = 1;

        assert_ne!(chip8.v[x], chip8.v[y]);

        chip8.se_vx_vy(x, y);

        assert_eq!(chip8.program_counter, 0x200);
    }

    #[test]
    fn ld_vx_byte() {
        let mut chip8 = Emulator::new();
        let x: usize = 1;
        let kk: u8 = 4;

        assert_ne!(chip8.v[x], kk);

        chip8.ld_vx_byte(x, kk);

        assert_eq!(chip8.v[x], kk);
    }

    #[test]
    fn add_vx_byte() {
        let mut chip8 = Emulator::new();
        let x: usize = 4;
        let kk: u8 = 2;
        chip8.v[x] = 5;

        chip8.add_vx_byte(x, kk);

        assert_eq!(chip8.v[x], 5 + kk);
    }

    #[test]
    fn ld_vx_vy() {
        let mut chip8 = Emulator::new();
        let x: usize = 1;
        let y: usize = 5;
        chip8.v[x] = 2;
        chip8.v[y] = 4;

        assert_ne!(chip8.v[x], chip8.v[y]);

        chip8.ld_vx_vy(x, y);

        assert_eq!(chip8.v[x], chip8.v[y]);
    }

    #[test]
    fn or_vx_vy() {
        let mut chip8 = Emulator::new();
        let x: usize = 1;
        let y: usize = 2;
        chip8.v[x] = 0b1100_0100;
        chip8.v[y] = 0b0111_0000;

        chip8.or_vx_vy(x, y);

        assert_eq!(chip8.v[x], 0b1111_0100);
    }

    #[test]
    fn and_vx_vy() {
        let mut chip8 = Emulator::new();
        let x: usize = 2;
        let y: usize = 0;
        chip8.v[x] = 0b1101_1001;
        chip8.v[y] = 0b0101_0010;

        chip8.and_vx_vy(x, y);

        assert_eq!(chip8.v[x], 0b0101_0000);
    }

    #[test]
    fn xor_vx_vy() {
        let mut chip8 = Emulator::new();
        let x: usize = 2;
        let y: usize = 8;
        chip8.v[x] = 0b1101_1001;
        chip8.v[y] = 0b0101_0010;

        chip8.xor_vx_vy(x, y);

        assert_eq!(chip8.v[x], 0b1000_1011);
    }

    #[test]
    fn add_vx_vy_carry() {
        let mut chip8 = Emulator::new();
        let x: usize = 1;
        let y: usize = 0;
        chip8.v[x] = 255;
        chip8.v[y] = 127;

        chip8.add_vx_vy(x, y);

        assert_eq!(chip8.v[x], 126);
        assert_eq!(chip8.v[0xF], 0x01);
    }

    #[test]
    fn add_vx_vy_no_carry() {
        let mut chip8 = Emulator::new();
        let x: usize = 5;
        let y: usize = 1;
        chip8.v[x] = 128;
        chip8.v[y] = 127;

        chip8.add_vx_vy(x, y);

        assert_eq!(chip8.v[x], 255);
        assert_eq!(chip8.v[0xF], 0x00);
    }

    #[test]
    fn sub_vx_vy_borrow() {
        let mut chip8 = Emulator::new();
        let x: usize = 1;
        let y: usize = 6;
        chip8.v[x] = 55;
        chip8.v[y] = 100;

        chip8.sub_vx_vy(x, y);

        assert_eq!(chip8.v[x], 211);
        assert_eq!(chip8.v[0xF], 0x00);
    }

    #[test]
    fn sub_vx_vy_no_borrow() {
        let mut chip8 = Emulator::new();
        let x: usize = 2;
        let y: usize = 0;
        chip8.v[x] = 100;
        chip8.v[y] = 55;

        chip8.sub_vx_vy(x, y);

        assert_eq!(chip8.v[x], 45);
        assert_eq!(chip8.v[0xF], 0x01);
    }

    #[test]
    fn shr_vx_lsb_is_set() {
        let mut chip8 = Emulator::new();
        let x: usize = 0;
        chip8.v[x] = 0b1011_0101;

        chip8.shr_vx(x);

        assert_eq!(chip8.v[x], 0b0101_1010);
        assert_eq!(chip8.v[0xF], 0x01);
    }

    #[test]
    fn shr_vx_lsb_is_not_set() {
        let mut chip8 = Emulator::new();
        let x: usize = 0;
        chip8.v[x] = 0b0110_0100;

        chip8.shr_vx(x);

        assert_eq!(chip8.v[x], 0b0011_0010);
        assert_eq!(chip8.v[0xF], 0x00);
    }

    #[test]
    fn subn_vx_vy_borrow() {
        let mut chip8 = Emulator::new();
        let x: usize = 1;
        let y: usize = 6;
        chip8.v[x] = 100;
        chip8.v[y] = 55;

        chip8.subn_vx_vy(x, y);

        assert_eq!(chip8.v[x], 211);
        assert_eq!(chip8.v[0xF], 0x00);
    }

    #[test]
    fn subn_vx_vy_no_borrow() {
        let mut chip8 = Emulator::new();
        let x: usize = 2;
        let y: usize = 0;
        chip8.v[x] = 20;
        chip8.v[y] = 55;

        chip8.subn_vx_vy(x, y);

        assert_eq!(chip8.v[x], 35);
        assert_eq!(chip8.v[0xF], 0x01);
    }

    #[test]
    fn shl_vx_msb_is_set() {
        let mut chip8 = Emulator::new();
        let x: usize = 0;
        chip8.v[x] = 0b1011_0101;

        chip8.shl_vx(x);

        assert_eq!(chip8.v[x], 0b0110_1010);
        assert_eq!(chip8.v[0xF], 0x01);
    }

    #[test]
    fn shl_vx_msb_is_not_set() {
        let mut chip8 = Emulator::new();
        let x: usize = 0;
        chip8.v[x] = 0b0110_0100;

        chip8.shl_vx(x);

        assert_eq!(chip8.v[x], 0b1100_1000);
        assert_eq!(chip8.v[0xF], 0x00);
    }

    #[test]
    fn sne_vx_vy_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 3;
        let y: usize = 4;
        chip8.v[x] = 2;
        chip8.v[y] = 0;

        chip8.sne_vx_vy(x, y);

        assert_eq!(chip8.program_counter, 0x202);
    }

    #[test]
    fn sne_vx_vy_no_skip() {
        let mut chip8 = Emulator::new();
        let x: usize = 0;
        let y: usize = 8;
        chip8.v[x] = 0;
        chip8.v[y] = 0;

        chip8.sne_vx_vy(x, y);

        assert_eq!(chip8.program_counter, 0x200);
    }

    #[test]
    fn ld_i_addr() {
        let mut chip8 = Emulator::new();
        let nnn: u16 = 0xF3B;

        chip8.ld_i_addr(nnn);

        assert_eq!(chip8.i, 0xF3B);
    }

    #[test]
    fn jp_v0_addr() {
        let mut chip8 = Emulator::new();
        chip8.v[0x0] = 0x0F0;
        let nnn: u16 = 0x203;

        chip8.jp_v0_addr(nnn);

        assert_eq!(chip8.program_counter, 0x2F3);
    }

    #[test]
    fn ld_vx_dt() {
        unimplemented!();
    }

    #[test]
    fn ld_dt_vx() {
        unimplemented!();
    }

    #[test]
    fn ld_st_vx() {
        unimplemented!();
    }

    #[test]
    fn add_i_vx() {
        let mut chip8 = Emulator::new();
        let x: usize = 3;
        chip8.v[x] = 10;
        chip8.i = 1 as u16;

        chip8.add_i_vx(x);

        assert_eq!(chip8.i, 11);
    }

    #[test]
    fn ld_f_vx() {
        unimplemented!();
    }

    #[test]
    fn ld_b_vx() {
        let mut chip8 = Emulator::new();
        let x: usize = 4;
        chip8.v[x] = 241;
        chip8.i = 4;

        chip8.ld_b_vx(x);

        assert_eq!(chip8.memory[chip8.i as usize], 2);
        assert_eq!(chip8.memory[(chip8.i + 1) as usize], 4);
        assert_eq!(chip8.memory[(chip8.i + 2) as usize], 1);
    }

    #[test]
    fn ld_i_vx() {
        unimplemented!();
    }

    #[test]
    fn ld_vx_i() {
        unimplemented!();
    }
}
