use super::parser::Instruction;
extern crate libc;
pub mod assembler;
use assembler::*;

/* General notes
 * Arguments:
 *  - %rdi memory region start
 *  - %rsi memory region len
 *  - %rdx memory pointer initial pos
 *
 * When running:
 *  - %rax memory pointer position (relative)
 *  - %rdi memory region start
 * Returns memory pointer position, relative to %rdi
 */

pub fn compile(code: &[Instruction]) -> Program {
    let mut assembler = allocate(code.len() * 10);

    assembler.push(RBP);

    assembler.mov_reg_reg(RBP, RSP);

    assembler.mov_reg_reg(RAX, RDX);

    // Argument for write syscall
    // rdx is unused the rest of the time
    assembler.mov_reg_imm64(RDX, 1);

    // TODO: Remove push_bytes, replace with the assembly thing

    let mut leftloops: Vec<isize> = vec![];

    for instr in code {
        match *instr {
            Instruction::MovePointer(i) => {
                if i < 0 {
                    // SUB RAX, i
                    assembler.push_bytes(&[0x66, 0x2D]);
                    assembler.push_bytes(&(isize::abs(i) as u16).to_ne_bytes());
                } else {
                    // ADD RAX, i
                    assembler.push_bytes(&[0x66, 0x05]);
                    assembler.push_bytes(&(i as u16).to_ne_bytes());
                }
            }
            Instruction::Add(i) => {
                if i < 0 {
                    // sub    BYTE PTR [rdi+rax*1],0xfa
                    assembler.push_bytes(&[0x80, 0x2C, 0x07, i8::abs(i) as u8]);
                } else {
                    // add    BYTE PTR [rdi+rax*1],0xfa
                    assembler.push_bytes(&[0x80, 0x04, 0x07, i as u8]);
                }
            }
            Instruction::ResetByte => {
                // mov    BYTE PTR [rdi+rax*1],0x0
                assembler.push_bytes(&[0xC6, 0x04, 0x07, 0x00]);
            }
            Instruction::Output(i) => {
                assembler.push(RAX);
                assembler.push(RDI);
                assembler.mov_reg_reg(RSI, RAX);
                assembler.add_reg_reg(RSI, RDI);
                assembler.mov_reg_imm64(RDI, 1);
                for _ in 0..i {
                    assembler.mov_reg_imm64(RAX, 1);
                    assembler.syscall();
                }
                assembler.pop(RDI);
                assembler.pop(RAX);
            }
            Instruction::Input(_) => {
                panic!("Unsupported instruction");
            }
            Instruction::LoopBegin(_) => {
                // cmp    BYTE PTR [rdi+rax*1],0x0
                assembler.push_bytes(&[0x80, 0x3C, 0x07, 0x00]);
                leftloops.push(assembler.jz(0));
            }
            Instruction::LoopEnd(_) => {
                // cmp    BYTE PTR [rdi+rax*1],0x0
                assembler.push_bytes(&[0x80, 0x3C, 0x07, 0x00]);
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
                // mov    cl,BYTE PTR [rdi+rax*1]
                assembler.push_bytes(&[0x8A, 0x0C, 0x07]);
                if mul != 1 {
                    assembler.imul_reg_imm32(RCX, mul as u8);
                }
                // add    BYTE PTR [rdi+rax*1+0x4],cl
                assembler.push_bytes(&[0x00, 0x4C, 0x07, offset as u8]);
            }
            Instruction::Nop(_) => {}
            Instruction::Call(_) => {
                panic!("Nested calls not supported");
            }
        }
    }
    assembler.mov_reg_reg(RSP, RBP);
    assembler.pop(RBP);
    assembler.ret();

    assembler.create_program(code.to_vec())
}
