use super::Eat;
use super::{instructions::*, operands::*, Cycle, Instruction, MemoryMap, Processor, Registers};

impl Instruction<JP, A16, Void> for Processor {
  fn instruction(r: &mut Registers, mm: &mut MemoryMap, _: A16, _: Void) -> Cycle {
    let a16: u16 = Processor::eat(r, mm);
    println!("JP a16=0x{:04X}", a16);
    r.set_pc(a16);
    16
  }
}
