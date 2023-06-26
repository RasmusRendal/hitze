use super::parser::Instruction;
extern crate libc;
pub mod assembler;
use assembler::*;

/* General notes
 * Arguments: %rdi memory region start
 * %rsi memory region len
 * %rdx memory region end
 * We store the pointer in %rax
 */

pub fn compile(code: &Vec<Instruction>) -> Program {
    let mut assembler = allocate(code.len() * 10);

    assembler.push(RBP);
    assembler.mov_reg_reg(RBP, RSP);

    assembler.push(RDI);
    assembler.pop(RAX);

    assembler.mov_reg_reg(RDX, RAX);
    assembler.add_reg_reg(RDX, RSI);

    let mut leftloops: Vec<isize> = vec![];

    for instr in code {
        match *instr {
            Instruction::MovePointer(i) => {
                if i < 0 {
                    assembler.sub_rax_imm32(isize::abs(i) as u32);
                    assembler.cmp_reg_reg(RDI, RAX);
                    assembler.jna(3);
                    assembler.add_reg_reg(RAX, RSI);
                } else {
                    assembler.add_rax_imm32(i as u32);
                    assembler.cmp_reg_reg(RDX, RAX);
                    assembler.jnbe(3);
                    assembler.sub_reg_reg(RAX, RSI);
                }
            }
            Instruction::Add(i) => {
                if i < 0 {
                    assembler.sub_regmem8_imm8(RAX, i8::abs(i) as u8);
                } else {
                    assembler.add_regmem8_imm8(RAX, i as u8);
                }
            }
            Instruction::ResetByte => {
                assembler.mov_mem8_imm8(RAX, 0);
            }
            Instruction::Output(i) => {
                assembler.push(RAX);
                assembler.push(RDI);
                assembler.push(RSI);
                assembler.push(RDX);
                assembler.mov_reg_reg(RSI, RAX);
                assembler.mov_reg_imm64(RDI, 1);
                assembler.mov_reg_imm64(RDX, 1);
                for _ in 0..i {
                    assembler.mov_reg_imm64(RAX, 1);
                    assembler.syscall();
                }
                assembler.pop(RDX);
                assembler.pop(RSI);
                assembler.pop(RDI);
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
                let relpos = leftpos - assembler.cur_index;
                assembler.jne((relpos - 2) as i32);
                let frontpos = (assembler.cur_index - leftpos - 4) as i32;
                let frontposbytes = frontpos.to_le_bytes();
                assembler.update_byte(leftpos, frontposbytes[0]);
                assembler.update_byte(leftpos + 1, frontposbytes[1]);
                assembler.update_byte(leftpos + 2, frontposbytes[2]);
                assembler.update_byte(leftpos + 3, frontposbytes[3]);
            }
            Instruction::AddRel(offset, mul) => {
                assembler.mov_reg_mem8(RCX, RAX);
                if mul != 1 {
                    // TODO: Use an actual mul instruction, this is stupid
                    for _ in 2..mul {
                        assembler.add_reg_reg(RCX, RCX);
                    }
                }
                assembler.add_rax8disp_reg(offset as i8, RCX);
            }
            Instruction::Nop => {}
        }
    }
    assembler.mov_reg_reg(RSP, RBP);
    assembler.pop(RBP);
    assembler.ret();

    assembler.create_program()
}
