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

  fn get_high_byte(register: u16) -> u8 {
    (register >> 8) as u8
  }

  fn set_high_byte(register: &mut u16, value: u8) {
    register = &mut ((value as u16) << 8 | (*register & 0xFF));
  }

  fn get_low_byte(register: u16) -> u8 {
    (register & 0xFF) as u8
  }

  fn set_low_byte(register: &mut u16, value: u8) {
    register = &mut ((*register & 0xFF00) | value as u16);
  }

  pub fn get_a(&self) -> u8 {
    Registers::get_high_byte(self.af)
  }

  pub fn set_a(&mut self, value: u8) {
    Registers::set_high_byte(&mut self.af, value)
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

  pub fn get_b(&self) -> u8 {
    Registers::get_high_byte(self.bc)
  }

  pub fn set_b(&mut self, value: u8) {
    Registers::set_high_byte(&mut self.bc, value)
  }

  pub fn get_c(&self) -> u8 {
    Registers::get_low_byte(self.bc)
  }

  pub fn set_c(&mut self, value: u8) {
    Registers::set_low_byte(&mut self.bc, value)
  }

  pub fn get_d(&self) -> u8 {
    Registers::get_high_byte(self.de)
  }

  pub fn set_d(&mut self, value: u8) {
    Registers::set_high_byte(&mut self.de, value)
  }

  pub fn get_e(&self) -> u8 {
    Registers::get_low_byte(self.de)
  }

  pub fn set_e(&mut self, value: u8) {
    Registers::set_low_byte(&mut self.de, value)
  }

  pub fn get_h(&self) -> u8 {
    Registers::get_high_byte(self.hl)
  }

  pub fn set_h(&mut self, value: u8) {
    Registers::set_high_byte(&mut self.hl, value)
  }

  pub fn get_l(&self) -> u8 {
    Registers::get_low_byte(self.hl)
  }

  pub fn set_l(&mut self, value: u8) {
    Registers::set_low_byte(&mut self.hl, value)
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

  fn get_f(&self) -> u8 {
    Registers::get_low_byte(self.af)
  }
}
