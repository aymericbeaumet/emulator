use super::Eat;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<LD, A, D8> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: A, _: D8) -> Cycle {
    let d8: u8 = Processor::eat(r, mm);
    println!("LD A d8=0x{:02X}", d8);
    r.set_a(d8);
    8
  }
}

impl Instruction<LD, C, D8> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: C, _: D8) -> Cycle {
    let d8: u8 = Processor::eat(r, mm);
    println!("LD C d8=0x{:02X}", d8);
    r.set_c(d8);
    8
  }
}

impl Instruction<LD, HL, D16> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: HL, _: D16) -> Cycle {
    let d16: u16 = Processor::eat(r, mm);
    println!("LD HL d16=0x{:04X}", d16);
    r.set_hl(d16);
    12
  }
}

impl Instruction<LD, SP, D16> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: SP, _: D16) -> Cycle {
    let d16: u16 = Processor::eat(r, mm);
    println!("LD SP d16=0x{:04X}", d16);
    r.set_sp(d16);
    12
  }
}
