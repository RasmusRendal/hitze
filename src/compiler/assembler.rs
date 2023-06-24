use std::mem;

const PAGE_SIZE: usize = 4096;

pub struct Program {
    pub func: fn(bf_memory: *mut u8) -> i64,
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
            libc::free(self.func as *mut libc::c_void);
        }
    }
}

pub struct Assembler {
    page: *mut libc::c_void,
    cur_index: isize,
}

pub unsafe fn allocate() -> Assembler {
    let contents = mem::MaybeUninit::<*mut u8>::uninit();
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
    Assembler {
        page: contents,
        cur_index: 0,
    }
}

impl Assembler {
    pub fn push_byte(&mut self, byte: u8) {
        unsafe {
            *(self.page.offset(self.cur_index) as *mut u8) = byte;
            self.cur_index += 1;
        }
    }

    pub fn create_program(self) -> Program {
        unsafe {
            Program {
                func: mem::transmute(self.page),
            }
        }
    }
}
