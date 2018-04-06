mod screen;

use self::screen::Screen;

pub struct Engine {
  screen: Screen,
}

impl Engine {
  pub fn new() -> Self {
    Engine {
      screen: Screen::new(160, 144),
    }
  }

  pub fn input(&mut self, inputs: u8) {
    if inputs != 0 {
      println!("inputs: {:>08b}", inputs);
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

impl Drop for Engine {
  fn drop(&mut self) {}
}
