use super::parser::Instruction;
extern crate libc;
mod assembler;
use assembler::*;

/* General notes
 * Arguments: %rdi memory region start
 * %rsi memory region len
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
                assembler.add_rax_imm32(i as u32);
            }
            Instruction::PointerDecrement(i) => {
                assembler.sub_rax_imm32(i as u32);
                assembler.cmp_reg_reg(RDI, RAX);
                assembler.jna(3);
                assembler.add_reg_reg(RAX, RSI);
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
                assembler.mov_reg_imm64(RDI, 1);
                assembler.mov_reg_imm64(RDX, 1);
                for _ in 0..i {
                    assembler.mov_reg_imm64(RAX, 1);
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
