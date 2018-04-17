use super::{super::memory_map::ToMemoryMap, Eat};
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<LDH, Pointer<A8>, A> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: Pointer<A8>, _: A) -> Cycle {
    let a8: u8 = Processor::eat(r, mm);
    mm.write(0xFF | a8 as u16, r.get_a());
    12
  }
}
