mod LD;
mod NOP;

use super::memory_map::MemoryMap;
use super::registers::Registers;

pub type Opcode = u8;
pub type Cycle = u8;

pub trait Instruction<T, U, V> {
  fn instruction(&mut self, _: U, _: V) -> Cycle;
}

pub mod Instructions {
  pub enum LD {}
  pub enum NOP {}
}

pub mod Operands {
  pub enum void {
    __type__,
  }
  pub enum A {
    __type__,
  }
  pub enum B {
    __type__,
  }
  pub enum C {
    __type__,
  }
  pub enum D {
    __type__,
  }
  pub enum E {
    __type__,
  }
  pub enum H {
    __type__,
  }
  pub enum L {
    __type__,
  }
  pub enum BC {
    __type__,
  }
  pub enum DE {
    __type__,
  }
  pub enum HL {
    __type__,
  }
  pub enum SP {
    __type__,
  }
  pub enum PC {
    __type__,
  }
  pub enum d8 {
    __type__,
  }
  pub enum d16 {
    __type__,
  }
  pub enum a8 {
    __type__,
  }
  pub enum a16 {
    __type__,
  }
  pub enum r8 {
    __type__,
  }
  pub type literal = u8;
  pub enum pointer<T> {
    __type__(T),
  }
}

macro_rules! instruction {
  // Arity 0

  {$instance:expr, $mnemonic:ident} => (
    Instruction::<
      Instructions::$mnemonic,
      Operands::void,
      Operands::void,
    >::instruction(
      $instance,
      Operands::void::__type__,
      Operands::void::__type__,
    )
  );

  // Arity 1

  {$instance:expr, $mnemonic:ident $op1_type:ident} => (
    Instruction::<
      Instructions::$mnemonic,
      Operands::$op1_type,
      Operands::void,
    >::instruction(
      $instance,
      Operands::$op1_type::__type__,
      Operands::void::__type__,
    )
  );

  {$instance:expr, $mnemonic:ident $op1_literal:expr} => (
    Instruction::<
      Instructions::$mnemonic,
      Operands::literal,
      Operands::void,
    >::instruction(
      $instance,
      $op1_literal,
      Operands::void::__type__,
    )
  );

  // Arity 2

  {$instance:expr, $mnemonic:ident $op1_type:ident, $op2_type:ident} => (
    Instruction::<
      Instructions::$mnemonic,
      Operands::$op1_type,
      Operands::$op2_type,
    >::instruction(
      $instance,
      Operands::$op1_type::__type__,
      Operands::$op2_type::__type__,
    )
 );

  {$instance:expr, $mnemonic:ident ($op1_type:ident), $op2_type:ident} => (
    Instruction::<
      Instructions::$mnemonic,
      Operands::pointer<Operands::$op1_type>,
      Operands::$op2_type,
    >::instruction(
      $instance,
      Operands::pointer::<Operands::$op1_type>::__type__(Operands::$op1_type::__type__),
      Operands::$op2_type::__type__,
    )
 );

  {$instance:expr, $mnemonic:ident $op1_type:ident, ($op2_type:ident)} => (
    Instruction::<
      Instructions::$mnemonic,
      Operands::$op1_type,
      Operands::pointer<Operands::$op2_type>
    >::instruction(
      $instance,
      Operands::$op1_type::__type__,
      Operands::pointer::<Operands::$op2_type>::__type__(Operands::$op2_type::__type__),
    )
 );

  {$instance:expr, $mnemonic:ident $op1_literal:expr, $op2_type:ident} => ({
    Instruction::<
      Instructions::$mnemonic,
      Operands::literal,
      Operands::$op2_type,
    >::instruction(
      $instance,
      $op1_literal,
      Operands::$op2_type::__type__,
    )
  });

  {$instance:expr, $mnemonic:ident $op1_literal:expr, ($op2_type:ident)} => ({
    Instruction::<
      Instructions::$mnemonic,
      Operands::literal,
      pointer<$op2_type>,
    >::instruction(
      $instance,
      $op1_literal,
      Operands::pointer::<Operands::$op2_type>::__type__(Operands::$op2_type::__type__),
    )
  });
}

pub struct Processor {}

impl Processor {
  pub fn next(r: &mut Registers, mm: &mut MemoryMap) -> Cycle {
    Processor::process_instruction(r, mm)
  }

