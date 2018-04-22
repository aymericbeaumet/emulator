use super::super::registers::Flag;
use super::{instructions::*, operands::*, Cycle, Eat, Instruction, MemoryMap, Processor, Registers};

impl Instruction<JR, NZ, R8> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: NZ, _: R8) -> Cycle {
    if r.get_flag(Flag::Z) {
      println!("JR NZ");
      return 8;
    }
    let r8: i8 = Processor::eat(r, mm);
    println!("JR NZ 0x{:02X}", r8);
    r.pc = (if r8 >= 0 {
      r.pc + r8 as u16
    } else {
      r.pc - (-r8) as u16
    }) - 2;
    12
  }
}

impl Instruction<JR, Z, R8> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: Z, _: R8) -> Cycle {
    if !r.get_flag(Flag::Z) {
      println!("JR Z");
      return 8;
    }
    let r8: i8 = Processor::eat(r, mm);
    println!("JR Z 0x{:02X}", r8);
    r.pc = (if r8 >= 0 {
      r.pc + r8 as u16
    } else {
      r.pc - (-r8) as u16
    }) - 2;
    12
  }
}
