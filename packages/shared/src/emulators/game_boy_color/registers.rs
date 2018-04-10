// 16bit Hi   Lo   Name/Function
// AF    A    -    Accumulator & Flags
// BC    B    C    BC
// DE    D    E    DE
// HL    H    L    HL
// SP    -    -    Stack Pointer
// PC    -    -    Program Counter/Pointer

pub enum Flag {
  Z,
  N,
  H,
  C,
}

impl Flag {
  fn index(flag: Flag) -> usize {
    match flag {
      Flag::Z => 7,
      Flag::N => 6,
      Flag::H => 5,
      Flag::C => 4,
    }
  }
}

pub struct Registers {
  pub af: u16,
  pub bc: u16,
  pub de: u16,
  pub hl: u16,
  pub sp: u16,
  pub pc: u16,
}

impl Registers {
  pub fn new() -> Self {
    Registers {
      af: 0x01B0,
      bc: 0x0013,
      de: 0x00D8,
      hl: 0x014D,
      sp: 0x0000,
      pc: 0x0000,
    }
  }

  pub fn get_a(&self) -> u8 {
    (self.af >> 8) as u8
  }

  pub fn set_a(&mut self, a: u8) {
    self.af = (a as u16) << 8 | (self.get_f() as u16)
  }

  fn get_f(&self) -> u8 {
    (self.af & 0xFF) as u8
  }

  pub fn get_flag(&self, flag: Flag) -> bool {
    let mask = 1 << Flag::index(flag);
    (self.get_f() & mask) != 0
  }

  pub fn set_flag(&mut self, flag: Flag, state: bool) {
    let mask = (1 as u16) << Flag::index(flag);
    if state {
      self.af |= mask;
    } else {
      self.af &= !mask;
    }
  }

  pub fn get_c(&self) -> u8 {
    (self.bc & 0xFF) as u8
  }

  pub fn set_c(&mut self, c: u8) {
    self.bc = (self.bc & (0xFF << 8)) | (c as u16)
  }

  pub fn dump(&self) {
    print!(
      "AF=0x{:04X} (Z={}, N={}, H={}, C={})",
      self.af,
      if self.get_flag(Flag::Z) { 1 } else { 0 },
      if self.get_flag(Flag::N) { 1 } else { 0 },
      if self.get_flag(Flag::H) { 1 } else { 0 },
      if self.get_flag(Flag::C) { 1 } else { 0 }
    );
    print!(" BC=0x{:04X}", self.bc);
    print!(" DE=0x{:04X}", self.de);
    print!(" HL=0x{:04X}", self.hl);
    print!(" SP=0x{:04X}", self.sp);
    println!(" PC=0x{:04X}", self.pc);
  }
}
