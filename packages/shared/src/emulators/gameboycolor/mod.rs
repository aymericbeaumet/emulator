use std::io::prelude::*;
use std::fs::File;

use emulators::gameboycolor::cpu::Cpu;
use screen::ColorDepth32Bit;

mod cpu;

pub struct GameBoyColor {
  bios: &'static [u8],
  cartridge: Vec<u8>,
  memory_map: [u8; 1 + 0xFFFF],
  cpu: Cpu,
  screen: ColorDepth32Bit,
}

impl GameBoyColor {
  pub fn new() -> Self {
    GameBoyColor {
      bios: include_bytes!("./assets/bios.gbc"),
      cartridge: Vec::new(),
      memory_map: [0; 1 + 0xFFFF],
      cpu: Cpu::new(4.194 * 1_000_000.),
      screen: ColorDepth32Bit::new(160, 144),
    }
  }

  pub fn load(&mut self, file_path: &str) {
    File::open(file_path)
      .expect("Cannot open cartridge")
      .read_to_end(&mut self.cartridge)
      .expect("Cannot read cartridge");
  }

  pub fn boot(&mut self) {
    (&mut self.memory_map[..self.bios.len()]).copy_from_slice(&self.bios);
    self.dump_memory_map(0, self.bios.len());
    self.cpu.start();
  }

  pub fn input(&mut self, inputs: u8) {
    if inputs != 0 {
      println!("input: {:>08b}", inputs);
    }
    if inputs & (1 << 0) != 0 {
      self.screen.fill(0xec, 0xd0, 0x25, 0xff);
    } else if inputs & (1 << 1) != 0 {
      self.screen.fill(0xce, 0xce, 0xce, 0xff);
    } else {
      self.screen.fill(0xff, 0xff, 0xff, 0xff);
    }
  }

  pub fn render(&mut self, callback: fn(*mut u32, usize, usize)) {
    let (pixels, width, height) = self.screen.dump();
    callback(pixels, width, height);
  }

  fn dump_memory_map(&self, from: usize, to: usize) {
    let address_width = format!("{:X}", to).len();
    for i in from..to {
      if i % 8 == 0 {
        print!("{:0>width$X} |", i, width = address_width);
      }
      if i % 4 == 0 {
        print!(" ");
      }
      print!("{:0>2X}", self.memory_map[i]);
      if (i + 1) % 8 == 0 {
        println!("");
      }
    }
  }
}

impl Drop for GameBoyColor {
  fn drop(&mut self) {}
}
