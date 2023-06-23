use crate::parser::Instruction;
extern crate libc;
use std::mem::{self, MaybeUninit};

const PAGE_SIZE: usize = 4096;

/* General notes
 * We store the pointer in %rax
 */

pub struct Program {
    func: fn(bf_memory: *mut u8) -> i64,
}

impl Program {
    pub fn run(&self, memory: &mut [u8]) -> i64 {
        let func = self.func;
        func(memory.as_mut_ptr())
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            // TODO: Fix memory leak. Probably just the type system screwing me up
            //libc::free(self.func as *mut libc::c_void);
        }
    }
}

unsafe fn allocate() -> *mut libc::c_void {
    let contents = MaybeUninit::<*mut u8>::uninit();
    libc::posix_memalign(
        contents.as_ptr() as *mut *mut libc::c_void,
        PAGE_SIZE,
        PAGE_SIZE,
    );
    let contents = *contents.as_ptr() as *mut libc::c_void;
    libc::mprotect(
        contents,
        PAGE_SIZE,
        libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
    );
    libc::memset(contents, 0, PAGE_SIZE);
    contents.offset(1)
}

fn push_byte(dest: &mut *mut libc::c_void, byte: u8) {
    unsafe {
        *(*dest as *mut u8) = byte;
        *dest = dest.offset(1);
    }
}

fn push_add(dest: &mut *mut libc::c_void, count: u64) {
    push_byte(dest, 0x01);
    //push_byte(dest, count);
}

pub fn compile(code: &Vec<Instruction>) -> Program {
    let contents = unsafe { allocate() };

    let mut pc = contents;
    // Returning the current memory position does make sense after all
    // Also, the addition ,instruction supports %rax directly
    // PUSH RBP
    push_byte(&mut pc, 0x50 + 0x05);
    // MOV RBP RSP
    push_byte(&mut pc, 0x48);
    push_byte(&mut pc, 0x89);
    push_byte(&mut pc, 0xE5);
    // PUSH RDI
    push_byte(&mut pc, 0x50 + 0x07);
    // POP RAX
    push_byte(&mut pc, 0x58 + 0x00);

    for instr in code {
        match *instr {
            Instruction::PointerIncrement(i) => {
                // TODO: We are kind of assuming that i fits in an u8
                // ADD id
                push_byte(&mut pc, 0x04);
                // operand
                push_byte(&mut pc, i as u8);
            }
            Instruction::PointerDecrement(i) => {
                panic!("Unsupported instruction");
            }
            Instruction::Plus(i) => {
                // ADD reg/mem8, imm8
                // 80 /0 ib
                push_byte(&mut pc, 0x80);
                // ModRM
                push_byte(&mut pc, 0b00_000_000);
                // Add with
                push_byte(&mut pc, i as u8);
            }
            Instruction::Minus(i) => {
                panic!("Unsupported instruction");
            }
            Instruction::Output(i) => {
                break;
                //panic!("Unsupported instruction");
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
    push_byte(&mut pc, 0x48);
    push_byte(&mut pc, 0x89);
    push_byte(&mut pc, 0xec);
    // POP RBP
    push_byte(&mut pc, 0x58 + 0x05);
    // RET
    push_byte(&mut pc, 0xC3);

    let func = unsafe { mem::transmute(contents) };
    Program { func }
}
