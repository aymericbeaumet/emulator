mod call;
mod cpl;
mod dec;
mod jp;
mod jr;
mod ld;
mod ldh;
mod ldi;
mod xor;

use std::mem::size_of_val;

use super::memory_map::{FromMemoryMap, MemoryMap, ToMemoryMap};
use super::registers::Registers;

pub type Cycle = u8;

pub trait Instruction<T, U, V> {
  fn instruction(&mut Registers, &mut MemoryMap, _: U, _: V) -> Cycle;
}

pub mod instructions {
  pub enum CALL {}
  pub enum CPL {}
  pub enum DEC {}
  pub enum JP {}
  pub enum JR {}
  pub enum LD {}
  pub enum LDH {}
  pub enum LDI {}
  pub enum XOR {}
}

pub mod operands {
  pub enum Void {
    Type,
  }
  pub enum A {
    Type,
  }
  pub enum B {
    Type,
  }
  pub enum C {
    Type,
  }
  pub enum D {
    Type,
  }
  pub enum E {
    Type,
  }
  pub enum H {
    Type,
  }
  pub enum L {
    Type,
  }
  pub enum BC {
    Type,
  }
  pub enum DE {
    Type,
  }
  pub enum HL {
    Type,
  }
  pub enum SP {
    Type,
  }
  pub enum PC {
    Type,
  }
  pub enum D8 {
    Type,
  }
  pub enum D16 {
    Type,
  }
  pub enum A8 {
    Type,
  }
  pub enum A16 {
    Type,
  }
  pub enum R8 {
    Type,
  }
  pub enum NZ {
    Type,
  }
  pub type Literal = u8;
  pub enum Pointer<T> {
    Type(T),
  }
}

macro_rules! instruction {
  // Arity 0

  {$r:expr, $mm:expr => $mnemonic:ident} => (
    <Processor as Instruction<
      instructions::$mnemonic,
      operands::Void,
      operands::Void,
    >>::instruction($r, $mm,
      operands::Void::Type,
      operands::Void::Type,
    )
  );

  // Arity 1

  {$r:expr, $mm:expr => $mnemonic:ident $op1_type:ident} => (
    <Processor as Instruction<
      instructions::$mnemonic,
      operands::$op1_type,
      operands::Void,
    >>::instruction($r, $mm,
      operands::$op1_type::Type,
      operands::Void::Type,
    )
  );

  {$r:expr, $mm:expr => $mnemonic:ident $op1_literal:expr} => (
    <Processor as Instruction<
      instructions::$mnemonic,
      operands::Literal,
      operands::Void,
    >>::instruction($r, $mm,
      $op1_literal,
      operands::Void::Type,
    )
  );

  // Arity 2

  {$r:expr, $mm:expr => $mnemonic:ident $op1_type:ident, $op2_type:ident} => (
    <Processor as Instruction<
      instructions::$mnemonic,
      operands::$op1_type,
      operands::$op2_type,
    >>::instruction($r, $mm,
      operands::$op1_type::Type,
      operands::$op2_type::Type,
    )
  );

  {$r:expr, $mm:expr => $mnemonic:ident ($op1_type:ident), $op2_type:ident} => (
    <Processor as Instruction<
      instructions::$mnemonic,
      operands::Pointer<operands::$op1_type>,
      operands::$op2_type,
    >>::instruction($r, $mm,
      operands::Pointer::Type(operands::$op1_type::Type),
      operands::$op2_type::Type,
    )
 );

  {$r:expr, $mm:expr => $mnemonic:ident $op1_type:ident, ($op2_type:ident)} => (
    <Processor as Instruction<
      instructions::$mnemonic,
      operands::$op1_type,
      operands::Pointer<operands::$op2_type>
    >>::instruction($r, $mm,
      operands::$op1_type::Type,
      operands::Pointer::Type(operands::$op2_type::Type),
    )
 );

  {$r:expr, $mm:expr => $mnemonic:ident $op1_literal:expr, $op2_type:ident} => (
    <Processor as Instruction<
      instructions::$mnemonic,
      operands::Literal,
      operands::$op2_type,
    >>::instruction($r, $mm,
      $op1_literal,
      operands::$op2_type::Type,
    )
 );

  {$r:expr, $mm:expr => $mnemonic:ident $op1_literal:expr, ($op2_type:ident)} => (
    <Processor as Instruction<
      instructions::$mnemonic,
      operands::Literal,
      Pointer<operands::$op2_type>,
    >>::instruction($r, $mm,
      $op1_literal,
      operands::Pointer::Type(operands::$op2_type::Type),
    )
 );
}

pub struct Processor {}

impl Processor {
  pub fn step(r: &mut Registers, mm: &mut MemoryMap) -> Cycle {
    Processor::process_instruction(r, mm)
  }

