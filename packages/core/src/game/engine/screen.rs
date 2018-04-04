const RGBA: usize = 4;

pub struct Screen {
  width: usize,
  height: usize,
  pixels: Vec<u8>,
}

impl Screen {
  pub fn new(width: usize, height: usize) -> Self {
    Screen {
      width,
      height,
      pixels: vec![0; RGBA * width * height],
    }
  }

  pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
    for i in 0..self.pixels.len() {
      if (i % 4) == 0 {
        self.pixels[i + 0] = r;
        self.pixels[i + 1] = g;
        self.pixels[i + 2] = b;
        self.pixels[i + 3] = a;
      }
    }
  }

  pub fn dump(&mut self) -> (*mut u8, usize, usize) {
    (self.pixels.as_mut_ptr(), self.width, self.height)
  }
}

impl Drop for Screen {
  fn drop(&mut self) {}
}
