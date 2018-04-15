// use super::{Cycle, Instruction, Instructions::*, Operands::*, Processor, Registers, MemoryMap};

// impl Instruction<LD, BC, d16> for Processor {
//   fn instruction((&mut r: Registers, &mm: MemoryMap), Re, op1: BC, op2: d16) -> Cycle {
//     12
//   }
// }

// impl Instruction<LD, pointer<BC>, A> for Processor {
//   fn instruction((&mut r: Registers, &mm: MemoryMap), op1: pointer<BC>, op2: A) -> Cycle {
//     8
//   }
// }
