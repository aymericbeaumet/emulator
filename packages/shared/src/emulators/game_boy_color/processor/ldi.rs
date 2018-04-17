use super::super::memory_map::ToMemoryMap;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<LDI, Pointer<HL>, A> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: Pointer<HL>, _: A) -> Cycle {
    let hl = r.get_hl();
    mm.write(hl, r.get_a());
    r.set_hl(hl + 1);
    8
  }
}