  fn process_instruction(r: &mut Registers, mm: &mut MemoryMap) -> Cycle {
    let opcode = Processor::eat_opcode(r, mm);
    match opcode {
      // 0x00 => instruction!{self, NOP},
    // 0x01 => instruction!{self, LD (BC),d16},
    // 0x02 => instruction!{self, LD (BC),A},
    // 0x03 => instruction!{self, INC BC},
    // 0x04 => instruction!{self, INC B},
    // 0x05 => instruction!{self, DEC B},
    // 0x06 => instruction!{self, LD B,d8},
    // 0x07 => instruction!{self, RLCA},
    // 0x08 => instruction!{self, LD (a16),SP},
    // 0x09 => instruction!{self, ADD HL,BC},
    // 0x0A => instruction!{self, LD A,(BC)},
    // 0x0B => instruction!{self, DEC BC},
    // 0x0C => instruction!{self, INC C},
    // 0x0D => instruction!{self, DEC C},
    // 0x0E => instruction!{self, LD C,d8},
    // 0x0F => instruction!{self, RRCA},

    // 0x10 => instruction!{self, STOP 0},
    // 0x11 => instruction!{self, LD DE,d16},
    // 0x12 => instruction!{self, LD (DE),A},
    // 0x13 => instruction!{self, INC DE},
    // 0x14 => instruction!{self, INC D},
    // 0x15 => instruction!{self, DEC D},
    // 0x16 => instruction!{self, LD D,d8},
    // 0x17 => instruction!{self, RLA},
    // 0x18 => instruction!{self, JR r8},
    // 0x19 => instruction!{self, ADD HL,DE},
    // 0x1A => instruction!{self, LD A,(DE)},
    // 0x1B => instruction!{self, DEC DE},
    // 0x1C => instruction!{self, INC E},
    // 0x1D => instruction!{self, DEC E},
    // 0x1E => instruction!{self, LD E,d8},
    // 0x1F => instruction!{self, RRA},

    // 0x20 => instruction!{self, JR NZ,r8},
    // 0x21 => instruction!{self, LD HL,d16},
    // 0x22 => instruction!{self, LDI (HL),A},
    // 0x23 => instruction!{self, INC HL},
    // 0x24 => instruction!{self, INC H},
    // 0x25 => instruction!{self, DEC H},
    // 0x26 => instruction!{self, LD H,d8},
    // 0x27 => instruction!{self, DAA},
    // 0x28 => instruction!{self, JR Z,r8},
    // 0x29 => instruction!{self, ADD HL,HL},
    // 0x2A => instruction!{self, LDI A, (HL)},
    // 0x2B => instruction!{self, DEC HL},
    // 0x2C => instruction!{self, INC L},
    // 0x2D => instruction!{self, DEC L},
    // 0x2E => instruction!{self, LD L,d8},
    // 0x2F => instruction!{self, CPL},

    // 0x30 => instruction!{self, JR NC,r8},
    // 0x31 => instruction!{self, LD SP,d16},
    // 0x32 => instruction!{self, LDD (HL),A},
    // 0x33 => instruction!{self, INC SP},
    // 0x34 => instruction!{self, INC (HL)},
    // 0x35 => instruction!{self, DEC (HL)},
    // 0x36 => instruction!{self, LD (HL),d8},
    // 0x37 => instruction!{self, SCF},
    // 0x38 => instruction!{self, JR C,r8},
    // 0x39 => instruction!{self, ADD HL,SP},
    // 0x3A => instruction!{self, LDD A,(HL)},
    // 0x3B => instruction!{self, DEC SP},
    // 0x3C => instruction!{self, INC A},
    // 0x3D => instruction!{self, DEC A},
    // 0x3E => instruction!{self, LD A,d8},
    // 0x3F => instruction!{self, CCF},

    // 0x40 => instruction!{self, LD B,B},
    // 0x41 => instruction!{self, LD B,C},
    // 0x42 => instruction!{self, LD B,D},
    // 0x43 => instruction!{self, LD B,E},
    // 0x44 => instruction!{self, LD B,H},
    // 0x45 => instruction!{self, LD B,L},
    // 0x46 => instruction!{self, LD B,(HL)},
    // 0x47 => instruction!{self, LD B,A},
    // 0x48 => instruction!{self, LD C,B},
    // 0x49 => instruction!{self, LD C,C},
    // 0x4A => instruction!{self, LD C,D},
    // 0x4B => instruction!{self, LD C,E},
    // 0x4C => instruction!{self, LD C,H},
    // 0x4D => instruction!{self, LD C,L},
    // 0x4E => instruction!{self, LD C,(HL)},
    // 0x4F => instruction!{self, LD C,A},

    // 0x50 => instruction!{self, LD D,B},
    // 0x51 => instruction!{self, LD D,C},
    // 0x52 => instruction!{self, LD D,D},
    // 0x53 => instruction!{self, LD D,E},
    // 0x54 => instruction!{self, LD D,H},
    // 0x55 => instruction!{self, LD D,L},
    // 0x56 => instruction!{self, LD D,(HL)},
    // 0x57 => instruction!{self, LD D,A},
    // 0x58 => instruction!{self, LD E,B},
    // 0x59 => instruction!{self, LD E,C},
    // 0x5A => instruction!{self, LD E,D},
    // 0x5B => instruction!{self, LD E,E},
    // 0x5C => instruction!{self, LD E,H},
    // 0x5D => instruction!{self, LD E,L},
    // 0x5E => instruction!{self, LD E,(HL)},
    // 0x5F => instruction!{self, LD E,A},

    // 0x60 => instruction!{self, LD H,B},
    // 0x61 => instruction!{self, LD H,C},
    // 0x62 => instruction!{self, LD H,D},
    // 0x63 => instruction!{self, LD H,E},
    // 0x64 => instruction!{self, LD H,H},
    // 0x65 => instruction!{self, LD H,L},
    // 0x66 => instruction!{self, LD H,(HL)},
    // 0x67 => instruction!{self, LD H,A},
    // 0x68 => instruction!{self, LD L,B},
    // 0x69 => instruction!{self, LD L,C},
    // 0x6A => instruction!{self, LD L,D},
    // 0x6B => instruction!{self, LD L,E},
    // 0x6C => instruction!{self, LD L,H},
    // 0x6D => instruction!{self, LD L,L},
    // 0x6E => instruction!{self, LD L,(HL)},
    // 0x6F => instruction!{self, LD L,A},

    // 0x70 => instruction!{self, LD (HL),B},
    // 0x71 => instruction!{self, LD (HL),C},
    // 0x72 => instruction!{self, LD (HL),D},
    // 0x73 => instruction!{self, LD (HL),E},
    // 0x74 => instruction!{self, LD (HL),H},
    // 0x75 => instruction!{self, LD (HL),L},
    // 0x76 => instruction!{self, HALT},
    // 0x77 => instruction!{self, LD (HL),A},
    // 0x78 => instruction!{self, LD A,B},
    // 0x79 => instruction!{self, LD A,C},
    // 0x7A => instruction!{self, LD A,D},
    // 0x7B => instruction!{self, LD A,E},
    // 0x7C => instruction!{self, LD A,H},
    // 0x7D => instruction!{self, LD A,L},
    // 0x7E => instruction!{self, LD A,(HL)},
    // 0x7F => instruction!{self, LD A,A},

    // 0x80 => instruction!{self, ADD A,B},
    // 0x81 => instruction!{self, ADD A,C},
    // 0x82 => instruction!{self, ADD A,D},
    // 0x83 => instruction!{self, ADD A,E},
    // 0x84 => instruction!{self, ADD A,H},
    // 0x85 => instruction!{self, ADD A,L},
    // 0x86 => instruction!{self, ADD A,(HL)},
    // 0x87 => instruction!{self, ADD A,A},
    // 0x88 => instruction!{self, ADC A,B},
    // 0x89 => instruction!{self, ADC A,C},
    // 0x8A => instruction!{self, ADC A,D},
    // 0x8B => instruction!{self, ADC A,E},
    // 0x8C => instruction!{self, ADC A,H},
    // 0x8D => instruction!{self, ADC A,L},
    // 0x8E => instruction!{self, ADC A,(HL)},
    // 0x8F => instruction!{self, ADC A,A},

    // 0x90 => instruction!{self, SUB B},
    // 0x91 => instruction!{self, SUB C},
    // 0x92 => instruction!{self, SUB D},
    // 0x93 => instruction!{self, SUB E},
    // 0x94 => instruction!{self, SUB H},
    // 0x95 => instruction!{self, SUB L},
    // 0x96 => instruction!{self, SUB (HL)},
    // 0x97 => instruction!{self, SUB A},
    // 0x98 => instruction!{self, SBC A,B},
    // 0x99 => instruction!{self, SBC A,C},
    // 0x9A => instruction!{self, SBC A,D},
    // 0x9B => instruction!{self, SBC A,E},
    // 0x9C => instruction!{self, SBC A,H},
    // 0x9D => instruction!{self, SBC A,L},
    // 0x9E => instruction!{self, SBC A,(HL)},
    // 0x9F => instruction!{self, SBC A,A},

    // 0xA0 => instruction!{self, AND B},
    // 0xA1 => instruction!{self, AND C},
    // 0xA2 => instruction!{self, AND D},
    // 0xA3 => instruction!{self, AND E},
    // 0xA4 => instruction!{self, AND H},
    // 0xA5 => instruction!{self, AND L},
    // 0xA6 => instruction!{self, AND (HL)},
    // 0xA7 => instruction!{self, AND A},
    // 0xA8 => instruction!{self, XOR B},
    // 0xA9 => instruction!{self, XOR C},
    // 0xAA => instruction!{self, XOR D},
    // 0xAB => instruction!{self, XOR E},
    // 0xAC => instruction!{self, XOR H},
    // 0xAD => instruction!{self, XOR L},
    // 0xAE => instruction!{self, XOR (HL)},
    // 0xAF => instruction!{self, XOR A},

    // 0xB0 => instruction!{self, OR B},
    // 0xB1 => instruction!{self, OR C},
    // 0xB2 => instruction!{self, OR D},
    // 0xB3 => instruction!{self, OR E},
    // 0xB4 => instruction!{self, OR H},
    // 0xB5 => instruction!{self, OR L},
    // 0xB6 => instruction!{self, OR (HL)},
    // 0xB7 => instruction!{self, OR A},
    // 0xB8 => instruction!{self, CP B},
    // 0xB9 => instruction!{self, CP C},
    // 0xBA => instruction!{self, CP D},
    // 0xBB => instruction!{self, CP E},
    // 0xBC => instruction!{self, CP H},
    // 0xBD => instruction!{self, CP L},
    // 0xBE => instruction!{self, CP (HL)},
    // 0xBF => instruction!{self, CP A},

    // 0xC0 => instruction!{self, RET NZ},
    // 0xC1 => instruction!{self, POP BC},
    // 0xC2 => instruction!{self, JP NZ,a16},
    // 0xC3 => instruction!{self, JP a16},
    // 0xC4 => instruction!{self, CALL NZ,a16},
    // 0xC5 => instruction!{self, PUSH BC},
    // 0xC6 => instruction!{self, ADD A,d8},
    // 0xC7 => instruction!{self, RST 0x00},
    // 0xC8 => instruction!{self, RET Z},
    // 0xC9 => instruction!{self, RET},
    // 0xCA => instruction!{self, JP Z,a16},
    // 0xCB => self.process_instruction_cb(),
    // 0xCC => instruction!{self, CALL Z,a16},
    // 0xCD => instruction!{self, CALL a16},
    // 0xCE => instruction!{self, ADC A,d8},
    // 0xCF => instruction!{self, RST 0x08},

    // 0xD0 => instruction!{self, RET NC},
    // 0xD1 => instruction!{self, POP DE},
    // 0xD2 => instruction!{self, JP NC,a16},
    // /* 0xD3 */
    // 0xD4 => instruction!{self, CALL NC,a16},
    // 0xD5 => instruction!{self, PUSH DE},
    // 0xD6 => instruction!{self, SUB d8},
    // 0xD7 => instruction!{self, RST 0x10},
    // 0xD8 => instruction!{self, RET C},
    // 0xD9 => instruction!{self, RETI},
    // 0xDA => instruction!{self, JP C,a16},
    // /* 0xDB */
    // 0xDC => instruction!{self, CALL C,a16},
    // /* 0xDD */
    // 0xDE => instruction!{self, SBC A,d8},
    // 0xDF => instruction!{self, RST 0x18},

    // 0xE0 => instruction!{self, LDH (a8),A},
    // 0xE1 => instruction!{self, POP HL},
    // 0xE2 => instruction!{self, LD (C),A},
    // /* 0xE3 */
    // /* 0xE4 */
    // 0xE5 => instruction!{self, PUSH HL},
    // 0xE6 => instruction!{self, AND d8},
    // 0xE7 => instruction!{self, RST 0x20},
    // 0xE8 => instruction!{self, ADD SP,r8},
    // 0xE9 => instruction!{self, JP (HL)},
    // 0xEA => instruction!{self, LD (a16),A},
    // /* 0xEB */
    // /* 0xEC */
    // /* 0xED */
    // 0xEE => instruction!{self, XOR d8},
    // 0xEF => instruction!{self, RST 0x28},

    // 0xF0 => instruction!{self, LDH A,(a8)},
    // 0xF1 => instruction!{self, POP AF},
    // 0xF2 => instruction!{self, LD A,(C)},
    // 0xF3 => instruction!{self, DI},
    // /* 0xF4 */
    // 0xF5 => instruction!{self, PUSH AF},
    // 0xF6 => instruction!{self, OR d8},
    // 0xF7 => instruction!{self, RST 0x30},
    // 0xF8 => instruction!{self, LDHL SP,r8},
    // 0xF9 => instruction!{self, LD SP,HL},
    // 0xFA => instruction!{self, LD A,(a16)},
    // 0xFB => instruction!{self, EI},
    // /* 0xFC */
    // /* 0xFD */
    // 0xFE => instruction!{self, CP d8},
    // 0xFF => instruction!{self, RST 0x38},
    //
      _ => panic!("Unsupported instruction 0x{:02X}", opcode),
    }
  }

