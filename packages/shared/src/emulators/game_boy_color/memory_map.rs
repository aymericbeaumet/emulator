pub struct MemoryMap {
  buffer: [u8; 0xFFFF],
}

impl MemoryMap {
  pub fn new() -> Self {
    MemoryMap {
      buffer: [0; 0xFFFF],
    }
  }

  pub fn read_u8(&self, index: u16) -> u8 {
    self.buffer[index as usize]
  }

  pub fn read_u16(&self, index: u16) -> u16 {
    (self.buffer[index as usize + 0] as u16) << 0 | (self.buffer[index as usize + 1] as u16) << 8
  }

  pub fn read_i8(&self, index: u16) -> i8 {
    self.buffer[index as usize] as i8
  }

  pub fn write(&mut self, index: u16, value: &[u8]) {
    (&mut self.buffer[index as usize..(index as usize + value.len())]).copy_from_slice(value);
  }

  pub fn write_u8(&mut self, index: u16, value: u8) {
    self.buffer[index as usize] = value;
  }

  pub fn write_u16(&mut self, index: u16, value: u16) {
    self.buffer[index as usize + 0] = (value & 0xFF) as u8;
    self.buffer[index as usize + 1] = (value >> 8) as u8;
  }

  // pub fn dump_memory_map(&self, from: usize, to: usize) {
  //   let address_width = format!("{:X}", to).len();
  //   for i in from..to {
  //     if i % 8 == 0 {
  //       print!("{:0>width$X} |", i, width = address_width);
  //     }
  //     if i % 4 == 0 {
  //       print!(" ");
  //     }
  //     print!("{:0>2X}", self.buffer[i]);
  //     if (i + 1) % 8 == 0 {
  //       println!("");
  //     }
  //   }
  // }
}
