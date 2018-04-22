use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<STOP, Literal, Void> for Processor {
  fn instruction(_: &mut Registers, _: &mut MemoryMap, _: Literal, _: Void) -> Cycle {
    println!("STOP");
    4
  }
}