  // fn process_instruction_cb(&mut self) -> u8 {
  //   let opcode = self.eat_opcode();
  //   match opcode {
  // 0x00 => instruction!{self, RLC B},
  // 0x01 => instruction!{self, RLC C},
  // 0x02 => instruction!{self, RLC D},
  // 0x03 => instruction!{self, RLC E},
  // 0x04 => instruction!{self, RLC H},
  // 0x05 => instruction!{self, RLC L},
  // 0x06 => instruction!{self, RLC (HL)},
  // 0x07 => instruction!{self, RLC A},
  // 0x08 => instruction!{self, RRC B},
  // 0x09 => instruction!{self, RRC C},
  // 0x0A => instruction!{self, RRC D},
  // 0x0B => instruction!{self, RRC E},
  // 0x0C => instruction!{self, RRC H},
  // 0x0D => instruction!{self, RRC L},
  // 0x0E => instruction!{self, RRC (HL)},
  // 0x0F => instruction!{self, RRC A},

  // 0x10 => instruction!{self, RL B},
  // 0x11 => instruction!{self, RL C},
  // 0x12 => instruction!{self, RL D},
  // 0x13 => instruction!{self, RL E},
  // 0x14 => instruction!{self, RL H},
  // 0x15 => instruction!{self, RL L},
  // 0x16 => instruction!{self, RL (HL)},
  // 0x17 => instruction!{self, RL A},
  // 0x18 => instruction!{self, RR B},
  // 0x19 => instruction!{self, RR C},
  // 0x1A => instruction!{self, RR D},
  // 0x1B => instruction!{self, RR E},
  // 0x1C => instruction!{self, RR H},
  // 0x1D => instruction!{self, RR L},
  // 0x1E => instruction!{self, RR (HL)},
  // 0x1F => instruction!{self, RR A},

