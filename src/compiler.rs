use crate::parser::Instruction;
extern crate libc;
use std::mem::{self, MaybeUninit};

const PAGE_SIZE: usize = 4096;

pub struct Program {
    pub func: fn() -> i64,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.func as *mut libc::c_void);
        }
    }
}

pub fn compile(code: &Vec<Instruction>) -> Program {
    let contents : *mut libc::c_void;
    unsafe {
        let mut _contents = MaybeUninit::<*mut u8>::uninit(); // avoid uninitalized warning
        libc::posix_memalign(_contents.as_ptr() as *mut *mut libc::c_void, PAGE_SIZE, PAGE_SIZE);
        contents = *_contents.as_ptr() as *mut libc::c_void;
        libc::mprotect(contents, PAGE_SIZE, libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE);
        libc::memset(contents, 0xc3, PAGE_SIZE);
    }

    let func : fn() -> i64 = unsafe { mem::transmute(contents) };
    Program { func }
}