  fn process_instruction(r: &mut Registers, mm: &mut MemoryMap) -> Cycle {
    let opcode: u8 = Processor::eat(r, mm);
    match opcode {
      // 0x00 => instruction!{r, mm => NOP},
      // 0x01 => instruction!{r, mm => LD (BC),D16},
      // 0x02 => instruction!{r, mm => LD (BC),A},
      // 0x03 => instruction!{r, mm => INC BC},
      // 0x04 => instruction!{r, mm => INC B},
      // 0x05 => instruction!{r, mm => DEC B},
      // 0x06 => instruction!{r, mm => LD B,D8},
      // 0x07 => instruction!{r, mm => RLCA},
      // 0x08 => instruction!{r, mm => LD (A16),SP},
      // 0x09 => instruction!{r, mm => ADD HL,BC},
      // 0x0A => instruction!{r, mm => LD A,(BC)},
      // 0x0B => instruction!{r, mm => DEC BC},
      // 0x0C => instruction!{r, mm => INC C},
      0x0D => instruction!{r, mm => DEC C},
      0x0E => instruction!{r, mm => LD C,D8},
      // 0x0F => instruction!{r, mm => RRCA},

    // 0x10 => instruction!{r, mm => STOP 0},
    // 0x11 => instruction!{r, mm => LD DE,D16},
    // 0x12 => instruction!{r, mm => LD (DE),A},
    // 0x13 => instruction!{r, mm => INC DE},
    // 0x14 => instruction!{r, mm => INC D},
    // 0x15 => instruction!{r, mm => DEC D},
    // 0x16 => instruction!{r, mm => LD D,D8},
    // 0x17 => instruction!{r, mm => RLA},
    // 0x18 => instruction!{r, mm => JR R8},
    // 0x19 => instruction!{r, mm => ADD HL,DE},
    // 0x1A => instruction!{r, mm => LD A,(DE)},
    // 0x1B => instruction!{r, mm => DEC DE},
    // 0x1C => instruction!{r, mm => INC E},
    // 0x1D => instruction!{r, mm => DEC E},
    // 0x1E => instruction!{r, mm => LD E,D8},
    // 0x1F => instruction!{r, mm => RRA},
      0x20 => instruction!{r, mm => JR NZ,R8},
      0x21 => instruction!{r, mm => LD HL,D16},
      0x22 => instruction!{r, mm => LDI (HL),A},
      // 0x23 => instruction!{r, mm => INC HL},
      // 0x24 => instruction!{r, mm => INC H},
      // 0x25 => instruction!{r, mm => DEC H},
      // 0x26 => instruction!{r, mm => LD H,D8},
      // 0x27 => instruction!{r, mm => DAA},
      // 0x28 => instruction!{r, mm => JR Z,R8},
      // 0x29 => instruction!{r, mm => ADD HL,HL},
      // 0x2A => instruction!{r, mm => LDI A, (HL)},
      // 0x2B => instruction!{r, mm => DEC HL},
      // 0x2C => instruction!{r, mm => INC L},
      // 0x2D => instruction!{r, mm => DEC L},
      // 0x2E => instruction!{r, mm => LD L,D8},
      0x2F => instruction!{r, mm => CPL},

      // 0x30 => instruction!{r, mm => JR NC,R8},
      0x31 => instruction!{r, mm => LD SP,D16},
      // 0x32 => instruction!{r, mm => LDD (HL),A},
      // 0x33 => instruction!{r, mm => INC SP},
      // 0x34 => instruction!{r, mm => INC (HL)},
      // 0x35 => instruction!{r, mm => DEC (HL)},
      // 0x36 => instruction!{r, mm => LD (HL),D8},
      // 0x37 => instruction!{r, mm => SCF},
      // 0x38 => instruction!{r, mm => JR C,R8},
      // 0x39 => instruction!{r, mm => ADD HL,SP},
      // 0x3A => instruction!{r, mm => LDD A,(HL)},
      // 0x3B => instruction!{r, mm => DEC SP},
      // 0x3C => instruction!{r, mm => INC A},
      // 0x3D => instruction!{r, mm => DEC A},
      0x3E => instruction!{r, mm => LD A,D8},
      // 0x3F => instruction!{r, mm => CCF},

    // 0x40 => instruction!{r, mm => LD B,B},
    // 0x41 => instruction!{r, mm => LD B,C},
    // 0x42 => instruction!{r, mm => LD B,D},
    // 0x43 => instruction!{r, mm => LD B,E},
    // 0x44 => instruction!{r, mm => LD B,H},
    // 0x45 => instruction!{r, mm => LD B,L},
    // 0x46 => instruction!{r, mm => LD B,(HL)},
    // 0x47 => instruction!{r, mm => LD B,A},
    // 0x48 => instruction!{r, mm => LD C,B},
    // 0x49 => instruction!{r, mm => LD C,C},
    // 0x4A => instruction!{r, mm => LD C,D},
    // 0x4B => instruction!{r, mm => LD C,E},
    // 0x4C => instruction!{r, mm => LD C,H},
    // 0x4D => instruction!{r, mm => LD C,L},
    // 0x4E => instruction!{r, mm => LD C,(HL)},
    // 0x4F => instruction!{r, mm => LD C,A},

    // 0x50 => instruction!{r, mm => LD D,B},
    // 0x51 => instruction!{r, mm => LD D,C},
    // 0x52 => instruction!{r, mm => LD D,D},
    // 0x53 => instruction!{r, mm => LD D,E},
    // 0x54 => instruction!{r, mm => LD D,H},
    // 0x55 => instruction!{r, mm => LD D,L},
    // 0x56 => instruction!{r, mm => LD D,(HL)},
    // 0x57 => instruction!{r, mm => LD D,A},
    // 0x58 => instruction!{r, mm => LD E,B},
    // 0x59 => instruction!{r, mm => LD E,C},
    // 0x5A => instruction!{r, mm => LD E,D},
    // 0x5B => instruction!{r, mm => LD E,E},
    // 0x5C => instruction!{r, mm => LD E,H},
    // 0x5D => instruction!{r, mm => LD E,L},
    // 0x5E => instruction!{r, mm => LD E,(HL)},
    // 0x5F => instruction!{r, mm => LD E,A},

    // 0x60 => instruction!{r, mm => LD H,B},
    // 0x61 => instruction!{r, mm => LD H,C},
    // 0x62 => instruction!{r, mm => LD H,D},
    // 0x63 => instruction!{r, mm => LD H,E},
    // 0x64 => instruction!{r, mm => LD H,H},
    // 0x65 => instruction!{r, mm => LD H,L},
    // 0x66 => instruction!{r, mm => LD H,(HL)},
    // 0x67 => instruction!{r, mm => LD H,A},
    // 0x68 => instruction!{r, mm => LD L,B},
    // 0x69 => instruction!{r, mm => LD L,C},
    // 0x6A => instruction!{r, mm => LD L,D},
    // 0x6B => instruction!{r, mm => LD L,E},
    // 0x6C => instruction!{r, mm => LD L,H},
    // 0x6D => instruction!{r, mm => LD L,L},
    // 0x6E => instruction!{r, mm => LD L,(HL)},
    // 0x6F => instruction!{r, mm => LD L,A},

    // 0x70 => instruction!{r, mm => LD (HL),B},
    // 0x71 => instruction!{r, mm => LD (HL),C},
    // 0x72 => instruction!{r, mm => LD (HL),D},
    // 0x73 => instruction!{r, mm => LD (HL),E},
    // 0x74 => instruction!{r, mm => LD (HL),H},
    // 0x75 => instruction!{r, mm => LD (HL),L},
    // 0x76 => instruction!{r, mm => HALT},
    // 0x77 => instruction!{r, mm => LD (HL),A},
    // 0x78 => instruction!{r, mm => LD A,B},
    // 0x79 => instruction!{r, mm => LD A,C},
    // 0x7A => instruction!{r, mm => LD A,D},
    // 0x7B => instruction!{r, mm => LD A,E},
    // 0x7C => instruction!{r, mm => LD A,H},
    // 0x7D => instruction!{r, mm => LD A,L},
    // 0x7E => instruction!{r, mm => LD A,(HL)},
    // 0x7F => instruction!{r, mm => LD A,A},

    // 0x80 => instruction!{r, mm => ADD A,B},
    // 0x81 => instruction!{r, mm => ADD A,C},
    // 0x82 => instruction!{r, mm => ADD A,D},
    // 0x83 => instruction!{r, mm => ADD A,E},
    // 0x84 => instruction!{r, mm => ADD A,H},
    // 0x85 => instruction!{r, mm => ADD A,L},
    // 0x86 => instruction!{r, mm => ADD A,(HL)},
    // 0x87 => instruction!{r, mm => ADD A,A},
    // 0x88 => instruction!{r, mm => ADC A,B},
    // 0x89 => instruction!{r, mm => ADC A,C},
    // 0x8A => instruction!{r, mm => ADC A,D},
    // 0x8B => instruction!{r, mm => ADC A,E},
    // 0x8C => instruction!{r, mm => ADC A,H},
    // 0x8D => instruction!{r, mm => ADC A,L},
    // 0x8E => instruction!{r, mm => ADC A,(HL)},
    // 0x8F => instruction!{r, mm => ADC A,A},

    // 0x90 => instruction!{r, mm => SUB B},
    // 0x91 => instruction!{r, mm => SUB C},
    // 0x92 => instruction!{r, mm => SUB D},
    // 0x93 => instruction!{r, mm => SUB E},
    // 0x94 => instruction!{r, mm => SUB H},
    // 0x95 => instruction!{r, mm => SUB L},
    // 0x96 => instruction!{r, mm => SUB (HL)},
    // 0x97 => instruction!{r, mm => SUB A},
    // 0x98 => instruction!{r, mm => SBC A,B},
    // 0x99 => instruction!{r, mm => SBC A,C},
    // 0x9A => instruction!{r, mm => SBC A,D},
    // 0x9B => instruction!{r, mm => SBC A,E},
    // 0x9C => instruction!{r, mm => SBC A,H},
    // 0x9D => instruction!{r, mm => SBC A,L},
    // 0x9E => instruction!{r, mm => SBC A,(HL)},
    // 0x9F => instruction!{r, mm => SBC A,A},

    // 0xA0 => instruction!{r, mm => AND B},
    // 0xA1 => instruction!{r, mm => AND C},
    // 0xA2 => instruction!{r, mm => AND D},
    // 0xA3 => instruction!{r, mm => AND E},
    // 0xA4 => instruction!{r, mm => AND H},
    // 0xA5 => instruction!{r, mm => AND L},
    // 0xA6 => instruction!{r, mm => AND (HL)},
    // 0xA7 => instruction!{r, mm => AND A},
    // 0xA8 => instruction!{r, mm => XOR B},
    // 0xA9 => instruction!{r, mm => XOR C},
    // 0xAA => instruction!{r, mm => XOR D},
    // 0xAB => instruction!{r, mm => XOR E},
    // 0xAC => instruction!{r, mm => XOR H},
    // 0xAD => instruction!{r, mm => XOR L},
    // 0xAE => instruction!{r, mm => XOR (HL)},
      0xAF => instruction!{r, mm => XOR A},

      // 0xB0 => instruction!{r, mm => OR B},
    // 0xB1 => instruction!{r, mm => OR C},
    // 0xB2 => instruction!{r, mm => OR D},
    // 0xB3 => instruction!{r, mm => OR E},
    // 0xB4 => instruction!{r, mm => OR H},
    // 0xB5 => instruction!{r, mm => OR L},
    // 0xB6 => instruction!{r, mm => OR (HL)},
    // 0xB7 => instruction!{r, mm => OR A},
    // 0xB8 => instruction!{r, mm => CP B},
    // 0xB9 => instruction!{r, mm => CP C},
    // 0xBA => instruction!{r, mm => CP D},
    // 0xBB => instruction!{r, mm => CP E},
    // 0xBC => instruction!{r, mm => CP H},
    // 0xBD => instruction!{r, mm => CP L},
    // 0xBE => instruction!{r, mm => CP (HL)},
    // 0xBF => instruction!{r, mm => CP A},

    // 0xC0 => instruction!{r, mm => RET NZ},
    // 0xC1 => instruction!{r, mm => POP BC},
    // 0xC2 => instruction!{r, mm => JP NZ,A16},
      0xC3 => instruction!{r, mm => JP A16},
      // 0xC4 => instruction!{r, mm => CALL NZ,A16},
      // 0xC5 => instruction!{r, mm => PUSH BC},
      // 0xC6 => instruction!{r, mm => ADD A,D8},
      // 0xC7 => instruction!{r, mm => RST 0x00},
      // 0xC8 => instruction!{r, mm => RET Z},
      // 0xC9 => instruction!{r, mm => RET},
      // 0xCA => instruction!{r, mm => JP Z,A16},
      // 0xCB => self.process_instruction_cb(),
      // 0xCC => instruction!{r, mm => CALL Z,A16},
      0xCD => instruction!{r, mm => CALL A16},
      // 0xCE => instruction!{r, mm => ADC A,D8},
    // 0xCF => instruction!{r, mm => RST 0x08},

    // 0xD0 => instruction!{r, mm => RET NC},
    // 0xD1 => instruction!{r, mm => POP DE},
    // 0xD2 => instruction!{r, mm => JP NC,A16},
    // /* 0xD3 */
    // 0xD4 => instruction!{r, mm => CALL NC,A16},
    // 0xD5 => instruction!{r, mm => PUSH DE},
    // 0xD6 => instruction!{r, mm => SUB D8},
    // 0xD7 => instruction!{r, mm => RST 0x10},
    // 0xD8 => instruction!{r, mm => RET C},
    // 0xD9 => instruction!{r, mm => RETI},
    // 0xDA => instruction!{r, mm => JP C,A16},
    // /* 0xDB */
    // 0xDC => instruction!{r, mm => CALL C,A16},
    // /* 0xDD */
    // 0xDE => instruction!{r, mm => SBC A,D8},
    // 0xDF => instruction!{r, mm => RST 0x18},
    //
      0xE0 => instruction!{r, mm => LDH (A8),A},
      // 0xE1 => instruction!{r, mm => POP HL},
    // 0xE2 => instruction!{r, mm => LD (C),A},
    // /* 0xE3 */
    // /* 0xE4 */
    // 0xE5 => instruction!{r, mm => PUSH HL},
    // 0xE6 => instruction!{r, mm => AND D8},
    // 0xE7 => instruction!{r, mm => RST 0x20},
    // 0xE8 => instruction!{r, mm => ADD SP,R8},
    // 0xE9 => instruction!{r, mm => JP (HL)},
    // 0xEA => instruction!{r, mm => LD (A16),A},
    // /* 0xEB */
    // /* 0xEC */
    // /* 0xED */
    // 0xEE => instruction!{r, mm => XOR D8},
    // 0xEF => instruction!{r, mm => RST 0x28},

    // 0xF0 => instruction!{r, mm => LDH A,(A8)},
    // 0xF1 => instruction!{r, mm => POP AF},
    // 0xF2 => instruction!{r, mm => LD A,(C)},
    // 0xF3 => instruction!{r, mm => DI},
    // /* 0xF4 */
    // 0xF5 => instruction!{r, mm => PUSH AF},
    // 0xF6 => instruction!{r, mm => OR D8},
    // 0xF7 => instruction!{r, mm => RST 0x30},
    // 0xF8 => instruction!{r, mm => LDHL SP,R8},
    // 0xF9 => instruction!{r, mm => LD SP,HL},
    // 0xFA => instruction!{r, mm => LD A,(A16)},
    // 0xFB => instruction!{r, mm => EI},
    // /* 0xFC */
    // /* 0xFD */
    // 0xFE => instruction!{r, mm => CP D8},
    // 0xFF => instruction!{r, mm => RST 0x38},
    //
      _ => panic!("Unsupported instruction 0x{:02X}", opcode),
    }
  }