  // 0x20 => instruction!{self, SLA B},
  // 0x21 => instruction!{self, SLA C},
  // 0x22 => instruction!{self, SLA D},
  // 0x23 => instruction!{self, SLA E},
  // 0x24 => instruction!{self, SLA H},
  // 0x25 => instruction!{self, SLA L},
  // 0x26 => instruction!{self, SLA (HL)},
  // 0x27 => instruction!{self, SLA A},
  // 0x28 => instruction!{self, SRA B},
  // 0x29 => instruction!{self, SRA C},
  // 0x2A => instruction!{self, SRA D},
  // 0x2B => instruction!{self, SRA E},
  // 0x2C => instruction!{self, SRA H},
  // 0x2D => instruction!{self, SRA L},
  // 0x2E => instruction!{self, SRA (HL)},
  // 0x2F => instruction!{self, SRA A},

  // 0x30 => instruction!{self, SWAP B},
  // 0x31 => instruction!{self, SWAP C},
  // 0x32 => instruction!{self, SWAP D},
  // 0x33 => instruction!{self, SWAP E},
  // 0x34 => instruction!{self, SWAP H},
  // 0x35 => instruction!{self, SWAP L},
  // 0x36 => instruction!{self, SWAP (HL)},
  // 0x37 => instruction!{self, SWAP A},
  // 0x38 => instruction!{self, SRL B},
  // 0x39 => instruction!{self, SRL C},
  // 0x3A => instruction!{self, SRL D},
  // 0x3B => instruction!{self, SRL E},
  // 0x3C => instruction!{self, SRL H},
  // 0x3D => instruction!{self, SRL L},
  // 0x3E => instruction!{self, SRL (HL)},
  // 0x3F => instruction!{self, SRL A},

