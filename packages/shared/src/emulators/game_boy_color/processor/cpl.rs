use super::super::registers::Flag;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<CPL, Void, Void> for Processor {
  fn instruction(r: &mut Registers, _: &mut MemoryMap, _: Void, _: Void) -> Cycle {
    let a = r.get_a();
    r.set_a(!a);
    r.set_flag(Flag::N, true);
    r.set_flag(Flag::H, true);
    println!("CPL");
    4
  }
}
