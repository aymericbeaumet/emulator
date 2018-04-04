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

  pub fn input(&self, keys: u8) {
    if keys != 0 {
      println!("keys: {:>08b}", keys);
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