  // fn process_instruction_cb(&mut self) -> u8 {
  //   let opcode = Processor::eat();
  //   match opcode {
  // 0x00 => instruction!{r, mm => RLC B},
  // 0x01 => instruction!{r, mm => RLC C},
  // 0x02 => instruction!{r, mm => RLC D},
  // 0x03 => instruction!{r, mm => RLC E},
  // 0x04 => instruction!{r, mm => RLC H},
  // 0x05 => instruction!{r, mm => RLC L},
  // 0x06 => instruction!{r, mm => RLC (HL)},
  // 0x07 => instruction!{r, mm => RLC A},
  // 0x08 => instruction!{r, mm => RRC B},
  // 0x09 => instruction!{r, mm => RRC C},
  // 0x0A => instruction!{r, mm => RRC D},
  // 0x0B => instruction!{r, mm => RRC E},
  // 0x0C => instruction!{r, mm => RRC H},
  // 0x0D => instruction!{r, mm => RRC L},
  // 0x0E => instruction!{r, mm => RRC (HL)},
  // 0x0F => instruction!{r, mm => RRC A},

  // 0x10 => instruction!{r, mm => RL B},
  // 0x11 => instruction!{r, mm => RL C},
  // 0x12 => instruction!{r, mm => RL D},
  // 0x13 => instruction!{r, mm => RL E},
  // 0x14 => instruction!{r, mm => RL H},
  // 0x15 => instruction!{r, mm => RL L},
  // 0x16 => instruction!{r, mm => RL (HL)},
  // 0x17 => instruction!{r, mm => RL A},
  // 0x18 => instruction!{r, mm => RR B},
  // 0x19 => instruction!{r, mm => RR C},
  // 0x1A => instruction!{r, mm => RR D},
  // 0x1B => instruction!{r, mm => RR E},
  // 0x1C => instruction!{r, mm => RR H},
  // 0x1D => instruction!{r, mm => RR L},
  // 0x1E => instruction!{r, mm => RR (HL)},
  // 0x1F => instruction!{r, mm => RR A},

