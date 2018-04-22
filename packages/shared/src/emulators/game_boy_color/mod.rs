mod memory_map;
mod processor;
mod registers;

use std::fs::File;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use self::memory_map::{MemoryMap, ToMemoryMap};
use self::processor::Processor;
use self::registers::Registers;
// use screen::ColorDepth32Bit;

const BIOS: &'static [u8] = include_bytes!("./bios.gbc");

pub struct GameBoyColor {
  cartridge: Vec<u8>,
  memory_map: MemoryMap,
  registers: Registers,
  cycle_duration_nanos: u64,
  // screen: ColorDepth32Bit,
}

impl GameBoyColor {
  pub fn new() -> Self {
    GameBoyColor {
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
    self.memory_map.write(0x0000, BIOS);
    self.registers.reset();
    loop {
      println!("---");
      self.registers.dump();
      let time = Instant::now();
      let cycles = Processor::step(&mut self.registers, &mut self.memory_map);
      let elapsed = time.elapsed();
      let elapsed_nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
      let supposedly_elapsed_nanos = (cycles as u64) * self.cycle_duration_nanos;
      if supposedly_elapsed_nanos > elapsed_nanos {
        sleep(Duration::from_nanos(
          supposedly_elapsed_nanos - elapsed_nanos,
        ))
      }
    }
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
