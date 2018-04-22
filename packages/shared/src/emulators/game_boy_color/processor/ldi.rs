use super::super::memory_map::ToMemoryMap;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<LDI, Pointer<HL>, A> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: Pointer<HL>, _: A) -> Cycle {
    let hl = r.get_hl();
    let a = r.get_a();
    println!("LDI (HL=0x{:04X}) A=0x{:02X}", hl, a);
    mm.write(hl, a);
    r.set_hl(hl + 1);
    8
  }
}