  // 0x20 => instruction!{r, mm => SLA B},
  // 0x21 => instruction!{r, mm => SLA C},
  // 0x22 => instruction!{r, mm => SLA D},
  // 0x23 => instruction!{r, mm => SLA E},
  // 0x24 => instruction!{r, mm => SLA H},
  // 0x25 => instruction!{r, mm => SLA L},
  // 0x26 => instruction!{r, mm => SLA (HL)},
  // 0x27 => instruction!{r, mm => SLA A},
  // 0x28 => instruction!{r, mm => SRA B},
  // 0x29 => instruction!{r, mm => SRA C},
  // 0x2A => instruction!{r, mm => SRA D},
  // 0x2B => instruction!{r, mm => SRA E},
  // 0x2C => instruction!{r, mm => SRA H},
  // 0x2D => instruction!{r, mm => SRA L},
  // 0x2E => instruction!{r, mm => SRA (HL)},
  // 0x2F => instruction!{r, mm => SRA A},

  // 0x30 => instruction!{r, mm => SWAP B},
  // 0x31 => instruction!{r, mm => SWAP C},
  // 0x32 => instruction!{r, mm => SWAP D},
  // 0x33 => instruction!{r, mm => SWAP E},
  // 0x34 => instruction!{r, mm => SWAP H},
  // 0x35 => instruction!{r, mm => SWAP L},
  // 0x36 => instruction!{r, mm => SWAP (HL)},
  // 0x37 => instruction!{r, mm => SWAP A},
  // 0x38 => instruction!{r, mm => SRL B},
  // 0x39 => instruction!{r, mm => SRL C},
  // 0x3A => instruction!{r, mm => SRL D},
  // 0x3B => instruction!{r, mm => SRL E},
  // 0x3C => instruction!{r, mm => SRL H},
  // 0x3D => instruction!{r, mm => SRL L},
  // 0x3E => instruction!{r, mm => SRL (HL)},
  // 0x3F => instruction!{r, mm => SRL A},

