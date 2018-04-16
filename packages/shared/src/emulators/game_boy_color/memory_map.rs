type Address = u16;

pub struct MemoryMap {
  buffer: [u8; 0xFFFF],
}

impl MemoryMap {
  pub fn new() -> Self {
    MemoryMap {
      buffer: [0; 0xFFFF],
    }
  }

  pub fn write(&mut self, index: Address, value: &[u8]) {
    (&mut self.buffer[index as usize..(index as usize + value.len())]).copy_from_slice(value);
  }

  pub fn write_u8(&mut self, index: Address, value: u8) {
    self.buffer[index as usize] = value;
  }

  pub fn write_u16(&mut self, index: Address, value: u16) {
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

pub trait FromMemoryMap<T> {
  fn read(&self, index: Address) -> T;
}

impl FromMemoryMap<u8> for MemoryMap {
  fn read(&self, index: Address) -> u8 {
    self.buffer[index as usize]
  }
}

impl FromMemoryMap<u16> for MemoryMap {
  fn read(&self, index: Address) -> u16 {
    (self.buffer[index as usize + 0] as u16) << 0 | (self.buffer[index as usize + 1] as u16) << 8
  }
}

impl FromMemoryMap<i8> for MemoryMap {
  fn read(&self, index: Address) -> i8 {
    self.buffer[index as usize] as i8
  }
}
