use super::Eat;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<CALL, A16, Void> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: A16, _: Void) -> Cycle {
    let a16: u16 = Processor::eat(r, mm);
    let pc = r.pc;
    Processor::stack_push_u16(r, mm, pc);
    r.pc = a16;
    24
  }
}
