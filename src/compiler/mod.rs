use super::parser::Instruction;
extern crate libc;
mod assembler;
use assembler::*;
use std::mem;

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
                assembler.push(RAX);
                assembler.mov_reg_reg(RSI, RAX);
                assembler.mov_reg_imm64(RAX, 1);
                assembler.mov_reg_imm64(RDI, 1);
                assembler.mov_reg_imm64(RDX, 1);
                for _ in 0..i {
                    assembler.syscall();
                }
                assembler.pop(RAX);
            }
            Instruction::Input(_) => {
                panic!("Unsupported instruction");
            }
            Instruction::LoopBegin(_) => {
                assembler.cmp_mem8_imm8(RAX, 0);
                leftloops.push(assembler.jz(0));
            }
            Instruction::LoopEnd(_) => {
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