  // 0x40 => instruction!{self, BIT 0,B},
  // 0x41 => instruction!{self, BIT 0,C},
  // 0x42 => instruction!{self, BIT 0,D},
  // 0x43 => instruction!{self, BIT 0,E},
  // 0x44 => instruction!{self, BIT 0,H},
  // 0x45 => instruction!{self, BIT 0,L},
  // 0x46 => instruction!{self, BIT 0,(HL)},
  // 0x47 => instruction!{self, BIT 0,A},
  // 0x48 => instruction!{self, BIT 1,B},
  // 0x49 => instruction!{self, BIT 1,C},
  // 0x4A => instruction!{self, BIT 1,D},
  // 0x4B => instruction!{self, BIT 1,E},
  // 0x4C => instruction!{self, BIT 1,H},
  // 0x4D => instruction!{self, BIT 1,L},
  // 0x4E => instruction!{self, BIT 1,(HL)},
  // 0x4F => instruction!{self, BIT 1,A},

  // 0x50 => instruction!{self, BIT 2,B},
  // 0x51 => instruction!{self, BIT 2,C},
  // 0x52 => instruction!{self, BIT 2,D},
  // 0x53 => instruction!{self, BIT 2,E},
  // 0x54 => instruction!{self, BIT 2,H},
  // 0x55 => instruction!{self, BIT 2,L},
  // 0x56 => instruction!{self, BIT 2,(HL)},
  // 0x57 => instruction!{self, BIT 2,A},
  // 0x58 => instruction!{self, BIT 3,B},
  // 0x59 => instruction!{self, BIT 3,C},
  // 0x5A => instruction!{self, BIT 3,D},
  // 0x5B => instruction!{self, BIT 3,E},
  // 0x5C => instruction!{self, BIT 3,H},
  // 0x5D => instruction!{self, BIT 3,L},
  // 0x5E => instruction!{self, BIT 3,(HL)},
  // 0x5F => instruction!{self, BIT 3,A},