  // 0x40 => instruction!{r, mm => BIT 0,B},
  // 0x41 => instruction!{r, mm => BIT 0,C},
  // 0x42 => instruction!{r, mm => BIT 0,D},
  // 0x43 => instruction!{r, mm => BIT 0,E},
  // 0x44 => instruction!{r, mm => BIT 0,H},
  // 0x45 => instruction!{r, mm => BIT 0,L},
  // 0x46 => instruction!{r, mm => BIT 0,(HL)},
  // 0x47 => instruction!{r, mm => BIT 0,A},
  // 0x48 => instruction!{r, mm => BIT 1,B},
  // 0x49 => instruction!{r, mm => BIT 1,C},
  // 0x4A => instruction!{r, mm => BIT 1,D},
  // 0x4B => instruction!{r, mm => BIT 1,E},
  // 0x4C => instruction!{r, mm => BIT 1,H},
  // 0x4D => instruction!{r, mm => BIT 1,L},
  // 0x4E => instruction!{r, mm => BIT 1,(HL)},
  // 0x4F => instruction!{r, mm => BIT 1,A},

  // 0x50 => instruction!{r, mm => BIT 2,B},
  // 0x51 => instruction!{r, mm => BIT 2,C},
  // 0x52 => instruction!{r, mm => BIT 2,D},
  // 0x53 => instruction!{r, mm => BIT 2,E},
  // 0x54 => instruction!{r, mm => BIT 2,H},
  // 0x55 => instruction!{r, mm => BIT 2,L},
  // 0x56 => instruction!{r, mm => BIT 2,(HL)},
  // 0x57 => instruction!{r, mm => BIT 2,A},
  // 0x58 => instruction!{r, mm => BIT 3,B},
  // 0x59 => instruction!{r, mm => BIT 3,C},
  // 0x5A => instruction!{r, mm => BIT 3,D},
  // 0x5B => instruction!{r, mm => BIT 3,E},
  // 0x5C => instruction!{r, mm => BIT 3,H},
  // 0x5D => instruction!{r, mm => BIT 3,L},
  // 0x5E => instruction!{r, mm => BIT 3,(HL)},
  // 0x5F => instruction!{r, mm => BIT 3,A},

