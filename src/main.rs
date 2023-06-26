use clap::Parser;
use std::fs;
pub mod compiler;
pub mod interpreter;
pub mod optimizer;
pub mod parser;
use crate::compiler::compile;
use crate::interpreter::interpret;
use crate::optimizer::optimize;
use crate::parser::parse;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    trace: bool,
    #[arg(short, long)]
    interpret: bool,
    file: String,
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(args.file).expect("Could not read file");
    let mut code = parse(&code);
    let mut memory: Vec<u8> = vec![0; u16::max_value() as usize + 1];
    if args.interpret {
        interpret(&code, memory.as_mut_slice(), args.trace);
    } else {
        optimize(&mut code);
        let prog = compile(&code);
        prog.run(memory.as_mut_slice());
    }
}
