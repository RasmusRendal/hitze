use super::parser::Instruction;
extern crate libc;
mod assembler;
use assembler::*;
use std::mem;

/* General notes
 * We store the pointer in %rax
 */

pub fn compile(code: &Vec<Instruction>) -> Program {
    //let contents = unsafe { allocate() };
    let mut assembler = unsafe { allocate() };

    // Returning the current memory position does make sense after all
    // Also, the addition ,instruction supports %rax directly
    // PUSH RBP
    assembler.push_byte(0x50 + 0x05);
    // MOV RBP RSP
    assembler.push_byte(0x48);
    assembler.push_byte(0x89);
    assembler.push_byte(0xE5);
    // PUSH RDI
    assembler.push_byte(0x50 + 0x07);
    // POP RAX
    assembler.push_byte(0x58 + 0x00);

    for instr in code {
        match *instr {
            Instruction::PointerIncrement(i) => {
                // TODO: We are kind of assuming that i fits in an u8
                // ADD id
                assembler.push_byte(0x04);
                // operand
                assembler.push_byte(i as u8);
            }
            Instruction::PointerDecrement(i) => {
                panic!("Unsupported instruction");
            }
            Instruction::Plus(i) => {
                // 80 /0 ib
                assembler.push_byte(0x80);
                // ModRM
                assembler.push_byte(0b00_000_000);
                // Add with
                assembler.push_byte(i as u8);
            }
            Instruction::Minus(i) => {
                // 80 /0 ib
                assembler.push_byte(0x80);
                // ModRM
                assembler.push_byte(0b00_101_000);
                // Add with
                assembler.push_byte(i as u8);
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
    //MOV RSP RBP
    assembler.push_byte(0x48);
    assembler.push_byte(0x89);
    assembler.push_byte(0xec);
    // POP RBP
    assembler.push_byte(0x58 + 0x05);
    // RET
    assembler.push_byte(0xC3);

    assembler.create_program()
}
