use super::parser::Instruction;
extern crate libc;
mod assembler;
use assembler::*;

/* General notes
 * We store the pointer in %rax
 */

pub fn compile(code: &Vec<Instruction>) -> Program {
    let mut assembler = unsafe { allocate() };

    assembler.push(RBP);
    assembler.mov_reg_reg(RBP, RSP);

    assembler.push(RDI);
    assembler.pop(RAX);

    for instr in code {
        match *instr {
            Instruction::PointerIncrement(i) => {
                assembler.add_al_imm8(i as u8);
            }
            Instruction::PointerDecrement(i) => {
                panic!("Unsupported instruction");
            }
            Instruction::Plus(i) => {
                assembler.add_regmem8_imm8(RAX, i);
            }
            Instruction::Minus(i) => {
                assembler.sub_regmem8_imm8(RAX, i);
            }
            Instruction::Output(i) => {
                panic!("Unsupported instruction");
            }
            Instruction::Input(i) => {
                panic!("Unsupported instruction");
            }
            Instruction::LoopBegin(i) => {
                panic!("Unsupported instruction");
            }
            Instruction::LoopEnd(i) => {
                panic!("Unsupported instruction");
            }
        }
    }
    assembler.mov_reg_reg(RSP, RBP);
    assembler.pop(RBP);
    assembler.ret();

    assembler.create_program()
}
