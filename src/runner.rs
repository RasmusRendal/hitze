use crate::interpreter::interpret;
use crate::optimizer::{compile_inner_loops, optimize};
use crate::parser::parse;

pub fn run(code: &str) -> Vec<u8> {
    let mut code = parse(&code);
    optimize(&mut code);
    compile_inner_loops(&mut code, 1);
    let mut memory: Vec<u8> = vec![0; u16::max_value() as usize + 1];
    interpret(&code, &mut memory, false);
    memory
}
