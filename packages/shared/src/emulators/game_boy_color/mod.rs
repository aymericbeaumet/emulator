mod memory_map;
mod registers;

use std::fs::File;
use std::io::prelude::*;
use std::mem::size_of_val;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use self::memory_map::MemoryMap;
use self::registers::{Flag, Registers};
// use screen::ColorDepth32Bit;

pub struct GameBoyColor {
  bios: &'static [u8],
  cartridge: Vec<u8>,

  memory_map: MemoryMap,
  registers: Registers,
  cycle_duration_nanos: u64,
  // screen: ColorDepth32Bit,
}

impl GameBoyColor {
  pub fn new() -> Self {
    GameBoyColor {
      bios: include_bytes!("./assets/bios.gbc"),
      cartridge: Vec::new(),

      memory_map: MemoryMap::new(),
      registers: Registers::new(),
      cycle_duration_nanos: (1_000_000_000. / (4.194 * 1_000_000.)) as u64,
      // screen: ColorDepth32Bit::new(160, 144),
    }
  }

  pub fn load(&mut self, file_path: &str) {
    File::open(file_path)
      .expect("Cannot open cartridge")
      .read_to_end(&mut self.cartridge)
      .expect("Cannot read cartridge");
  }

  pub fn boot(&mut self) {
    self.memory_map.write(0x0000, &self.bios);
    loop {
      self.registers.dump();
      let time = Instant::now();
      let instruction = self.memory_map.read_u8(self.registers.pc as usize);

      print!("[0x{:04X}] 0x{:02X}:", self.registers.pc, instruction);
      let cycles = match instruction {
        0x0E => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let d8 = self.memory_map.read_u8(self.registers.pc as usize);
          self.registers.pc += size_of_val(&d8) as u16;
          self.registers.set_c(d8);
          println!("LD C, d8=0x{:02X}", d8);
          8
        }

        0x20 => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let r8 = self.memory_map.read_i8(self.registers.pc as usize);
          if !self.registers.get_flag(Flag::Z) {
            self.registers.pc += size_of_val(&r8) as u16;
            println!("JR NZ, r8=noop");
            8
          } else {
            self.registers.pc =
              (self.registers.pc as i32 - (size_of_val(&instruction) as i32) + r8 as i32) as u16;
            println!("JR NZ, r8=0x{:02X}", r8);
            12
          }
        }

        0x21 => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let d16 = self.memory_map.read_u16(self.registers.pc as usize);
          self.registers.pc += size_of_val(&d16) as u16;
          self.registers.hl = d16;
          println!("LD HL, d16=0x{:04X}", d16);
          12
        }

        0x22 => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let a = self.registers.get_a();
          self.memory_map.write_u8(self.registers.hl as usize, a);
          println!("LDI (HL=0x{:04X}), A=0x{:02X}", self.registers.hl, a);
          self.registers.hl += 1;
          8
        }

        0x2F => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let a = self.registers.get_a();
          let a = !a;
          self.registers.set_a(a);
          self.registers.set_flag(Flag::N, true);
          self.registers.set_flag(Flag::H, true);
          println!("CPL");
          4
        }

        0x31 => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let d16 = self.memory_map.read_u16(self.registers.pc as usize);
          self.registers.pc += size_of_val(&d16) as u16;
          self.registers.sp = d16;
          println!("LD SP, d16=0x{:04X}", d16);
          12
        }

        0x3E => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let d8 = self.memory_map.read_u8(self.registers.pc as usize);
          self.registers.pc += size_of_val(&d8) as u16;
          self.registers.set_a(d8);
          println!("LD A, d8=0x{:02X}", d8);
          8
        }

        0xAF => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let a = self.registers.get_a();
          let a = a ^ a;
          self.registers.set_a(a);
          self.registers.set_flag(Flag::Z, a == 0);
          self.registers.set_flag(Flag::N, false);
          self.registers.set_flag(Flag::H, false);
          self.registers.set_flag(Flag::C, false);
          println!("XOR A");
          4
        }

        0xC3 => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let a16 = self.memory_map.read_u16(self.registers.pc as usize);
          self.registers.pc = a16;
          println!("JP a16=0x{:04X}", a16);
          16
        }

        0xCD => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let a16 = self.memory_map.read_u16(self.registers.pc as usize);
          self.registers.pc += size_of_val(&a16) as u16;
          let pc = self.registers.pc;
          self.stack_push_u16(pc);
          self.registers.pc = a16;
          println!("CALL a16=0x{:04X}", a16);
          24
        }

        0xE0 => {
          self.registers.pc += size_of_val(&instruction) as u16;
          let a8 = self.memory_map.read_u8(self.registers.pc as usize);
          self.registers.pc += size_of_val(&a8) as u16;
          self
            .memory_map
            .write_u8(0xFF00 + a8 as usize, self.registers.get_a());
          println!("LDH (a8=0xFF{:02X}), A", a8);
          12
        }

        _ => {
          panic!("Unsupported instruction 0x{:02X}", instruction);
        }
      };

      let elapsed = time.elapsed();
      let elapsed_nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
      let realistic_elapsed_nanos = cycles * self.cycle_duration_nanos;
      if elapsed_nanos < realistic_elapsed_nanos {
        sleep(Duration::from_nanos(
          realistic_elapsed_nanos - elapsed_nanos,
        ))
      }
    }
  }

  fn stack_push_u16(&mut self, value: u16) {
    self.registers.sp -= size_of_val(&value) as u16;
    self.memory_map.write_u16(self.registers.sp as usize, value);
  }

  // pub fn input(&mut self, inputs: u8) {
  //   if inputs != 0 {
  //     println!("input: {:>08b}", inputs);
  //   }
  //   if inputs & (1 << 0) != 0 {
  //     self.screen.fill(0xec, 0xd0, 0x25, 0xff);
  //   } else if inputs & (1 << 1) != 0 {
  //     self.screen.fill(0xce, 0xce, 0xce, 0xff);
  //   } else {
  //     self.screen.fill(0xff, 0xff, 0xff, 0xff);
  //   }
  // }

  // pub fn render(&mut self, callback: fn(*mut u32, usize, usize)) {
  //   let (pixels, width, height) = self.screen.dump();
  //   callback(pixels, width, height);
  // }
}

impl Drop for GameBoyColor {
  fn drop(&mut self) {}
}
