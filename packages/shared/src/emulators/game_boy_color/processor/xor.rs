use super::super::registers::Flag;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<XOR, A, Void> for Processor {
  fn instruction(r: &mut Registers, _: &mut MemoryMap, _: A, _: Void) -> Cycle {
    let a = r.get_a();
    println!("XOR A=0x{:02X}", a);
    let a = a ^ a;
    r.set_a(a);
    r.set_flag(Flag::Z, a != 0);
    r.set_flag(Flag::N, false);
    r.set_flag(Flag::H, false);
    r.set_flag(Flag::C, false);
    4
  }
}
