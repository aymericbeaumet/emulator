use super::super::registers::Flag;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<DEC, C, Void> for Processor {
  fn instruction(r: &mut Registers, _: &mut MemoryMap, _: C, _: Void) -> Cycle {
    let c = r.get_c() - 1;
    r.set_c(c);
    r.set_flag(Flag::Z, c != 0);
    r.set_flag(Flag::N, true);
    // r.set_flag(Flag::H, true); // todo: ??
    4
  }
}
