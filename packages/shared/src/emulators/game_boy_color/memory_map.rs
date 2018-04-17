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

pub trait ToMemoryMap<T> {
  fn write(&mut self, index: Address, value: T);
}

impl<'a> ToMemoryMap<&'a [u8]> for MemoryMap {
  fn write(&mut self, index: Address, value: &[u8]) {
    (&mut self.buffer[index as usize..(index as usize + value.len())]).copy_from_slice(value);
  }
}

impl ToMemoryMap<u8> for MemoryMap {
  fn write(&mut self, index: Address, value: u8) {
    self.buffer[index as usize] = value;
  }
}

impl ToMemoryMap<u16> for MemoryMap {
  fn write(&mut self, index: Address, value: u16) {
    self.buffer[index as usize + 0] = (value & 0xFF) as u8;
    self.buffer[index as usize + 1] = (value >> 8) as u8;
  }
}
