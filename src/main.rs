use clap::Parser;
use std::fs;
pub mod parser;
pub mod compiler;
use crate::parser::{Instruction, parse};
use crate::compiler::compile;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    trace: bool,
    #[arg(short, long)]
    interpret: bool,
    file: String,
}


fn interpret(code : &Vec<Instruction>, trace: bool) {
    let mut pc : usize = 0;
    let mut mp : u16 = 0;
    let mut memory : Vec<u8> = vec![0; u16::max_value() as usize + 1];
    while pc < code.len() {
        if trace {
            println!("Executing instruction {:?}", code[pc]);
            println!("pc: {}, mp: {}", pc, mp);
        }
        match code[pc] {
            Instruction::PointerIncrement(i) => {
                mp = mp.wrapping_add(i as u16);
                pc += 1;
            }
            Instruction::PointerDecrement(i) => {
                mp = mp.wrapping_sub(i as u16);
                pc += 1;
            }
            Instruction::Plus(i) => {
                memory[mp as usize] = memory[mp as usize].wrapping_add(i);
                pc += 1;
            }
            Instruction::Minus(i) => {
                memory[mp as usize] = memory[mp as usize].wrapping_sub(i);
                pc += 1;
            }
            Instruction::Output(i) => {
                for _ in 0..i {
                    let c: char = memory[mp as usize].into();
                    print!("{}", c);
                }
                pc += 1;
            }
            Instruction::Input(_) => {
                // TODO: Take some input
                pc += 1;
            }
            Instruction::LoopBegin(i) => {
                if memory[mp as usize] == 0 {
                    pc = i;
                } else {
                    pc += 1;
                }
            }
            Instruction::LoopEnd(i) => {
                if memory[mp as usize] != 0 {
                    pc = i;
                } else {
                    pc += 1;
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(args.file).expect("Could not read file");
    let code = parse(&code);
    if args.interpret {
        interpret(&code, args.trace);
    } else {
        let prog = compile(&code);
        // I have to do this, because prog.func() looks for an impl
        let f = &prog.func;
        f();
    }

}