  // 0x60 => instruction!{self, BIT 4,B},
  // 0x61 => instruction!{self, BIT 4,C},
  // 0x62 => instruction!{self, BIT 4,D},
  // 0x63 => instruction!{self, BIT 4,E},
  // 0x64 => instruction!{self, BIT 4,H},
  // 0x65 => instruction!{self, BIT 4,L},
  // 0x66 => instruction!{self, BIT 4,(HL)},
  // 0x67 => instruction!{self, BIT 4,A},
  // 0x68 => instruction!{self, BIT 5,B},
  // 0x69 => instruction!{self, BIT 5,C},
  // 0x6A => instruction!{self, BIT 5,D},
  // 0x6B => instruction!{self, BIT 5,E},
  // 0x6C => instruction!{self, BIT 5,H},
  // 0x6D => instruction!{self, BIT 5,L},
  // 0x6E => instruction!{self, BIT 5,(HL)},
  // 0x6F => instruction!{self, BIT 5,A},

  // 0x70 => instruction!{self, BIT 6,B},
  // 0x71 => instruction!{self, BIT 6,C},
  // 0x72 => instruction!{self, BIT 6,D},
  // 0x73 => instruction!{self, BIT 6,E},
  // 0x74 => instruction!{self, BIT 6,H},
  // 0x75 => instruction!{self, BIT 6,L},
  // 0x76 => instruction!{self, BIT 6,(HL)},
  // 0x77 => instruction!{self, BIT 6,A},
  // 0x78 => instruction!{self, BIT 7,B},
  // 0x79 => instruction!{self, BIT 7,C},
  // 0x7A => instruction!{self, BIT 7,D},
  // 0x7B => instruction!{self, BIT 7,E},
  // 0x7C => instruction!{self, BIT 7,H},
  // 0x7D => instruction!{self, BIT 7,L},
  // 0x7E => instruction!{self, BIT 7,(HL)},
  // 0x7F => instruction!{self, BIT 7,A},

