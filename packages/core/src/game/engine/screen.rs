const RGBA: usize = 4;

pub struct Screen {
  width: usize,
  height: usize,
  pixels: Vec<u8>,
}

impl Screen {
  pub fn new(width: usize, height: usize) -> Self {
    let mut screen = Screen {
      width,
      height,
      pixels: vec![0; RGBA * width * height],
    };
    for i in 0..screen.pixels.len() {
      if (i + 1) % 4 == 0 {
        screen.pixels[i] = 100; // A
      } else {
        screen.pixels[i] = 0x33; // R, G, B
      }
    }
    screen
  }

  pub fn dump(&mut self) -> (*mut u8, usize, usize) {
    (self.pixels.as_mut_ptr(), self.width, self.height)
  }
}

impl Drop for Screen {
  fn drop(&mut self) {}
}
