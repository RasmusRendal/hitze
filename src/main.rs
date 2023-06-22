use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    trace: bool,
    file: String,
}

#[derive(Debug)]
enum Instruction {
    PointerIncrement(usize),
    PointerDecrement(usize),
    Plus(u8),
    Minus(u8),
    Output(usize),
    Input(usize),
    LoopBegin(usize),
    LoopEnd(usize),
}

fn parse(code : &String) -> Vec<Instruction> {
    let mut output: Vec<Instruction> = vec![];
    let mut i : Option<Instruction> = None;
    let mut loop_begins : Vec<usize> = vec![];
    for c in code.chars() {
        match c {
            '>' => {
                match i {
                    Some(Instruction::PointerIncrement(j)) => i = Some(Instruction::PointerIncrement(j+1)),
                    None => i = Some(Instruction::PointerIncrement(1)),
                    _ => {
                        output.push(i.unwrap());
                        i = Some(Instruction::PointerIncrement(1));
                    }
                }
            }
            '<' => {
                match i {
                    Some(Instruction::PointerDecrement(j)) => i = Some(Instruction::PointerDecrement(j+1)),
                    None => i = Some(Instruction::PointerDecrement(1)),
                    _ => {
                        output.push(i.unwrap());
                        i = Some(Instruction::PointerDecrement(1));
                    }
                }
            }
            '+' => {
                match i {
                    Some(Instruction::Plus(j)) => i = Some(Instruction::Plus(j+1)),
                    None => i = Some(Instruction::Plus(1)),
                    _ => {
                        output.push(i.unwrap());
                        i = Some(Instruction::Plus(1));
                    }
                }
            }
            '-' => {
                match i {
                    Some(Instruction::Minus(j)) => i = Some(Instruction::Minus(j+1)),
                    None => i = Some(Instruction::Minus(1)),
                    _ => {
                        output.push(i.unwrap());
                        i = Some(Instruction::Minus(1));
                    }
                }
            }
            '.' => {
                match i {
                    Some(Instruction::Output(j)) => i = Some(Instruction::Output(j+1)),
                    None => i = Some(Instruction::Output(1)),
                    _ => {
                        output.push(i.unwrap());
                        i = Some(Instruction::Output(1));
                    }
                }
            }
            ',' => {
                match i {
                    Some(Instruction::Input(j)) => i = Some(Instruction::Input(j+1)),
                    None => i = Some(Instruction::Input(1)),
                    _ => {
                        output.push(i.unwrap());
                        i = Some(Instruction::Input(1));
                    }
                }
            }
            '[' => {
                if let Some(inst) = i {
                    output.push(inst);
                    i = None;
                }
                loop_begins.push(output.len());
                output.push(Instruction::LoopBegin(0));
            }
            ']' => {
                if let Some(inst) = i {
                        output.push(inst);
                        i = None;
                }
                let matching = loop_begins.pop().expect("Unmatched loop close!");
                output[matching] = Instruction::LoopBegin(output.len());
                output.push(Instruction::LoopEnd(matching));
            }
            _ => {}
        }
    }
    for instr in output.iter() {
        match instr {
            Instruction::LoopBegin(i) => {
                assert!(*i != 0);
            }
            _ => {}
        }
    }
    output
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
    interpret(&code, args.trace);
}