  // 0x80 => instruction!{self, RES 0,B},
  // 0x81 => instruction!{self, RES 0,C},
  // 0x82 => instruction!{self, RES 0,D},
  // 0x83 => instruction!{self, RES 0,E},
  // 0x84 => instruction!{self, RES 0,H},
  // 0x85 => instruction!{self, RES 0,L},
  // 0x86 => instruction!{self, RES 0,(HL)},
  // 0x87 => instruction!{self, RES 0,A},
  // 0x88 => instruction!{self, RES 1,B},
  // 0x89 => instruction!{self, RES 1,C},
  // 0x8A => instruction!{self, RES 1,D},
  // 0x8B => instruction!{self, RES 1,E},
  // 0x8C => instruction!{self, RES 1,H},
  // 0x8D => instruction!{self, RES 1,L},
  // 0x8E => instruction!{self, RES 1,(HL)},
  // 0x8F => instruction!{self, RES 1,A},

  // 0x90 => instruction!{self, RES 2,B},
  // 0x91 => instruction!{self, RES 2,C},
  // 0x92 => instruction!{self, RES 2,D},
  // 0x93 => instruction!{self, RES 2,E},
  // 0x94 => instruction!{self, RES 2,H},
  // 0x95 => instruction!{self, RES 2,L},
  // 0x96 => instruction!{self, RES 2,(HL)},
  // 0x97 => instruction!{self, RES 2,A},
  // 0x98 => instruction!{self, RES 3,B},
  // 0x99 => instruction!{self, RES 3,C},
  // 0x9A => instruction!{self, RES 3,D},
  // 0x9B => instruction!{self, RES 3,E},
  // 0x9C => instruction!{self, RES 3,H},
  // 0x9D => instruction!{self, RES 3,L},
  // 0x9E => instruction!{self, RES 3,(HL)},
  // 0x9F => instruction!{self, RES 3,A},

  // 0xA0 => instruction!{self, RES 4,B},
  // 0xA1 => instruction!{self, RES 4,C},
  // 0xA2 => instruction!{self, RES 4,D},
  // 0xA3 => instruction!{self, RES 4,E},
  // 0xA4 => instruction!{self, RES 4,H},
  // 0xA5 => instruction!{self, RES 4,L},
  // 0xA6 => instruction!{self, RES 4,(HL)},
  // 0xA7 => instruction!{self, RES 4,A},
  // 0xA8 => instruction!{self, RES 5,B},
  // 0xA9 => instruction!{self, RES 5,C},
  // 0xAA => instruction!{self, RES 5,D},
  // 0xAB => instruction!{self, RES 5,E},
  // 0xAC => instruction!{self, RES 5,H},
  // 0xAD => instruction!{self, RES 5,L},
  // 0xAE => instruction!{self, RES 5,(HL)},
  // 0xAF => instruction!{self, RES 5,A},

  // 0xB0 => instruction!{self, RES 6,B},
  // 0xB1 => instruction!{self, RES 6,C},
  // 0xB2 => instruction!{self, RES 6,D},
  // 0xB3 => instruction!{self, RES 6,E},
  // 0xB4 => instruction!{self, RES 6,H},
  // 0xB5 => instruction!{self, RES 6,L},
  // 0xB6 => instruction!{self, RES 6,(HL)},
  // 0xB7 => instruction!{self, RES 6,A},
  // 0xB8 => instruction!{self, RES 7,B},
  // 0xB9 => instruction!{self, RES 7,C},
  // 0xBA => instruction!{self, RES 7,D},
  // 0xBB => instruction!{self, RES 7,E},
  // 0xBC => instruction!{self, RES 7,H},
  // 0xBD => instruction!{self, RES 7,L},
  // 0xBE => instruction!{self, RES 7,(HL)},
  // 0xBF => instruction!{self, RES 7,A},

