pub struct Screen {
  pixels: Vec<u32>,
  width: usize,
  height: usize,
}

impl Screen {
  pub fn new(width: usize, height: usize) -> Self {
    Screen {
      pixels: vec![0; width * height],
      width,
      height,
    }
  }

  pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
    let rgba = (r as u32) << 0 | (g as u32) << 8 | (b as u32) << 16 | (a as u32) << 24;
    for pixel in &mut self.pixels.iter_mut() {
      *pixel = rgba
    }
  }

  pub fn dump(&mut self) -> (*mut u32, usize, usize) {
    (self.pixels.as_mut_ptr(), self.width, self.height)
  }
}

impl Drop for Screen {
  fn drop(&mut self) {}
}
