use super::super::registers::Flag;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<BIT, Literal, H> for Processor {
  fn instruction(r: &mut Registers, _: &mut MemoryMap, literal: Literal, _: H) -> Cycle {
    let h = r.get_h();
    println!("BIT {} H", literal);
    let h = h | (1 << literal);
    r.set_h(h);
    r.set_flag(Flag::Z, h == 0);
    r.set_flag(Flag::N, false);
    r.set_flag(Flag::H, true);
    8
  }
}
