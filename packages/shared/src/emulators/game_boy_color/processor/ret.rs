use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<RET, Void, Void> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: Void, _: Void) -> Cycle {
    let a16 = Processor::stack_pop_u16(r, mm);
    println!("RET");
    r.pc = a16;
    16
  }
}