  // 0xC0 => instruction!{self, SET 0,B},
  // 0xC1 => instruction!{self, SET 0,C},
  // 0xC2 => instruction!{self, SET 0,D},
  // 0xC3 => instruction!{self, SET 0,E},
  // 0xC4 => instruction!{self, SET 0,H},
  // 0xC5 => instruction!{self, SET 0,L},
  // 0xC6 => instruction!{self, SET 0,(HL)},
  // 0xC7 => instruction!{self, SET 0,A},
  // 0xC8 => instruction!{self, SET 1,B},
  // 0xC9 => instruction!{self, SET 1,C},
  // 0xCA => instruction!{self, SET 1,D},
  // 0xCB => instruction!{self, SET 1,E},
  // 0xCC => instruction!{self, SET 1,H},
  // 0xCD => instruction!{self, SET 1,L},
  // 0xCE => instruction!{self, SET 1,(HL)},
  // 0xCF => instruction!{self, SET 1,A},

  // 0xD0 => instruction!{self, SET 2,B},
  // 0xD1 => instruction!{self, SET 2,C},
  // 0xD2 => instruction!{self, SET 2,D},
  // 0xD3 => instruction!{self, SET 2,E},
  // 0xD4 => instruction!{self, SET 2,H},
  // 0xD5 => instruction!{self, SET 2,L},
  // 0xD6 => instruction!{self, SET 2,(HL)},
  // 0xD7 => instruction!{self, SET 2,A},
  // 0xD8 => instruction!{self, SET 3,B},
  // 0xD9 => instruction!{self, SET 3,C},
  // 0xDA => instruction!{self, SET 3,D},
  // 0xDB => instruction!{self, SET 3,E},
  // 0xDC => instruction!{self, SET 3,H},
  // 0xDD => instruction!{self, SET 3,L},
  // 0xDE => instruction!{self, SET 3,(HL)},
  // 0xDF => instruction!{self, SET 3,A},

  // 0xE0 => instruction!{self, SET 4,B},
  // 0xE1 => instruction!{self, SET 4,C},
  // 0xE2 => instruction!{self, SET 4,D},
  // 0xE3 => instruction!{self, SET 4,E},
  // 0xE4 => instruction!{self, SET 4,H},
  // 0xE5 => instruction!{self, SET 4,L},
  // 0xE6 => instruction!{self, SET 4,(HL)},
  // 0xE7 => instruction!{self, SET 4,A},
  // 0xE8 => instruction!{self, SET 5,B},
  // 0xE9 => instruction!{self, SET 5,C},
  // 0xEA => instruction!{self, SET 5,D},
  // 0xEB => instruction!{self, SET 5,E},
  // 0xEC => instruction!{self, SET 5,H},
  // 0xED => instruction!{self, SET 5,L},
  // 0xEE => instruction!{self, SET 5,(HL)},
  // 0xEF => instruction!{self, SET 5,A},

  // 0xF0 => instruction!{self, SET 6,B},
  // 0xF1 => instruction!{self, SET 6,C},
  // 0xF2 => instruction!{self, SET 6,D},
  // 0xF3 => instruction!{self, SET 6,E},
  // 0xF4 => instruction!{self, SET 6,H},
  // 0xF5 => instruction!{self, SET 6,L},
  // 0xF6 => instruction!{self, SET 6,(HL)},
  // 0xF7 => instruction!{self, SET 6,A},
  // 0xF8 => instruction!{self, SET 7,B},
  // 0xF9 => instruction!{self, SET 7,C},
  // 0xFA => instruction!{self, SET 7,D},
  // 0xFB => instruction!{self, SET 7,E},
  // 0xFC => instruction!{self, SET 7,H},
  // 0xFD => instruction!{self, SET 7,L},
  // 0xFE => instruction!{self, SET 7,(HL)},
  // 0xFF => instruction!{self, SET 7,A},
  //
  //     _ => panic!("Unsupported instruction 0xCB{:02X}", opcode),
  //   }
  // }

  fn eat_opcode(r: &mut Registers, mm: &MemoryMap) -> Opcode {
    let opcode = mm.read_u8(r.pc as usize);
    print!("0xCB{:02X}:", opcode);
    r.sp += 1;
    opcode
  }

  // fn stack_push_u16(&mut self, value: u16) {
  //   self.registers.sp -= size_of_val(&value) as u16;
  //   self.memory_map.write_u16(self.registers.sp as usize, value);
  // }

  // fn stack_pop_u16(&mut self) -> u16 {
  //   let value = self.memory_map.read_u16(self.registers.sp as usize);
  //   self.registers.sp += size_of_val(&value) as u16;
  //   value
  // }
}
