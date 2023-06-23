#[derive(Debug)]
pub enum Instruction {
    PointerIncrement(usize),
    PointerDecrement(usize),
    Plus(u8),
    Minus(u8),
    Output(usize),
    Input(usize),
    LoopBegin(usize),
    LoopEnd(usize),
}

pub fn parse(code : &String) -> Vec<Instruction> {
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

