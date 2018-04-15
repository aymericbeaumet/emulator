use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<JP, A16, Void> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: A16, _: Void) -> Cycle {
    let a16 = Processor::eat_u16(r, mm);
    r.set_pc(a16);
    16
  }
}
