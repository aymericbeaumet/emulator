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
  fn position(&self) -> usize {
    match self {
      Flag::Z => 7,
      Flag::N => 6,
      Flag::H => 5,
      Flag::C => 4,
    }
  }
}

pub struct Registers {
  af: u16,
  bc: u16,
  de: u16,
  hl: u16,
  pub sp: u16,
  pub pc: u16,
}

impl Registers {
  pub fn new() -> Self {
    let mut registers = Registers {
      af: 0x0000,
      bc: 0x0000,
      de: 0x0000,
      hl: 0x0000,
      sp: 0x0000,
      pc: 0x0000,
    };
    registers.reset();
    registers
  }

  pub fn reset(&mut self) {
    self.af = 0x01B0;
    self.bc = 0x0013;
    self.de = 0x00D8;
    self.hl = 0x014D;
    self.sp = 0x0000;
    self.pc = 0x0000;
  }

  fn get_high_byte(register: u16) -> u8 {
    (register >> 8) as u8
  }

  fn set_high_byte(register: u16, value: u8) -> u16 {
    let high_byte = (value as u16) << 8;
    let low_byte = Registers::get_low_byte(register) as u16;
    high_byte | low_byte
  }

  fn get_low_byte(register: u16) -> u8 {
    (register & 0xFF) as u8
  }

  fn set_low_byte(register: u16, value: u8) -> u16 {
    let high_byte = (Registers::get_high_byte(register) as u16) << 8;
    let low_byte = value as u16;
    high_byte | low_byte
  }

  pub fn get_a(&self) -> u8 {
    Registers::get_high_byte(self.af)
  }

  pub fn set_a(&mut self, value: u8) {
    self.af = Registers::set_high_byte(self.af, value);
  }

  fn get_f(&self) -> u8 {
    Registers::get_low_byte(self.af)
  }

  fn set_f(&mut self, value: u8) {
    self.af = Registers::set_high_byte(self.af, value);
  }

  pub fn get_b(&self) -> u8 {
    Registers::get_high_byte(self.bc)
  }

  pub fn set_b(&mut self, value: u8) {
    self.bc = Registers::set_high_byte(self.bc, value);
  }

  pub fn get_c(&self) -> u8 {
    Registers::get_low_byte(self.bc)
  }

  pub fn set_c(&mut self, value: u8) {
    self.bc = Registers::set_low_byte(self.bc, value);
  }

  pub fn get_d(&self) -> u8 {
    Registers::get_high_byte(self.de)
  }

  pub fn set_d(&mut self, value: u8) {
    self.de = Registers::set_high_byte(self.de, value);
  }

  pub fn get_e(&self) -> u8 {
    Registers::get_low_byte(self.de)
  }

  pub fn set_e(&mut self, value: u8) {
    self.de = Registers::set_low_byte(self.de, value);
  }

  pub fn get_h(&self) -> u8 {
    Registers::get_high_byte(self.hl)
  }

  pub fn set_h(&mut self, value: u8) {
    self.hl = Registers::set_high_byte(self.hl, value);
  }

  pub fn get_l(&self) -> u8 {
    Registers::get_low_byte(self.hl)
  }

  pub fn set_l(&mut self, value: u8) {
    self.hl = Registers::set_low_byte(self.hl, value);
  }

  pub fn get_flag(&self, flag: Flag) -> bool {
    let mask = 1 << flag.position();
    (self.get_f() & mask) != 0
  }

  pub fn set_flag(&mut self, flag: Flag, enabled: bool) {
    let mask = 1 << flag.position();
    if enabled {
      self.af |= mask;
    } else {
      self.af &= !mask;
    }
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
