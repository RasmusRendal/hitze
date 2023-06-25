use std::mem;

const PAGE_SIZE: usize = 4096;

pub struct Program {
    pub func: fn(bf_memory: *mut u8, len: usize) -> i64,
}

impl Program {
    pub fn run(&self, memory: &mut [u8]) -> i64 {
        let func = self.func;
        func(memory.as_mut_ptr(), memory.len())
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
    pub cur_index: isize,
}

pub unsafe fn allocate(len: usize) -> Assembler {
    let len = ((len / PAGE_SIZE) + 1) * PAGE_SIZE;
    let contents = mem::MaybeUninit::<*mut u8>::uninit();
    libc::posix_memalign(contents.as_ptr() as *mut *mut libc::c_void, PAGE_SIZE, len);
    let contents = *contents.as_ptr() as *mut libc::c_void;
    libc::mprotect(
        contents,
        len,
        libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE,
    );
    libc::memset(contents, 0, len);
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

    fn push_qword(&mut self, qword: u64) {
        let bytes = qword.to_ne_bytes();
        for i in 0..bytes.len() {
            self.push_byte(bytes[i]);
        }
    }

    fn push_dword(&mut self, dword: u32) {
        let bytes = dword.to_ne_bytes();
        for i in 0..bytes.len() {
            self.push_byte(bytes[i]);
        }
    }

    pub fn update_byte(&mut self, index: isize, byte: u8) {
        unsafe {
            *(self.page.offset(index) as *mut u8) = byte;
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

    pub fn add_reg_reg(&mut self, dst: u8, src: u8) {
        self.push_byte(REXW);
        self.push_byte(0x01);
        self.push_byte(0b11_000_000 + dst + (src << 3));
    }

    pub fn sub_reg_reg(&mut self, dst: u8, src: u8) {
        self.push_byte(REXW);
        self.push_byte(0x29);
        self.push_byte(0b11_000_000 + dst + (src << 3));
    }

    pub fn sub_regmem8_imm8(&mut self, reg: u8, imm8: u8) {
        self.push_byte(0x80);
        self.push_byte(0b00_101_000 + reg);
        self.push_byte(imm8);
    }

    pub fn sub_reg_imm32(&mut self, reg: u8, imm: u32) {
        self.push_byte(REXW);
        self.push_byte(0x81);
        self.push_byte(0b11_010_000 + reg);
        self.push_dword(imm);
    }

    pub fn add_al_imm8(&mut self, imm8: u8) {
        self.push_byte(0x04);
        self.push_byte(imm8);
    }

    pub fn add_rax_imm32(&mut self, imm32: u32) {
        self.push_byte(REXW);
        self.push_byte(0x05);
        self.push_dword(imm32);
    }

    pub fn sub_al_imm8(&mut self, imm8: u8) {
        self.push_byte(0x2C);
        self.push_byte(imm8);
    }

    pub fn sub_rax_imm32(&mut self, imm32: u32) {
        self.push_byte(REXW);
        self.push_byte(0x2D);
        self.push_dword(imm32);
    }

    pub fn mov_reg_reg(&mut self, dst: u8, src: u8) {
        self.push_byte(REXW);
        self.push_byte(0x89);
        self.push_byte(0b11_000_000 + dst + (src << 3));
    }

    pub fn mov_reg_imm64(&mut self, dst: u8, imm64: u64) {
        self.push_byte(REXW);
        self.push_byte(0xB8 + dst);
        self.push_qword(imm64);
    }

    pub fn cmp_mem8_imm8(&mut self, memreg: u8, imm8: u8) {
        self.push_byte(0x80);
        self.push_byte(0b00_111_000 + memreg);
        self.push_byte(imm8);
    }

    pub fn cmp_reg_reg(&mut self, reg1: u8, reg2: u8) {
        self.push_byte(REXW);
        self.push_byte(0x39);
        self.push_byte(0b11_000_000 + reg1 + (reg2 << 3));
    }

    // Returns a pointer to the address, so we can update it later
    pub fn jz(&mut self, rel32off: i32) -> isize {
        self.push_byte(0x0F);
        self.push_byte(0x84);
        self.push_dword(rel32off as u32);
        self.cur_index - 4
    }

    pub fn jne(&mut self, rel32off: i32) {
        self.push_byte(0x0F);
        self.push_byte(0x85);
        self.push_dword(rel32off as u32);
    }

    pub fn jna(&mut self, rel8off: i8) {
        self.push_byte(0x76);
        self.push_byte(rel8off as u8);
    }

    pub fn jnbe(&mut self, rel8off: i8) {
        self.push_byte(0x77);
        self.push_byte(rel8off as u8);
    }

    pub fn syscall(&mut self) {
        self.push_byte(0x0F);
        self.push_byte(0x05);
    }
}
