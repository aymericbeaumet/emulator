use std::io::prelude::*;
use std::fs::File;

use screen::Screen;

pub struct GameBoyColor {
  screen: Screen,
  cartridge: Vec<u8>,
}

impl GameBoyColor {
  pub fn new() -> Self {
    GameBoyColor {
      screen: Screen::new(160, 144),
      cartridge: Vec::new(),
    }
  }

  pub fn boot_with_file_path(&mut self, file_path: &str) {
    File::open(file_path)
      .expect("Cannot open cartridge")
      .read_to_end(&mut self.cartridge)
      .expect("Cannot read cartridge");
    // TODO: start CPU
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
}

impl Drop for GameBoyColor {
  fn drop(&mut self) {}
}
