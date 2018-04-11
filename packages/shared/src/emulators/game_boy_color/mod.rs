#[macro_use]
mod instructions;
mod memory_map;
mod registers;

use std::fs::File;
use std::io::prelude::*;
use std::mem::size_of_val;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use self::memory_map::MemoryMap;
use self::registers::{Flag, Registers};
// use screen::ColorDepth32Bit;

pub struct GameBoyColor {
  bios: &'static [u8],
  cartridge: Vec<u8>,

  memory_map: MemoryMap,
  registers: Registers,
  cycle_duration_nanos: u64,
  // screen: ColorDepth32Bit,
}

impl GameBoyColor {
  pub fn new() -> Self {
    GameBoyColor {
      bios: include_bytes!("./assets/bios.gbc"),
      cartridge: Vec::new(),

      memory_map: MemoryMap::new(),
      registers: Registers::new(),
      cycle_duration_nanos: (1_000_000_000. / (4.194 * 1_000_000.)) as u64,
      // screen: ColorDepth32Bit::new(160, 144),
    }
  }

  pub fn load(&mut self, file_path: &str) {
    File::open(file_path)
      .expect("Cannot open cartridge")
      .read_to_end(&mut self.cartridge)
      .expect("Cannot read cartridge");
  }

  pub fn boot(&mut self) {
    self.memory_map.write(0x0000, &self.bios);
    loop {
      self.registers.dump();
      let time = Instant::now();
      let cycles = self.execute_instruction();
      let elapsed = time.elapsed();
      let elapsed_nanos = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
      let realistic_elapsed_nanos = cycles * self.cycle_duration_nanos;
      if elapsed_nanos < realistic_elapsed_nanos {
        sleep(Duration::from_nanos(
          realistic_elapsed_nanos - elapsed_nanos,
        ))
      }
    }
  }

