use rand::Rng;

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
    sound_timer: u8,
    delay_timer: u8,
    rng: rand::rngs::ThreadRng,
}

impl Emulator {
    pub fn new() -> Self {
        let mut memory = [0; MEMORY_SIZE];
        memory[..0x50].copy_from_slice(&SPRITES[..0x50]);

        Emulator {
            program_counter: 0x200,
            memory,
            i: 0,
            v: [0; V_SIZE],
            stack_pointer: 0x0,
            stack: [0; STACK_SIZE],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            sound_timer: 0,
            delay_timer: 0,
            rng: rand::thread_rng(),
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

    fn cls(&mut self) {
        for row in self.display.iter_mut() {
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
        self.v[x] += kk;
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
        self.v[x] = self.v[x] >> 1;
    }

    fn subn_vx_vy(&mut self, x: usize, y: usize) {
        let (difference, underflow) = self.v[y].overflowing_sub(self.v[x]);
        self.v[x] = difference;
        self.v[0xF] = !underflow as u8;
    }

    fn shl_vx(&mut self, x: usize) {
        self.v[0xF] = (self.v[x] & 0b1000_0000) >> 7;
        self.v[x] = self.v[x] << 1;
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
        unsafe {
            let random_byte = self.rng.gen_range(0, 256) as u8;
            self.v[x] = kk & random_byte;
        }
    }

    fn drw_vx_vy_nibble(&mut self, x: usize, y: usize, nibble: u8) {
        self.v[0xF] = 0;
        for byte in 0..nibble {
            for pixel in 0..8 {
                let row = usize::from(self.v[y] + byte) % 32;
                let column = usize::from(self.v[x] + pixel) % 64;
                let bit = (byte & (1 << (7 - pixel))) != 0;
                // TODO: double check indexing
                self.v[0xF] |= (self.display[row][column] && bit) as u8;
                self.display[row][column] ^= bit;
            }
        }
    }

    fn skp_vx(&mut self, x: usize) {
        unimplemented!();
    }

    fn sknp_vx(&mut self, x: usize) {
        unimplemented!();
    }

    fn ld_vx_dt(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }

    fn ld_vx_k(&mut self, x: usize) {
        unimplemented!();
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
        unimplemented!();
    }

    fn ld_b_vx(&mut self, x: usize) {
        let i = self.i as usize;
        self.memory[i] = self.v[x] / 100;
        self.memory[i + 1] = (self.v[x] / 10) % 10;
        self.memory[i + 2] = self.v[x] % 10;
    }

    fn ld_i_vx(&mut self, x: usize) {
        for index in 0x0..0xF {
            self.memory[(self.i + index) as usize] = self.v[index as usize];
        }
    }

    fn ld_vx_i(&mut self, x: usize) {
        for index in 0x0..0xF {
            self.v[index as usize] = self.memory[(self.i + index) as usize];
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
    fn sys_addr() {
        let mut chip8 = Emulator::new();

        chip8.sys_addr(0);
    }

    #[test]
    fn cls() {
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
