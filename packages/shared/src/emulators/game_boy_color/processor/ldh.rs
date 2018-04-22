use super::{super::memory_map::ToMemoryMap, Eat};
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<LDH, Pointer<A8>, A> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: Pointer<A8>, _: A) -> Cycle {
    let a8: u8 = Processor::eat(r, mm);
    let a = r.get_a();
    println!("LDH (a8=0x{:02X}) A=0x{:02X}", a8, a);
    mm.write(0xFF00 + a8 as u16, a);
    12
  }
}