  // 0x60 => instruction!{r, mm => BIT 4,B},
  // 0x61 => instruction!{r, mm => BIT 4,C},
  // 0x62 => instruction!{r, mm => BIT 4,D},
  // 0x63 => instruction!{r, mm => BIT 4,E},
  // 0x64 => instruction!{r, mm => BIT 4,H},
  // 0x65 => instruction!{r, mm => BIT 4,L},
  // 0x66 => instruction!{r, mm => BIT 4,(HL)},
  // 0x67 => instruction!{r, mm => BIT 4,A},
  // 0x68 => instruction!{r, mm => BIT 5,B},
  // 0x69 => instruction!{r, mm => BIT 5,C},
  // 0x6A => instruction!{r, mm => BIT 5,D},
  // 0x6B => instruction!{r, mm => BIT 5,E},
  // 0x6C => instruction!{r, mm => BIT 5,H},
  // 0x6D => instruction!{r, mm => BIT 5,L},
  // 0x6E => instruction!{r, mm => BIT 5,(HL)},
  // 0x6F => instruction!{r, mm => BIT 5,A},

  // 0x70 => instruction!{r, mm => BIT 6,B},
  // 0x71 => instruction!{r, mm => BIT 6,C},
  // 0x72 => instruction!{r, mm => BIT 6,D},
  // 0x73 => instruction!{r, mm => BIT 6,E},
  // 0x74 => instruction!{r, mm => BIT 6,H},
  // 0x75 => instruction!{r, mm => BIT 6,L},
  // 0x76 => instruction!{r, mm => BIT 6,(HL)},
  // 0x77 => instruction!{r, mm => BIT 6,A},
  // 0x78 => instruction!{r, mm => BIT 7,B},
  // 0x79 => instruction!{r, mm => BIT 7,C},
  // 0x7A => instruction!{r, mm => BIT 7,D},
  // 0x7B => instruction!{r, mm => BIT 7,E},
  // 0x7C => instruction!{r, mm => BIT 7,H},
  // 0x7D => instruction!{r, mm => BIT 7,L},
  // 0x7E => instruction!{r, mm => BIT 7,(HL)},
  // 0x7F => instruction!{r, mm => BIT 7,A},

