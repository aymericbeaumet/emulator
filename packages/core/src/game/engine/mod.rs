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

  pub fn input(&mut self, keys: u8) {
    if keys != 0 {
      println!("keys: {:>08b}", keys);
    }
    if keys & (1 << 0) != 0 {
      self.screen.fill(0xec, 0xd0, 0x25, 100);
    } else if keys & (1 << 1) != 0 {
      self.screen.fill(0xce, 0xce, 0xce, 100);
    } else {
      self.screen.fill(0xff, 0xff, 0xff, 100);
    }
  }

  pub fn update(&self) {}

  pub fn render(&mut self, callback: fn(*mut u8, usize, usize)) {
    let (pixels, width, height) = self.screen.dump();
    callback(pixels, width, height);
  }
}

impl Drop for Engine {
  fn drop(&mut self) {}
}
