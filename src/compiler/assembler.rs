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

pub const RAX: u8 = 0b000;
pub const RCX: u8 = 0b001;
pub const RDX: u8 = 0b010;
pub const RBX: u8 = 0b011;
pub const RSP: u8 = 0b100;
pub const RBP: u8 = 0b101;
pub const RSI: u8 = 0b110;
pub const RDI: u8 = 0b111;

const REXW: u8 = 0x48;

impl Assembler {
    fn push_byte(&mut self, byte: u8) {
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

    pub fn push(&mut self, reg: u8) {
        self.push_byte(0x50 + reg);
    }

    pub fn pop(&mut self, reg: u8) {
        self.push_byte(0x58 + reg);
    }

    pub fn ret(&mut self) {
        self.push_byte(0xc3);
    }

    pub fn add_regmem8_imm8(&mut self, reg: u8, imm8: u8) {
        self.push_byte(0x80);
        self.push_byte(0b00_000_000 + (reg << 3));
        self.push_byte(imm8);
    }

    pub fn sub_regmem8_imm8(&mut self, reg: u8, imm8: u8) {
        self.push_byte(0x80);
        self.push_byte(0b00_101_000 + reg);
        self.push_byte(imm8);
    }

    pub fn add_al_imm8(&mut self, imm8: u8) {
        self.push_byte(0x04);
        self.push_byte(imm8);
    }

    pub fn sub_al_imm8(&mut self, imm8: u8) {
        self.push_byte(0x2C);
        self.push_byte(imm8);
    }

    pub fn mov_reg_reg(&mut self, dst: u8, src: u8) {
        self.push_byte(REXW);
        self.push_byte(0x89);
        self.push_byte(0b11_000_000 + dst + (src << 3));
    }
}
