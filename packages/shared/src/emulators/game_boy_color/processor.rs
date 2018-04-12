use std::mem::size_of_val;

use super::registers::{Flag, Register, Registers};
use super::MemoryMap;

macro_rules! factory {
  {ADC $o1_value:ident, $o2_value:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      let register = registers.get_u8(Register::$o2_value);
      let carry = registers.get_flag_u8(Flag::C);
      4
    }
  );
  {ADC $o1:ident, ($o2_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {ADD $o1:ident, $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {ADD $o1:ident, ($o2_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {AND $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {AND ($o1_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {BIT $o1:expr, $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {BIT $o1:expr, ($o2_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {CALL $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {CALL $o1:ident, $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {CCF} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {CP $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {CP ($o1:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {CPL} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {DAA} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {DEC $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {DEC ($o1:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {DI} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {EI} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {HALT} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {INC $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {INC ($o1:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {JP $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {JP ($o1:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {JP $o1:ident, $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {JR $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {JR $o1:ident, $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {LD $o1:ident, $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {LD ($o1_ref:ident), $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {LD $o1:ident, ($o2_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {LD $o1:ident, SP + $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {LDD ($o1_ref:ident), $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {LDD $o1:ident, ($o2_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {LDH $o1_ref:ident, ($o2:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {LDH ($o1_ref:ident), $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {LDI ($o1_ref:ident), $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {LDI $o1:ident, ($o2_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {NOP} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {OR $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {OR ($o1:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {PUSH $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {POP $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RES $o1:expr, $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {RES $o1:expr, ($o2_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RET} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {RET $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RETI} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RL $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {RL ($o1_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RLA} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RLC $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {RLC ($o1_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RLCA} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RR $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {RR ($o1_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RRA} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RST $o1:expr} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RRC $o1:expr} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {RRCA} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {SCF} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {SET $o1:expr, $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {SET $o1:expr, ($o2_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {SLA $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {SLA ($o1_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {SRA $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {SRA ($o1_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {SRL $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {SRL ($o1_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {STOP $o1:expr} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {SBC $o1:ident, $o2:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {SBC $o1:ident, ($o2_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {SUB $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {SUB ($o1_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {SWAP $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {SWAP ($o1_ref:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );

  {XOR $o1:ident} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
  {XOR ($o1:ident)} => (
    |registers: &mut Registers, memory_map: &mut MemoryMap| -> u8 {
      12
    }
  );
}
pub struct Processor {
  registers: &mut Registers,
  memory_map: &mut MemoryMap,
}

impl Processor {
  pub fn new(registers: &mut Registers, memory_map: &mut MemoryMap) -> Self {
    Processor {
      registers,
      memory_map,
    }
  }

  pub fn process_instruction(&self) -> u8 {
    let byte = self.memory_map.read_u8(self.registers.pc as usize);
    print!("[0x{:04X}] 0x{:02X}:", self.registers.pc, byte);
    match byte {
      0x00 => factory!{NOP},
      0x01 => factory!{LD (BC),d16},
      0x02 => factory!{LD (BC),A},
      0x03 => factory!{INC BC},
      0x04 => factory!{INC B},
      0x05 => factory!{DEC B},
      0x06 => factory!{LD B,d8},
      0x07 => factory!{RLCA},
      0x08 => factory!{LD (a16),SP},
      0x09 => factory!{ADD HL,BC},
      0x0A => factory!{LD A,(BC)},
      0x0B => factory!{DEC BC},
      0x0C => factory!{INC C},
      0x0D => factory!{DEC C},
      0x0E => factory!{LD C,d8},
      0x0F => factory!{RRCA},

      0x10 => factory!{STOP 0},
      0x11 => factory!{LD DE,d16},
      0x12 => factory!{LD (DE),A},
      0x13 => factory!{INC DE},
      0x14 => factory!{INC D},
      0x15 => factory!{DEC D},
      0x16 => factory!{LD D,d8},
      0x17 => factory!{RLA},
      0x18 => factory!{JR r8},
      0x19 => factory!{ADD HL,DE},
      0x1A => factory!{LD A,(DE)},
      0x1B => factory!{DEC DE},
      0x1C => factory!{INC E},
      0x1D => factory!{DEC E},
      0x1E => factory!{LD E,d8},
      0x1F => factory!{RRA},

      0x20 => factory!{JR NZ,r8},
      0x21 => factory!{LD HL,d16},
      0x22 => factory!{LDI (HL),A},
      0x23 => factory!{INC HL},
      0x24 => factory!{INC H},
      0x25 => factory!{DEC H},
      0x26 => factory!{LD H,d8},
      0x27 => factory!{DAA},
      0x28 => factory!{JR Z,r8},
      0x29 => factory!{ADD HL,HL},
      0x2A => factory!{LDI A, (HL)},
      0x2B => factory!{DEC HL},
      0x2C => factory!{INC L},
      0x2D => factory!{DEC L},
      0x2E => factory!{LD L,d8},
      0x2F => factory!{CPL},

      0x30 => factory!{JR NC,r8},
      0x31 => factory!{LD SP,d16},
      0x32 => factory!{LDD (HL),A},
      0x33 => factory!{INC SP},
      0x34 => factory!{INC (HL)},
      0x35 => factory!{DEC (HL)},
      0x36 => factory!{LD (HL),d8},
      0x37 => factory!{SCF},
      0x38 => factory!{JR C,r8},
      0x39 => factory!{ADD HL,SP},
      0x3A => factory!{LDD A,(HL)},
      0x3B => factory!{DEC SP},
      0x3C => factory!{INC A},
      0x3D => factory!{DEC A},
      0x3E => factory!{LD A,d8},
      0x3F => factory!{CCF},

      0x40 => factory!{LD B,B},
      0x41 => factory!{LD B,C},
      0x42 => factory!{LD B,D},
      0x43 => factory!{LD B,E},
      0x44 => factory!{LD B,H},
      0x45 => factory!{LD B,L},
      0x46 => factory!{LD B,(HL)},
      0x47 => factory!{LD B,A},
      0x48 => factory!{LD C,B},
      0x49 => factory!{LD C,C},
      0x4A => factory!{LD C,D},
      0x4B => factory!{LD C,E},
      0x4C => factory!{LD C,H},
      0x4D => factory!{LD C,L},
      0x4E => factory!{LD C,(HL)},
      0x4F => factory!{LD C,A},

      0x50 => factory!{LD D,B},
      0x51 => factory!{LD D,C},
      0x52 => factory!{LD D,D},
      0x53 => factory!{LD D,E},
      0x54 => factory!{LD D,H},
      0x55 => factory!{LD D,L},
      0x56 => factory!{LD D,(HL)},
      0x57 => factory!{LD D,A},
      0x58 => factory!{LD E,B},
      0x59 => factory!{LD E,C},
      0x5A => factory!{LD E,D},
      0x5B => factory!{LD E,E},
      0x5C => factory!{LD E,H},
      0x5D => factory!{LD E,L},
      0x5E => factory!{LD E,(HL)},
      0x5F => factory!{LD E,A},

      0x60 => factory!{LD H,B},
      0x61 => factory!{LD H,C},
      0x62 => factory!{LD H,D},
      0x63 => factory!{LD H,E},
      0x64 => factory!{LD H,H},
      0x65 => factory!{LD H,L},
      0x66 => factory!{LD H,(HL)},
      0x67 => factory!{LD H,A},
      0x68 => factory!{LD L,B},
      0x69 => factory!{LD L,C},
      0x6A => factory!{LD L,D},
      0x6B => factory!{LD L,E},
      0x6C => factory!{LD L,H},
      0x6D => factory!{LD L,L},
      0x6E => factory!{LD L,(HL)},
      0x6F => factory!{LD L,A},

      0x70 => factory!{LD (HL),B},
      0x71 => factory!{LD (HL),C},
      0x72 => factory!{LD (HL),D},
      0x73 => factory!{LD (HL),E},
      0x74 => factory!{LD (HL),H},
      0x75 => factory!{LD (HL),L},
      0x76 => factory!{HALT},
      0x77 => factory!{LD (HL),A},
      0x78 => factory!{LD A,B},
      0x79 => factory!{LD A,C},
      0x7A => factory!{LD A,D},
      0x7B => factory!{LD A,E},
      0x7C => factory!{LD A,H},
      0x7D => factory!{LD A,L},
      0x7E => factory!{LD A,(HL)},
      0x7F => factory!{LD A,A},

      0x80 => factory!{ADD A,B},
      0x81 => factory!{ADD A,C},
      0x82 => factory!{ADD A,D},
      0x83 => factory!{ADD A,E},
      0x84 => factory!{ADD A,H},
      0x85 => factory!{ADD A,L},
      0x86 => factory!{ADD A,(HL)},
      0x87 => factory!{ADD A,A},
      0x88 => factory!{ADC A,B},
      0x89 => factory!{ADC A,C},
      0x8A => factory!{ADC A,D},
      0x8B => factory!{ADC A,E},
      0x8C => factory!{ADC A,H},
      0x8D => factory!{ADC A,L},
      0x8E => factory!{ADC A,(HL)},
      0x8F => factory!{ADC A,A},

      0x90 => factory!{SUB B},
      0x91 => factory!{SUB C},
      0x92 => factory!{SUB D},
      0x93 => factory!{SUB E},
      0x94 => factory!{SUB H},
      0x95 => factory!{SUB L},
      0x96 => factory!{SUB (HL)},
      0x97 => factory!{SUB A},
      0x98 => factory!{SBC A,B},
      0x99 => factory!{SBC A,C},
      0x9A => factory!{SBC A,D},
      0x9B => factory!{SBC A,E},
      0x9C => factory!{SBC A,H},
      0x9D => factory!{SBC A,L},
      0x9E => factory!{SBC A,(HL)},
      0x9F => factory!{SBC A,A},

      0xA0 => factory!{AND B},
      0xA1 => factory!{AND C},
      0xA2 => factory!{AND D},
      0xA3 => factory!{AND E},
      0xA4 => factory!{AND H},
      0xA5 => factory!{AND L},
      0xA6 => factory!{AND (HL)},
      0xA7 => factory!{AND A},
      0xA8 => factory!{XOR B},
      0xA9 => factory!{XOR C},
      0xAA => factory!{XOR D},
      0xAB => factory!{XOR E},
      0xAC => factory!{XOR H},
      0xAD => factory!{XOR L},
      0xAE => factory!{XOR (HL)},
      0xAF => factory!{XOR A},

      0xB0 => factory!{OR B},
      0xB1 => factory!{OR C},
      0xB2 => factory!{OR D},
      0xB3 => factory!{OR E},
      0xB4 => factory!{OR H},
      0xB5 => factory!{OR L},
      0xB6 => factory!{OR (HL)},
      0xB7 => factory!{OR A},
      0xB8 => factory!{CP B},
      0xB9 => factory!{CP C},
      0xBA => factory!{CP D},
      0xBB => factory!{CP E},
      0xBC => factory!{CP H},
      0xBD => factory!{CP L},
      0xBE => factory!{CP (HL)},
      0xBF => factory!{CP A},

      0xC0 => factory!{RET NZ},
      0xC1 => factory!{POP BC},
      0xC2 => factory!{JP NZ,a16},
      0xC3 => factory!{JP a16},
      0xC4 => factory!{CALL NZ,a16},
      0xC5 => factory!{PUSH BC},
      0xC6 => factory!{ADD A,d8},
      0xC7 => factory!{RST 0x00},
      0xC8 => factory!{RET Z},
      0xC9 => factory!{RET},
      0xCA => factory!{JP Z,a16},
      0xCB => self.process_instruction_cb(),
      0xCC => factory!{CALL Z,a16},
      0xCD => factory!{CALL a16},
      0xCE => factory!{ADC A,d8},
      0xCF => factory!{RST 0x08},

      0xD0 => factory!{RET NC},
      0xD1 => factory!{POP DE},
      0xD2 => factory!{JP NC,a16},
      /* 0xD3 */
      0xD4 => factory!{CALL NC,a16},
      0xD5 => factory!{PUSH DE},
      0xD6 => factory!{SUB d8},
      0xD7 => factory!{RST 0x10},
      0xD8 => factory!{RET C},
      0xD9 => factory!{RETI},
      0xDA => factory!{JP C,a16},
      /* 0xDB */
      0xDC => factory!{CALL C,a16},
      /* 0xDD */
      0xDE => factory!{SBC A,d8},
      0xDF => factory!{RST 0x18},

      0xE0 => factory!{LDH (a8),A},
      0xE1 => factory!{POP HL},
      0xE2 => factory!{LD (C),A},
      /* 0xE3 */
      /* 0xE4 */
      0xE5 => factory!{PUSH HL},
      0xE6 => factory!{AND d8},
      0xE7 => factory!{RST 0x20},
      0xE8 => factory!{ADD SP,r8},
      0xE9 => factory!{JP (HL)},
      0xEA => factory!{LD (a16),A},
      /* 0xEB */
      /* 0xEC */
      /* 0xED */
      0xEE => factory!{XOR d8},
      0xEF => factory!{RST 0x28},

      0xF0 => factory!{LDH A,(a8)},
      0xF1 => factory!{POP AF},
      0xF2 => factory!{LD A,(C)},
      0xF3 => factory!{DI},
      /* 0xF4 */
      0xF5 => factory!{PUSH AF},
      0xF6 => factory!{OR d8},
      0xF7 => factory!{RST 0x30},
      0xF8 => factory!{LD HL,SP+r8},
      0xF9 => factory!{LD SP,HL},
      0xFA => factory!{LD A,(a16)},
      0xFB => factory!{EI},
      /* 0xFC */
      /* 0xFD */
      0xFE => factory!{CP d8},
      0xFF => factory!{RST 0x38},
      _ => panic!("Unsupported instruction 0x{:02X}", byte),
    }
  }

  fn process_instruction_cb(&self) -> u8 {
    let byte = self.memory_map.read_u8(self.registers.pc as usize + 1);
    print!("0xCB{:02X}:", byte);
    match byte {
      0x00 => factory!{RLC B},
      0x01 => factory!{RLC C},
      0x02 => factory!{RLC D},
      0x03 => factory!{RLC E},
      0x04 => factory!{RLC H},
      0x05 => factory!{RLC L},
      0x06 => factory!{RLC (HL)},
      0x07 => factory!{RLC A},
      0x08 => factory!{RRC B},
      0x09 => factory!{RRC C},
      0x0A => factory!{RRC D},
      0x0B => factory!{RRC E},
      0x0C => factory!{RRC H},
      0x0D => factory!{RRC L},
      0x0E => factory!{RRC (HL)},
      0x0F => factory!{RRC A},

      0x10 => factory!{RL B},
      0x11 => factory!{RL C},
      0x12 => factory!{RL D},
      0x13 => factory!{RL E},
      0x14 => factory!{RL H},
      0x15 => factory!{RL L},
      0x16 => factory!{RL (HL)},
      0x17 => factory!{RL A},
      0x18 => factory!{RR B},
      0x19 => factory!{RR C},
      0x1A => factory!{RR D},
      0x1B => factory!{RR E},
      0x1C => factory!{RR H},
      0x1D => factory!{RR L},
      0x1E => factory!{RR (HL)},
      0x1F => factory!{RR A},

      0x20 => factory!{SLA B},
      0x21 => factory!{SLA C},
      0x22 => factory!{SLA D},
      0x23 => factory!{SLA E},
      0x24 => factory!{SLA H},
      0x25 => factory!{SLA L},
      0x26 => factory!{SLA (HL)},
      0x27 => factory!{SLA A},
      0x28 => factory!{SRA B},
      0x29 => factory!{SRA C},
      0x2A => factory!{SRA D},
      0x2B => factory!{SRA E},
      0x2C => factory!{SRA H},
      0x2D => factory!{SRA L},
      0x2E => factory!{SRA (HL)},
      0x2F => factory!{SRA A},

      0x30 => factory!{SWAP B},
      0x31 => factory!{SWAP C},
      0x32 => factory!{SWAP D},
      0x33 => factory!{SWAP E},
      0x34 => factory!{SWAP H},
      0x35 => factory!{SWAP L},
      0x36 => factory!{SWAP (HL)},
      0x37 => factory!{SWAP A},
      0x38 => factory!{SRL B},
      0x39 => factory!{SRL C},
      0x3A => factory!{SRL D},
      0x3B => factory!{SRL E},
      0x3C => factory!{SRL H},
      0x3D => factory!{SRL L},
      0x3E => factory!{SRL (HL)},
      0x3F => factory!{SRL A},

      0x40 => factory!{BIT 0,B},
      0x41 => factory!{BIT 0,C},
      0x42 => factory!{BIT 0,D},
      0x43 => factory!{BIT 0,E},
      0x44 => factory!{BIT 0,H},
      0x45 => factory!{BIT 0,L},
      0x46 => factory!{BIT 0,(HL)},
      0x47 => factory!{BIT 0,A},
      0x48 => factory!{BIT 1,B},
      0x49 => factory!{BIT 1,C},
      0x4A => factory!{BIT 1,D},
      0x4B => factory!{BIT 1,E},
      0x4C => factory!{BIT 1,H},
      0x4D => factory!{BIT 1,L},
      0x4E => factory!{BIT 1,(HL)},
      0x4F => factory!{BIT 1,A},

      0x50 => factory!{BIT 2,B},
      0x51 => factory!{BIT 2,C},
      0x52 => factory!{BIT 2,D},
      0x53 => factory!{BIT 2,E},
      0x54 => factory!{BIT 2,H},
      0x55 => factory!{BIT 2,L},
      0x56 => factory!{BIT 2,(HL)},
      0x57 => factory!{BIT 2,A},
      0x58 => factory!{BIT 3,B},
      0x59 => factory!{BIT 3,C},
      0x5A => factory!{BIT 3,D},
      0x5B => factory!{BIT 3,E},
      0x5C => factory!{BIT 3,H},
      0x5D => factory!{BIT 3,L},
      0x5E => factory!{BIT 3,(HL)},
      0x5F => factory!{BIT 3,A},

      0x60 => factory!{BIT 4,B},
      0x61 => factory!{BIT 4,C},
      0x62 => factory!{BIT 4,D},
      0x63 => factory!{BIT 4,E},
      0x64 => factory!{BIT 4,H},
      0x65 => factory!{BIT 4,L},
      0x66 => factory!{BIT 4,(HL)},
      0x67 => factory!{BIT 4,A},
      0x68 => factory!{BIT 5,B},
      0x69 => factory!{BIT 5,C},
      0x6A => factory!{BIT 5,D},
      0x6B => factory!{BIT 5,E},
      0x6C => factory!{BIT 5,H},
      0x6D => factory!{BIT 5,L},
      0x6E => factory!{BIT 5,(HL)},
      0x6F => factory!{BIT 5,A},

      0x70 => factory!{BIT 6,B},
      0x71 => factory!{BIT 6,C},
      0x72 => factory!{BIT 6,D},
      0x73 => factory!{BIT 6,E},
      0x74 => factory!{BIT 6,H},
      0x75 => factory!{BIT 6,L},
      0x76 => factory!{BIT 6,(HL)},
      0x77 => factory!{BIT 6,A},
      0x78 => factory!{BIT 7,B},
      0x79 => factory!{BIT 7,C},
      0x7A => factory!{BIT 7,D},
      0x7B => factory!{BIT 7,E},
      0x7C => factory!{BIT 7,H},
      0x7D => factory!{BIT 7,L},
      0x7E => factory!{BIT 7,(HL)},
      0x7F => factory!{BIT 7,A},

      0x80 => factory!{RES 0,B},
      0x81 => factory!{RES 0,C},
      0x82 => factory!{RES 0,D},
      0x83 => factory!{RES 0,E},
      0x84 => factory!{RES 0,H},
      0x85 => factory!{RES 0,L},
      0x86 => factory!{RES 0,(HL)},
      0x87 => factory!{RES 0,A},
      0x88 => factory!{RES 1,B},
      0x89 => factory!{RES 1,C},
      0x8A => factory!{RES 1,D},
      0x8B => factory!{RES 1,E},
      0x8C => factory!{RES 1,H},
      0x8D => factory!{RES 1,L},
      0x8E => factory!{RES 1,(HL)},
      0x8F => factory!{RES 1,A},

      0x90 => factory!{RES 2,B},
      0x91 => factory!{RES 2,C},
      0x92 => factory!{RES 2,D},
      0x93 => factory!{RES 2,E},
      0x94 => factory!{RES 2,H},
      0x95 => factory!{RES 2,L},
      0x96 => factory!{RES 2,(HL)},
      0x97 => factory!{RES 2,A},
      0x98 => factory!{RES 3,B},
      0x99 => factory!{RES 3,C},
      0x9A => factory!{RES 3,D},
      0x9B => factory!{RES 3,E},
      0x9C => factory!{RES 3,H},
      0x9D => factory!{RES 3,L},
      0x9E => factory!{RES 3,(HL)},
      0x9F => factory!{RES 3,A},

      0xA0 => factory!{RES 4,B},
      0xA1 => factory!{RES 4,C},
      0xA2 => factory!{RES 4,D},
      0xA3 => factory!{RES 4,E},
      0xA4 => factory!{RES 4,H},
      0xA5 => factory!{RES 4,L},
      0xA6 => factory!{RES 4,(HL)},
      0xA7 => factory!{RES 4,A},
      0xA8 => factory!{RES 5,B},
      0xA9 => factory!{RES 5,C},
      0xAA => factory!{RES 5,D},
      0xAB => factory!{RES 5,E},
      0xAC => factory!{RES 5,H},
      0xAD => factory!{RES 5,L},
      0xAE => factory!{RES 5,(HL)},
      0xAF => factory!{RES 5,A},

      0xB0 => factory!{RES 6,B},
      0xB1 => factory!{RES 6,C},
      0xB2 => factory!{RES 6,D},
      0xB3 => factory!{RES 6,E},
      0xB4 => factory!{RES 6,H},
      0xB5 => factory!{RES 6,L},
      0xB6 => factory!{RES 6,(HL)},
      0xB7 => factory!{RES 6,A},
      0xB8 => factory!{RES 7,B},
      0xB9 => factory!{RES 7,C},
      0xBA => factory!{RES 7,D},
      0xBB => factory!{RES 7,E},
      0xBC => factory!{RES 7,H},
      0xBD => factory!{RES 7,L},
      0xBE => factory!{RES 7,(HL)},
      0xBF => factory!{RES 7,A},

      0xC0 => factory!{SET 0,B},
      0xC1 => factory!{SET 0,C},
      0xC2 => factory!{SET 0,D},
      0xC3 => factory!{SET 0,E},
      0xC4 => factory!{SET 0,H},
      0xC5 => factory!{SET 0,L},
      0xC6 => factory!{SET 0,(HL)},
      0xC7 => factory!{SET 0,A},
      0xC8 => factory!{SET 1,B},
      0xC9 => factory!{SET 1,C},
      0xCA => factory!{SET 1,D},
      0xCB => factory!{SET 1,E},
      0xCC => factory!{SET 1,H},
      0xCD => factory!{SET 1,L},
      0xCE => factory!{SET 1,(HL)},
      0xCF => factory!{SET 1,A},

      0xD0 => factory!{SET 2,B},
      0xD1 => factory!{SET 2,C},
      0xD2 => factory!{SET 2,D},
      0xD3 => factory!{SET 2,E},
      0xD4 => factory!{SET 2,H},
      0xD5 => factory!{SET 2,L},
      0xD6 => factory!{SET 2,(HL)},
      0xD7 => factory!{SET 2,A},
      0xD8 => factory!{SET 3,B},
      0xD9 => factory!{SET 3,C},
      0xDA => factory!{SET 3,D},
      0xDB => factory!{SET 3,E},
      0xDC => factory!{SET 3,H},
      0xDD => factory!{SET 3,L},
      0xDE => factory!{SET 3,(HL)},
      0xDF => factory!{SET 3,A},

      0xE0 => factory!{SET 4,B},
      0xE1 => factory!{SET 4,C},
      0xE2 => factory!{SET 4,D},
      0xE3 => factory!{SET 4,E},
      0xE4 => factory!{SET 4,H},
      0xE5 => factory!{SET 4,L},
      0xE6 => factory!{SET 4,(HL)},
      0xE7 => factory!{SET 4,A},
      0xE8 => factory!{SET 5,B},
      0xE9 => factory!{SET 5,C},
      0xEA => factory!{SET 5,D},
      0xEB => factory!{SET 5,E},
      0xEC => factory!{SET 5,H},
      0xED => factory!{SET 5,L},
      0xEE => factory!{SET 5,(HL)},
      0xEF => factory!{SET 5,A},

      0xF0 => factory!{SET 6,B},
      0xF1 => factory!{SET 6,C},
      0xF2 => factory!{SET 6,D},
      0xF3 => factory!{SET 6,E},
      0xF4 => factory!{SET 6,H},
      0xF5 => factory!{SET 6,L},
      0xF6 => factory!{SET 6,(HL)},
      0xF7 => factory!{SET 6,A},
      0xF8 => factory!{SET 7,B},
      0xF9 => factory!{SET 7,C},
      0xFA => factory!{SET 7,D},
      0xFB => factory!{SET 7,E},
      0xFC => factory!{SET 7,H},
      0xFD => factory!{SET 7,L},
      0xFE => factory!{SET 7,(HL)},
      0xFF => factory!{SET 7,A},

      _ => panic!("Unsupported instruction 0xCB{:02X}", byte),
    }
  }

  fn stack_push_u16(&mut self, value: u16) {
    self.registers.sp -= size_of_val(&value) as u16;
    self.memory_map.write_u16(self.registers.sp as usize, value);
  }

  fn stack_pop_u16(&mut self) -> u16 {
    let value = self.memory_map.read_u16(self.registers.sp as usize);
    self.registers.sp += size_of_val(&value) as u16;
    value
  }
}
