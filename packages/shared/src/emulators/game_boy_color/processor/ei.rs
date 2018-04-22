use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<EI, Void, Void> for Processor {
  fn instruction(_: &mut Registers, _: &mut MemoryMap, _: Void, _: Void) -> Cycle {
    println!("EI");
    4
  }
}
