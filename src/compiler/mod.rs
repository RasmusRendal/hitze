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

    let mut leftloops: Vec<isize> = vec![];

    for instr in code {
        match *instr {
            Instruction::PointerIncrement(i) => {
                assembler.add_al_imm8(i as u8);
            }
            Instruction::PointerDecrement(i) => {
                assembler.sub_al_imm8(i as u8);
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
                assembler.cmp_mem8_imm8(RAX, 0);
                leftloops.push(assembler.jz(0));
            }
            Instruction::LoopEnd(i) => {
                assembler.cmp_mem8_imm8(RAX, 0);
                let leftpos = leftloops.pop().unwrap();
                let relpos = leftpos - assembler.cur_index - 1;
                assembler.jne(relpos as i8);
                assembler.update_byte(leftpos, (assembler.cur_index - leftpos - 1) as u8);
            }
        }
    }
    assembler.mov_reg_reg(RSP, RBP);
    assembler.pop(RBP);
    assembler.ret();

    assembler.create_program()
}
