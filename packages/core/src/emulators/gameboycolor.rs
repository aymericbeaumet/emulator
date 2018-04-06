use screen::Screen;

pub struct GameBoyColor {
  screen: Screen,
}

impl GameBoyColor {
  pub fn new() -> Self {
    GameBoyColor {
      screen: Screen::new(160, 144),
    }
  }

  pub fn load(&self, filepath: &str) {
    println!("load: {}", filepath)
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