  // 0x80 => instruction!{r, mm => RES 0,B},
  // 0x81 => instruction!{r, mm => RES 0,C},
  // 0x82 => instruction!{r, mm => RES 0,D},
  // 0x83 => instruction!{r, mm => RES 0,E},
  // 0x84 => instruction!{r, mm => RES 0,H},
  // 0x85 => instruction!{r, mm => RES 0,L},
  // 0x86 => instruction!{r, mm => RES 0,(HL)},
  // 0x87 => instruction!{r, mm => RES 0,A},
  // 0x88 => instruction!{r, mm => RES 1,B},
  // 0x89 => instruction!{r, mm => RES 1,C},
  // 0x8A => instruction!{r, mm => RES 1,D},
  // 0x8B => instruction!{r, mm => RES 1,E},
  // 0x8C => instruction!{r, mm => RES 1,H},
  // 0x8D => instruction!{r, mm => RES 1,L},
  // 0x8E => instruction!{r, mm => RES 1,(HL)},
  // 0x8F => instruction!{r, mm => RES 1,A},

  // 0x90 => instruction!{r, mm => RES 2,B},
  // 0x91 => instruction!{r, mm => RES 2,C},
  // 0x92 => instruction!{r, mm => RES 2,D},
  // 0x93 => instruction!{r, mm => RES 2,E},
  // 0x94 => instruction!{r, mm => RES 2,H},
  // 0x95 => instruction!{r, mm => RES 2,L},
  // 0x96 => instruction!{r, mm => RES 2,(HL)},
  // 0x97 => instruction!{r, mm => RES 2,A},
  // 0x98 => instruction!{r, mm => RES 3,B},
  // 0x99 => instruction!{r, mm => RES 3,C},
  // 0x9A => instruction!{r, mm => RES 3,D},
  // 0x9B => instruction!{r, mm => RES 3,E},
  // 0x9C => instruction!{r, mm => RES 3,H},
  // 0x9D => instruction!{r, mm => RES 3,L},
  // 0x9E => instruction!{r, mm => RES 3,(HL)},
  // 0x9F => instruction!{r, mm => RES 3,A},

  // 0xA0 => instruction!{r, mm => RES 4,B},
  // 0xA1 => instruction!{r, mm => RES 4,C},
  // 0xA2 => instruction!{r, mm => RES 4,D},
  // 0xA3 => instruction!{r, mm => RES 4,E},
  // 0xA4 => instruction!{r, mm => RES 4,H},
  // 0xA5 => instruction!{r, mm => RES 4,L},
  // 0xA6 => instruction!{r, mm => RES 4,(HL)},
  // 0xA7 => instruction!{r, mm => RES 4,A},
  // 0xA8 => instruction!{r, mm => RES 5,B},
  // 0xA9 => instruction!{r, mm => RES 5,C},
  // 0xAA => instruction!{r, mm => RES 5,D},
  // 0xAB => instruction!{r, mm => RES 5,E},
  // 0xAC => instruction!{r, mm => RES 5,H},
  // 0xAD => instruction!{r, mm => RES 5,L},
  // 0xAE => instruction!{r, mm => RES 5,(HL)},
  // 0xAF => instruction!{r, mm => RES 5,A},

  // 0xB0 => instruction!{r, mm => RES 6,B},
  // 0xB1 => instruction!{r, mm => RES 6,C},
  // 0xB2 => instruction!{r, mm => RES 6,D},
  // 0xB3 => instruction!{r, mm => RES 6,E},
  // 0xB4 => instruction!{r, mm => RES 6,H},
  // 0xB5 => instruction!{r, mm => RES 6,L},
  // 0xB6 => instruction!{r, mm => RES 6,(HL)},
  // 0xB7 => instruction!{r, mm => RES 6,A},
  // 0xB8 => instruction!{r, mm => RES 7,B},
  // 0xB9 => instruction!{r, mm => RES 7,C},
  // 0xBA => instruction!{r, mm => RES 7,D},
  // 0xBB => instruction!{r, mm => RES 7,E},
  // 0xBC => instruction!{r, mm => RES 7,H},
  // 0xBD => instruction!{r, mm => RES 7,L},
  // 0xBE => instruction!{r, mm => RES 7,(HL)},
  // 0xBF => instruction!{r, mm => RES 7,A},

  // 0xC0 => instruction!{r, mm => SET 0,B},
  // 0xC1 => instruction!{r, mm => SET 0,C},
  // 0xC2 => instruction!{r, mm => SET 0,D},
  // 0xC3 => instruction!{r, mm => SET 0,E},
  // 0xC4 => instruction!{r, mm => SET 0,H},
  // 0xC5 => instruction!{r, mm => SET 0,L},
  // 0xC6 => instruction!{r, mm => SET 0,(HL)},
  // 0xC7 => instruction!{r, mm => SET 0,A},
  // 0xC8 => instruction!{r, mm => SET 1,B},
  // 0xC9 => instruction!{r, mm => SET 1,C},
  // 0xCA => instruction!{r, mm => SET 1,D},
  // 0xCB => instruction!{r, mm => SET 1,E},
  // 0xCC => instruction!{r, mm => SET 1,H},
  // 0xCD => instruction!{r, mm => SET 1,L},
  // 0xCE => instruction!{r, mm => SET 1,(HL)},
  // 0xCF => instruction!{r, mm => SET 1,A},

