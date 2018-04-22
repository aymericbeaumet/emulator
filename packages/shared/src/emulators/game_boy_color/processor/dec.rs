use super::super::registers::Flag;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<DEC, C, Void> for Processor {
  fn instruction(r: &mut Registers, _: &mut MemoryMap, _: C, _: Void) -> Cycle {
    let c = r.get_c();
    println!("DEC C=0x{:02X}", c);
    let (c, h) = match c {
      0 => (<u8>::max_value() - 1, true),
      _ => (c - 1, false),
    };
    r.set_c(c);
    r.set_flag(Flag::Z, c == 0);
    r.set_flag(Flag::N, true);
    r.set_flag(Flag::H, h);
    4
  }
}
