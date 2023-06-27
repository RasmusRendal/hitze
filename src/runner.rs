use crate::interpreter::interpret;
use crate::optimizer::{compile_inner_loops, optimize};
use crate::parser::parse;

pub const DEFAULT_COMPILE_DEPTH: usize = 2;

pub fn run(code: &str, compile_depth: usize) -> Vec<u8> {
    let mut code = parse(code);
    optimize(&mut code);
    compile_inner_loops(&mut code, compile_depth);
    let mut memory: Vec<u8> = vec![0; u16::max_value() as usize + 1];
    interpret(&code, &mut memory, 0, false);
    memory
}