  // 0xD0 => instruction!{r, mm => SET 2,B},
  // 0xD1 => instruction!{r, mm => SET 2,C},
  // 0xD2 => instruction!{r, mm => SET 2,D},
  // 0xD3 => instruction!{r, mm => SET 2,E},
  // 0xD4 => instruction!{r, mm => SET 2,H},
  // 0xD5 => instruction!{r, mm => SET 2,L},
  // 0xD6 => instruction!{r, mm => SET 2,(HL)},
  // 0xD7 => instruction!{r, mm => SET 2,A},
  // 0xD8 => instruction!{r, mm => SET 3,B},
  // 0xD9 => instruction!{r, mm => SET 3,C},
  // 0xDA => instruction!{r, mm => SET 3,D},
  // 0xDB => instruction!{r, mm => SET 3,E},
  // 0xDC => instruction!{r, mm => SET 3,H},
  // 0xDD => instruction!{r, mm => SET 3,L},
  // 0xDE => instruction!{r, mm => SET 3,(HL)},
  // 0xDF => instruction!{r, mm => SET 3,A},

  // 0xE0 => instruction!{r, mm => SET 4,B},
  // 0xE1 => instruction!{r, mm => SET 4,C},
  // 0xE2 => instruction!{r, mm => SET 4,D},
  // 0xE3 => instruction!{r, mm => SET 4,E},
  // 0xE4 => instruction!{r, mm => SET 4,H},
  // 0xE5 => instruction!{r, mm => SET 4,L},
  // 0xE6 => instruction!{r, mm => SET 4,(HL)},
  // 0xE7 => instruction!{r, mm => SET 4,A},
  // 0xE8 => instruction!{r, mm => SET 5,B},
  // 0xE9 => instruction!{r, mm => SET 5,C},
  // 0xEA => instruction!{r, mm => SET 5,D},
  // 0xEB => instruction!{r, mm => SET 5,E},
  // 0xEC => instruction!{r, mm => SET 5,H},
  // 0xED => instruction!{r, mm => SET 5,L},
  // 0xEE => instruction!{r, mm => SET 5,(HL)},
  // 0xEF => instruction!{r, mm => SET 5,A},

  // 0xF0 => instruction!{r, mm => SET 6,B},
  // 0xF1 => instruction!{r, mm => SET 6,C},
  // 0xF2 => instruction!{r, mm => SET 6,D},
  // 0xF3 => instruction!{r, mm => SET 6,E},
  // 0xF4 => instruction!{r, mm => SET 6,H},
  // 0xF5 => instruction!{r, mm => SET 6,L},
  // 0xF6 => instruction!{r, mm => SET 6,(HL)},
  // 0xF7 => instruction!{r, mm => SET 6,A},
  // 0xF8 => instruction!{r, mm => SET 7,B},
  // 0xF9 => instruction!{r, mm => SET 7,C},
  // 0xFA => instruction!{r, mm => SET 7,D},
  // 0xFB => instruction!{r, mm => SET 7,E},
  // 0xFC => instruction!{r, mm => SET 7,H},
  // 0xFD => instruction!{r, mm => SET 7,L},
  // 0xFE => instruction!{r, mm => SET 7,(HL)},
  // 0xFF => instruction!{r, mm => SET 7,A},
  //
  //     _ => panic!("Unsupported instruction 0xCB{:02X}", opcode),
  //   }
  // }

  fn stack_push_u16(r: &mut Registers, mm: &mut MemoryMap, value: u16) {
    r.sp -= size_of_val(&value) as u16;
    mm.write(r.sp, value);
  }

  fn stack_pop_u16(r: &mut Registers, mm: &mut MemoryMap) -> u16 {
    let popped: u16 = mm.read(r.sp);
    r.sp += size_of_val(&popped) as u16;
    popped
  }
}

pub trait Eat<T> {
  fn eat(r: &mut Registers, mm: &MemoryMap) -> T;
}

impl Eat<u8> for Processor {
  fn eat(r: &mut Registers, mm: &MemoryMap) -> u8 {
    let ret: u8 = mm.read(r.pc);
    r.pc += size_of_val(&ret) as u16;
    ret
  }
}

impl Eat<u16> for Processor {
  fn eat(r: &mut Registers, mm: &MemoryMap) -> u16 {
    let ret: u16 = mm.read(r.pc);
    r.pc += size_of_val(&ret) as u16;
    ret
  }
}

impl Eat<i8> for Processor {
  fn eat(r: &mut Registers, mm: &MemoryMap) -> i8 {
    let ret: i8 = mm.read(r.pc);
    r.pc += size_of_val(&ret) as u16 - 1;
    ret
  }
}