  fn execute_instruction(&mut self) -> u8 {
    let instruction = self.memory_map.read_u8(self.registers.pc as usize);
    print!("[0x{:04X}] 0x{:02X}:", self.registers.pc, instruction);
    match instruction {
      0x00 => instruction!{NOP},
      0x01 => instruction!{LD (BC),d16},
      0x02 => instruction!{LD (BC),A},
      0x03 => instruction!{INC BC},
      0x04 => instruction!{INC B},
      0x05 => instruction!{DEC B},
      0x06 => instruction!{LD B,d8},
      0x07 => instruction!{RLCA},
      0x08 => instruction!{LD (a16),SP},
      0x09 => instruction!{ADD HL,BC},
      0x0A => instruction!{LD A,(BC)},
      0x0B => instruction!{DEC BC},
      0x0C => instruction!{INC C},
      0x0D => instruction!{DEC C},
      0x0E => instruction!{LD C,d8},
      0x0F => instruction!{RRCA},

      0x10 => instruction!{STOP 0},
      0x11 => instruction!{LD DE,d16},
      0x12 => instruction!{LD (DE),A},
      0x13 => instruction!{INC DE},
      0x14 => instruction!{INC D},
      0x15 => instruction!{DEC D},
      0x16 => instruction!{LD D,d8},
      0x17 => instruction!{RLA},
      0x18 => instruction!{JR r8},
      0x19 => instruction!{ADD HL,DE},
      0x1A => instruction!{LD A,(DE)},
      0x1B => instruction!{DEC DE},
      0x1C => instruction!{INC E},
      0x1D => instruction!{DEC E},
      0x1E => instruction!{LD E,d8},
      0x1F => instruction!{RRA},

      0x20 => instruction!{JR NZ,r8},
      0x21 => instruction!{LD HL,d16},
      0x22 => instruction!{LDI (HL),A},
      0x23 => instruction!{INC HL},
      0x24 => instruction!{INC H},
      0x25 => instruction!{DEC H},
      0x26 => instruction!{LD H,d8},
      0x27 => instruction!{DAA},
      0x28 => instruction!{JR Z,r8},
      0x29 => instruction!{ADD HL,HL},
      0x2A => instruction!{LDI A, (HL)},
      0x2B => instruction!{DEC HL},
      0x2C => instruction!{INC L},
      0x2D => instruction!{DEC L},
      0x2E => instruction!{LD L,d8},
      0x2F => instruction!{CPL},

      0x30 => instruction!{JR NC,r8},
      0x31 => instruction!{LD SP,d16},
      0x32 => instruction!{LDD (HL),A},
      0x33 => instruction!{INC SP},
      0x34 => instruction!{INC (HL)},
      0x35 => instruction!{DEC (HL)},
      0x36 => instruction!{LD (HL),d8},
      0x37 => instruction!{SCF},
      0x38 => instruction!{JR C,r8},
      0x39 => instruction!{ADD HL,SP},
      0x3A => instruction!{LDD A,(HL)},
      0x3B => instruction!{DEC SP},
      0x3C => instruction!{INC A},
      0x3D => instruction!{DEC A},
      0x3E => instruction!{LD A,d8},
      0x3F => instruction!{CCF},

      0x40 => instruction!{LD B,B},
      0x41 => instruction!{LD B,C},
      0x42 => instruction!{LD B,D},
      0x43 => instruction!{LD B,E},
      0x44 => instruction!{LD B,H},
      0x45 => instruction!{LD B,L},
      0x46 => instruction!{LD B,(HL)},
      0x47 => instruction!{LD B,A},
      0x48 => instruction!{LD C,B},
      0x49 => instruction!{LD C,C},
      0x4A => instruction!{LD C,D},
      0x4B => instruction!{LD C,E},
      0x4C => instruction!{LD C,H},
      0x4D => instruction!{LD C,L},
      0x4E => instruction!{LD C,(HL)},
      0x4F => instruction!{LD C,A},

      0x50 => instruction!{LD D,B},
      0x51 => instruction!{LD D,C},
      0x52 => instruction!{LD D,D},
      0x53 => instruction!{LD D,E},
      0x54 => instruction!{LD D,H},
      0x55 => instruction!{LD D,L},
      0x56 => instruction!{LD D,(HL)},
      0x57 => instruction!{LD D,A},
      0x58 => instruction!{LD E,B},
      0x59 => instruction!{LD E,C},
      0x5A => instruction!{LD E,D},
      0x5B => instruction!{LD E,E},
      0x5C => instruction!{LD E,H},
      0x5D => instruction!{LD E,L},
      0x5E => instruction!{LD E,(HL)},
      0x5F => instruction!{LD E,A},

      0x60 => instruction!{LD H,B},
      0x61 => instruction!{LD H,C},
      0x62 => instruction!{LD H,D},
      0x63 => instruction!{LD H,E},
      0x64 => instruction!{LD H,H},
      0x65 => instruction!{LD H,L},
      0x66 => instruction!{LD H,(HL)},
      0x67 => instruction!{LD H,A},
      0x68 => instruction!{LD L,B},
      0x69 => instruction!{LD L,C},
      0x6A => instruction!{LD L,D},
      0x6B => instruction!{LD L,E},
      0x6C => instruction!{LD L,H},
      0x6D => instruction!{LD L,L},
      0x6E => instruction!{LD L,(HL)},
      0x6F => instruction!{LD L,A},

      0x70 => instruction!{LD (HL),B},
      0x71 => instruction!{LD (HL),C},
      0x72 => instruction!{LD (HL),D},
      0x73 => instruction!{LD (HL),E},
      0x74 => instruction!{LD (HL),H},
      0x75 => instruction!{LD (HL),L},
      0x76 => instruction!{HALT},
      0x77 => instruction!{LD (HL),A},
      0x78 => instruction!{LD A,B},
      0x79 => instruction!{LD A,C},
      0x7A => instruction!{LD A,D},
      0x7B => instruction!{LD A,E},
      0x7C => instruction!{LD A,H},
      0x7D => instruction!{LD A,L},
      0x7E => instruction!{LD A,(HL)},
      0x7F => instruction!{LD A,A},

      0x80 => instruction!{ADD A,B},
      0x81 => instruction!{ADD A,C},
      0x82 => instruction!{ADD A,D},
      0x83 => instruction!{ADD A,E},
      0x84 => instruction!{ADD A,H},
      0x85 => instruction!{ADD A,L},
      0x86 => instruction!{ADD A,(HL)},
      0x87 => instruction!{ADD A,A},
      0x88 => instruction!{ADC A,B},
      0x89 => instruction!{ADC A,C},
      0x8A => instruction!{ADC A,D},
      0x8B => instruction!{ADC A,E},
      0x8C => instruction!{ADC A,H},
      0x8D => instruction!{ADC A,L},
      0x8E => instruction!{ADC A,(HL)},
      0x8F => instruction!{ADC A,A},

      0x90 => instruction!{SUB B},
      0x91 => instruction!{SUB C},
      0x92 => instruction!{SUB D},
      0x93 => instruction!{SUB E},
      0x94 => instruction!{SUB H},
      0x95 => instruction!{SUB L},
      0x96 => instruction!{SUB (HL)},
      0x97 => instruction!{SUB A},
      0x98 => instruction!{SBC A,B},
      0x99 => instruction!{SBC A,C},
      0x9A => instruction!{SBC A,D},
      0x9B => instruction!{SBC A,E},
      0x9C => instruction!{SBC A,H},
      0x9D => instruction!{SBC A,L},
      0x9E => instruction!{SBC A,(HL)},
      0x9F => instruction!{SBC A,A},

      // above is OK

      0xA0 => instruction!{AND (B),
      0xA1 => instruction!{AND (C),
      0xA2 => instruction!{AND (D),
      0xA3 => instruction!{AND (E),
      0xA4 => instruction!{AND (H),
      0xA5 => instruction!{AND (L),
      0xA6 => instruction!{AND ((HL)),
      0xA7 => instruction!{AND (A),
      0xA8 => instruction!{XOR (B),
      0xA9 => instruction!{XOR (C),
      0xAA => instruction!{XOR (D),
      0xAB => instruction!{XOR (E),
      0xAC => instruction!{XOR (H),
      0xAD => instruction!{XOR (L),
      0xAE => instruction!{XOR ((HL)),
      0xAF => instruction!{XOR (A),

      0xB0 => instruction!{OR (B),
      0xB1 => instruction!{OR (C),
      0xB2 => instruction!{OR (D),
      0xB3 => instruction!{OR (E),
      0xB4 => instruction!{OR (H),
      0xB5 => instruction!{OR (L),
      0xB6 => instruction!{OR ((HL)),
      0xB7 => instruction!{OR (A),
      0xB8 => instruction!{CP B),
      0xB9 => instruction!{CP C),
      0xBA => instruction!{CP D),
      0xBB => instruction!{CP E),
      0xBC => instruction!{CP H),
      0xBD => instruction!{CP L),
      0xBE => instruction!{CP (HL)),
      0xBF => instruction!{CP A),

      0xC0 => instruction!{RET(NZ),
      0xC1 => instruction!{POP(BC),
      0xC2 => instruction!{JP(NZ, a16),
      0xC3 => instruction!{JP(a16),
      0xC4 => instruction!{CALL(NZ, a16),
      0xC5 => instruction!{PUSH(BC),
      0xC6 => instruction!{instruction!{ADD (A, d8),
      0xC7 => instruction!{RST(00H),
      0xC8 => instruction!{RET(Z),
      0xC9 => instruction!{RET(),
      0xCA => instruction!{JP(Z, a16),
      0xCB => {
        self.registers.pc += size_of_val(&instruction);
        self.execute_instruction_cb()
      },
      0xCC => instruction!{CALL Z, a16),
      0xCD => instruction!{CALL a16),
      0xCE => instruction!{ADC (A, d8),
      0xCF => instruction!{RST 08H),

      0xD0 => instruction!{RET NC),
      0xD1 => instruction!{POP DE),
      0xD2 => instruction!{JP NC, a16),
      /* 0xD3 */
      0xD4 => instruction!{CALL NC, a16),
      0xD5 => instruction!{PUSH DE),
      0xD6 => instruction!{SUB (d8),
      0xD7 => instruction!{RST 10H),
      0xD8 => instruction!{RET C),
      0xD9 => instruction!{RETI ),
      0xDA => instruction!{JP C, a16),
      /* 0xDB */
      0xDC => instruction!{CALL C, a16),
      /* 0xDD */
      0xDE => instruction!{SBC (A, d8),
      0xDF => instruction!{RST 18H),

      0xE0 => instruction!{LDH (a8), A),
      0xE1 => instruction!{POP HL),
      0xE2 => instruction!{LD (C), A),
      /* 0xE3 */
      /* 0xE4 */
      0xE5 => instruction!{PUSH HL),
      0xE6 => instruction!{AND (d8),
      0xE7 => instruction!{RST 20H),
      0xE8 => instruction!{ADD (SP, r8),
      0xE9 => instruction!{JP (HL)),
      0xEA => instruction!{LD (a16), A),
      /* 0xEB */
      /* 0xEC */
      /* 0xED */
      0xEE => instruction!{XOR (d8),
      0xEF => instruction!{RST 28H),

      0xF0 => instruction!{LDH A, (a8)),
      0xF1 => instruction!{POP AF),
      0xF2 => instruction!{LD A, (C)),
      0xF3 => instruction!{DI ),
      /* 0xF4 */
      0xF5 => instruction!{PUSH AF),
      0xF6 => instruction!{OR (d8),
      0xF7 => instruction!{RST 30H),
      0xF8 => instruction!{LD HL, SP+r8),
      0xF9 => instruction!{LD SP, HL),
      0xFA => instruction!{LD A, (a16)),
      0xFB => instruction!{EI ),
      /* 0xFC */
      /* 0xFD */
      0xFE => instruction!{CP d8),
      0xFF => instruction!{RST 38H),

      _ => {
        panic!("Unsupported instruction 0x{:02X}", instruction);
      }
    }
  }

  fn execute_instruction_cb(&mut self) -> u8 {
    let instruction_cb = self.memory_map.read_u8(self.registers.pc as usize);
    print!("0xCB{:02X}:", instruction);
    match instruction_cb {
      0x00 => instruction_cb!(RLB B),
      0x01 => instruction_cb!(RLC C),
      0x02 => instruction_cb!(RLC D),
      0x03 => instruction_cb!(RLC E),
      0x04 => instruction_cb!(RLC H),
      0x05 => instruction_cb!(RLC L),
      0x06 => instruction_cb!(RLC (HL)),
      0x07 => instruction_cb!(RLC A),
      0x08 => instruction_cb!(RRC B),
      0x09 => instruction_cb!(RRC C),
      0x0A => instruction_cb!(RRC D),
      0x0B => instruction_cb!(RRC E),
      0x0C => instruction_cb!(RRC H),
      0x0D => instruction_cb!(RRC L),
      0x0E => instruction_cb!(RRC (HL)),
      0x0F => instruction_cb!(RRC A),

      0x10 => instruction_cb!(RL B),
      0x11 => instruction_cb!(RL C),
      0x12 => instruction_cb!(RL D),
      0x13 => instruction_cb!(RL E),
      0x14 => instruction_cb!(RL H),
      0x15 => instruction_cb!(RL L),
      0x16 => instruction_cb!(RL (HL)),
      0x17 => instruction_cb!(RL A),
      0x18 => instruction_cb!(RR B),
      0x19 => instruction_cb!(RR C),
      0x1A => instruction_cb!(RR D),
      0x1B => instruction_cb!(RR E),
      0x1C => instruction_cb!(RR H),
      0x1D => instruction_cb!(RR L),
      0x1E => instruction_cb!(RR (HL)),
      0x1F => instruction_cb!(RR A),

      0x20 => instruction_cb!(SLA B),
      0x21 => instruction_cb!(SLA C),
      0x22 => instruction_cb!(SLA D),
      0x23 => instruction_cb!(SLA E),
      0x24 => instruction_cb!(SLA H),
      0x25 => instruction_cb!(SLA L),
      0x26 => instruction_cb!(SLA (HL)),
      0x27 => instruction_cb!(SLA A),
      0x28 => instruction_cb!(SRA B),
      0x29 => instruction_cb!(SRA C),
      0x2A => instruction_cb!(SRA D),
      0x2B => instruction_cb!(SRA E),
      0x2C => instruction_cb!(SRA H),
      0x2D => instruction_cb!(SRA L),
      0x2E => instruction_cb!(SRA (HL)),
      0x2F => instruction_cb!(SRA A),

      0x30 => instruction_cb!(SWAP B),
      0x31 => instruction_cb!(SWAP C),
      0x32 => instruction_cb!(SWAP D),
      0x33 => instruction_cb!(SWAP E),
      0x34 => instruction_cb!(SWAP H),
      0x35 => instruction_cb!(SWAP L),
      0x36 => instruction_cb!(SWAP (HL)),
      0x37 => instruction_cb!(SWAP A),
      0x38 => instruction_cb!(SRL B),
      0x39 => instruction_cb!(SRL C),
      0x3A => instruction_cb!(SRL D),
      0x3B => instruction_cb!(SRL E),
      0x3C => instruction_cb!(SRL H),
      0x3D => instruction_cb!(SRL L),
      0x3E => instruction_cb!(SRL (HL)),
      0x3F => instruction_cb!(SRL A),

      0x40 => instruction_cb!(BIT 0, B),
      0x41 => instruction_cb!(BIT 0, C),
      0x42 => instruction_cb!(BIT 0, D),
      0x43 => instruction_cb!(BIT 0, E),
      0x44 => instruction_cb!(BIT 0, H),
      0x45 => instruction_cb!(BIT 0, L),
      0x46 => instruction_cb!(BIT 0, (HL)),
      0x47 => instruction_cb!(BIT 0, A),
      0x48 => instruction_cb!(BIT 1, B),
      0x49 => instruction_cb!(BIT 1, C),
      0x4A => instruction_cb!(BIT 1, D),
      0x4B => instruction_cb!(BIT 1, E),
      0x4C => instruction_cb!(BIT 1, H),
      0x4D => instruction_cb!(BIT 1, L),
      0x4E => instruction_cb!(BIT 1, (HL)),
      0x4F => instruction_cb!(BIT 1, A),

      0x50 => instruction_cb!(BIT 2, B),
      0x51 => instruction_cb!(BIT 2, C),
      0x52 => instruction_cb!(BIT 2, D),
      0x53 => instruction_cb!(BIT 2, E),
      0x54 => instruction_cb!(BIT 2, H),
      0x55 => instruction_cb!(BIT 2, L),
      0x56 => instruction_cb!(BIT 2, (HL)),
      0x57 => instruction_cb!(BIT 2, A),
      0x58 => instruction_cb!(BIT 3, B),
      0x59 => instruction_cb!(BIT 3, C),
      0x5A => instruction_cb!(BIT 3, D),
      0x5B => instruction_cb!(BIT 3, E),
      0x5C => instruction_cb!(BIT 3, H),
      0x5D => instruction_cb!(BIT 3, L),
      0x5E => instruction_cb!(BIT 3, (HL)),
      0x5F => instruction_cb!(BIT 3, A),

      0x60 => instruction_cb!(BIT 4, B),
      0x61 => instruction_cb!(BIT 4, C),
      0x62 => instruction_cb!(BIT 4, D),
      0x63 => instruction_cb!(BIT 4, E),
      0x64 => instruction_cb!(BIT 4, H),
      0x65 => instruction_cb!(BIT 4, L),
      0x66 => instruction_cb!(BIT 4, (HL)),
      0x67 => instruction_cb!(BIT 4, A),
      0x68 => instruction_cb!(BIT 5, B),
      0x69 => instruction_cb!(BIT 5, C),
      0x6A => instruction_cb!(BIT 5, D),
      0x6B => instruction_cb!(BIT 5, E),
      0x6C => instruction_cb!(BIT 5, H),
      0x6D => instruction_cb!(BIT 5, L),
      0x6E => instruction_cb!(BIT 5, (HL)),
      0x6F => instruction_cb!(BIT 5, A),

      0x70 => instruction_cb!(BIT 6, B),
      0x71 => instruction_cb!(BIT 6, C),
      0x72 => instruction_cb!(BIT 6, D),
      0x73 => instruction_cb!(BIT 6, E),
      0x74 => instruction_cb!(BIT 6, H),
      0x75 => instruction_cb!(BIT 6, L),
      0x76 => instruction_cb!(BIT 6, (HL)),
      0x77 => instruction_cb!(BIT 6, A),
      0x78 => instruction_cb!(BIT 7, B),
      0x79 => instruction_cb!(BIT 7, C),
      0x7A => instruction_cb!(BIT 7, D),
      0x7B => instruction_cb!(BIT 7, E),
      0x7C => instruction_cb!(BIT 7, H),
      0x7D => instruction_cb!(BIT 7, L),
      0x7E => instruction_cb!(BIT 7, (HL)),
      0x7F => instruction_cb!(BIT 7, A),

      0x80 => instruction_cb!(RES 0, B),
      0x81 => instruction_cb!(RES 0, C),
      0x82 => instruction_cb!(RES 0, D),
      0x83 => instruction_cb!(RES 0, E),
      0x84 => instruction_cb!(RES 0, H),
      0x85 => instruction_cb!(RES 0, L),
      0x86 => instruction_cb!(RES 0, (HL)),
      0x87 => instruction_cb!(RES 0, A),
      0x88 => instruction_cb!(RES 1, B),
      0x89 => instruction_cb!(RES 1, C),
      0x8A => instruction_cb!(RES 1, D),
      0x8B => instruction_cb!(RES 1, E),
      0x8C => instruction_cb!(RES 1, H),
      0x8D => instruction_cb!(RES 1, L),
      0x8E => instruction_cb!(RES 1, (HL)),
      0x8F => instruction_cb!(RES 1, A),

      0x90 => instruction_cb!(RES 2, B),
      0x91 => instruction_cb!(RES 2, C),
      0x92 => instruction_cb!(RES 2, D),
      0x93 => instruction_cb!(RES 2, E),
      0x94 => instruction_cb!(RES 2, H),
      0x95 => instruction_cb!(RES 2, L),
      0x96 => instruction_cb!(RES 2, (HL)),
      0x97 => instruction_cb!(RES 2, A),
      0x98 => instruction_cb!(RES 3, B),
      0x99 => instruction_cb!(RES 3, C),
      0x9A => instruction_cb!(RES 3, D),
      0x9B => instruction_cb!(RES 3, E),
      0x9C => instruction_cb!(RES 3, H),
      0x9D => instruction_cb!(RES 3, L),
      0x9E => instruction_cb!(RES 3, (HL)),
      0x9F => instruction_cb!(RES 3, A),

      0xA0 => instruction_cb!(RES 4, B),
      0xA1 => instruction_cb!(RES 4, C),
      0xA2 => instruction_cb!(RES 4, D),
      0xA3 => instruction_cb!(RES 4, E),
      0xA4 => instruction_cb!(RES 4, H),
      0xA5 => instruction_cb!(RES 4, L),
      0xA6 => instruction_cb!(RES 4, (HL)),
      0xA7 => instruction_cb!(RES 4, A),
      0xA8 => instruction_cb!(RES 5, B),
      0xA9 => instruction_cb!(RES 5, C),
      0xAA => instruction_cb!(RES 5, D),
      0xAB => instruction_cb!(RES 5, E),
      0xAC => instruction_cb!(RES 5, H),
      0xAD => instruction_cb!(RES 5, L),
      0xAE => instruction_cb!(RES 5, (HL)),
      0xAF => instruction_cb!(RES 5, A),

      0xB0 => instruction_cb!(RES 6, B),
      0xB1 => instruction_cb!(RES 6, C),
      0xB2 => instruction_cb!(RES 6, D),
      0xB3 => instruction_cb!(RES 6, E),
      0xB4 => instruction_cb!(RES 6, H),
      0xB5 => instruction_cb!(RES 6, L),
      0xB6 => instruction_cb!(RES 6, (HL)),
      0xB7 => instruction_cb!(RES 6, A),
      0xB8 => instruction_cb!(RES 7, B),
      0xB9 => instruction_cb!(RES 7, C),
      0xBA => instruction_cb!(RES 7, D),
      0xBB => instruction_cb!(RES 7, E),
      0xBC => instruction_cb!(RES 7, H),
      0xBD => instruction_cb!(RES 7, L),
      0xBE => instruction_cb!(RES 7, (HL)),
      0xBF => instruction_cb!(RES 7, A),

      0xC0 => instruction_cb!(SET 0, B),
      0xC1 => instruction_cb!(SET 0, C),
      0xC2 => instruction_cb!(SET 0, D),
      0xC3 => instruction_cb!(SET 0, E),
      0xC4 => instruction_cb!(SET 0, H),
      0xC5 => instruction_cb!(SET 0, L),
      0xC6 => instruction_cb!(SET 0, (HL)),
      0xC7 => instruction_cb!(SET 0, A),
      0xC8 => instruction_cb!(SET 1, B),
      0xC9 => instruction_cb!(SET 1, C),
      0xCA => instruction_cb!(SET 1, D),
      0xCB => instruction_cb!(SET 1, E),
      0xCC => instruction_cb!(SET 1, H),
      0xCD => instruction_cb!(SET 1, L),
      0xCE => instruction_cb!(SET 1, (HL)),
      0xCF => instruction_cb!(SET 1, A),

      0xD0 => instruction_cb!(SET 2, B),
      0xD1 => instruction_cb!(SET 2, C),
      0xD2 => instruction_cb!(SET 2, D),
      0xD3 => instruction_cb!(SET 2, E),
      0xD4 => instruction_cb!(SET 2, H),
      0xD5 => instruction_cb!(SET 2, L),
      0xD6 => instruction_cb!(SET 2, (HL)),
      0xD7 => instruction_cb!(SET 2, A),
      0xD8 => instruction_cb!(SET 3, B),
      0xD9 => instruction_cb!(SET 3, C),
      0xDA => instruction_cb!(SET 3, D),
      0xDB => instruction_cb!(SET 3, E),
      0xDC => instruction_cb!(SET 3, H),
      0xDD => instruction_cb!(SET 3, L),
      0xDE => instruction_cb!(SET 3, (HL)),
      0xDF => instruction_cb!(SET 3, A),

      0xE0 => instruction_cb!(SET 4, B),
      0xE1 => instruction_cb!(SET 4, C),
      0xE2 => instruction_cb!(SET 4, D),
      0xE3 => instruction_cb!(SET 4, E),
      0xE4 => instruction_cb!(SET 4, H),
      0xE5 => instruction_cb!(SET 4, L),
      0xE6 => instruction_cb!(SET 4, (HL)),
      0xE7 => instruction_cb!(SET 4, A),
      0xE8 => instruction_cb!(SET 5, B),
      0xE9 => instruction_cb!(SET 5, C),
      0xEA => instruction_cb!(SET 5, D),
      0xEB => instruction_cb!(SET 5, E),
      0xEC => instruction_cb!(SET 5, H),
      0xED => instruction_cb!(SET 5, L),
      0xEE => instruction_cb!(SET 5, (HL)),
      0xEF => instruction_cb!(SET 5, A),

      0xF0 => instruction_cb!(SET 6, B),
      0xF1 => instruction_cb!(SET 6, C),
      0xF2 => instruction_cb!(SET 6, D),
      0xF3 => instruction_cb!(SET 6, E),
      0xF4 => instruction_cb!(SET 6, H),
      0xF5 => instruction_cb!(SET 6, L),
      0xF6 => instruction_cb!(SET 6, (HL)),
      0xF7 => instruction_cb!(SET 6, A),
      0xF8 => instruction_cb!(SET 7, B),
      0xF9 => instruction_cb!(SET 7, C),
      0xFA => instruction_cb!(SET 7, D),
      0xFB => instruction_cb!(SET 7, E),
      0xFC => instruction_cb!(SET 7, H),
      0xFD => instruction_cb!(SET 7, L),
      0xFE => instruction_cb!(SET 7, (HL)),
      0xFF => instruction_cb!(SET 7, A),

      _ => {
        panic!("Unsupported CB instruction 0x{:02X}", instruction);
      }
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

  // pub fn input(&mut self, inputs: u8) {
  //   if inputs != 0 {
  //     println!("input: {:>08b}", inputs);
  //   }
  //   if inputs & (1 << 0) != 0 {
  //     self.screen.fill(0xec, 0xd0, 0x25, 0xff);
  //   } else if inputs & (1 << 1) != 0 {
  //     self.screen.fill(0xce, 0xce, 0xce, 0xff);
  //   } else {
  //     self.screen.fill(0xff, 0xff, 0xff, 0xff);
  //   }
  // }

  // pub fn render(&mut self, callback: fn(*mut u32, usize, usize)) {
  //   let (pixels, width, height) = self.screen.dump();
  //   callback(pixels, width, height);
  // }
}

impl Drop for GameBoyColor {
  fn drop(&mut self) {}
}
