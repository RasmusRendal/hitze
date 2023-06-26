#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    MovePointer(isize),
    Add(i8),
    ResetByte,
    Output(usize),
    Input(usize),
    LoopBegin(usize),
    LoopEnd(usize),
    Nop,
}

#[inline(always)]
fn append_current(output: &mut Vec<Instruction>, current: Option<Instruction>) {
    if let Some(i) = current {
        match i {
            Instruction::MovePointer(0) => {}
            Instruction::Add(0) => {}
            _ => {
                output.push(i);
            }
        }
    }
}

pub fn parse(code: &str) -> Vec<Instruction> {
    let mut output: Vec<Instruction> = vec![];
    let mut i: Option<Instruction> = None;
    let mut loop_begins: Vec<usize> = vec![];
    for c in code.chars() {
        match c {
            '>' => match i {
                Some(Instruction::MovePointer(j)) => i = Some(Instruction::MovePointer(j + 1)),
                None => i = Some(Instruction::MovePointer(1)),
                _ => {
                    append_current(&mut output, i);
                    i = Some(Instruction::MovePointer(1));
                }
            },
            '<' => match i {
                Some(Instruction::MovePointer(j)) => i = Some(Instruction::MovePointer(j - 1)),
                None => i = Some(Instruction::MovePointer(-1)),
                _ => {
                    append_current(&mut output, i);
                    i = Some(Instruction::MovePointer(-1));
                }
            },
            '+' => match i {
                Some(Instruction::Add(j)) => i = Some(Instruction::Add(j + 1)),
                None => i = Some(Instruction::Add(1)),
                _ => {
                    append_current(&mut output, i);
                    i = Some(Instruction::Add(1));
                }
            },
            '-' => match i {
                Some(Instruction::Add(j)) => i = Some(Instruction::Add(j - 1)),
                None => i = Some(Instruction::Add(-1)),
                _ => {
                    append_current(&mut output, i);
                    i = Some(Instruction::Add(-1));
                }
            },
            '.' => match i {
                Some(Instruction::Output(j)) => i = Some(Instruction::Output(j + 1)),
                None => i = Some(Instruction::Output(1)),
                _ => {
                    append_current(&mut output, i);
                    i = Some(Instruction::Output(1));
                }
            },
            ',' => match i {
                Some(Instruction::Input(j)) => i = Some(Instruction::Input(j + 1)),
                None => i = Some(Instruction::Input(1)),
                _ => {
                    append_current(&mut output, i);
                    i = Some(Instruction::Input(1));
                }
            },
            '[' => {
                append_current(&mut output, i);
                i = None;
                loop_begins.push(output.len());
                output.push(Instruction::LoopBegin(0));
            }
            ']' => {
                append_current(&mut output, i);
                i = None;
                let matching = loop_begins.pop().expect("Unmatched loop close!");
                output[matching] = Instruction::LoopBegin(output.len());
                output.push(Instruction::LoopEnd(matching));
            }
            _ => {}
        }
    }
    append_current(&mut output, i);
    for instr in output.iter() {
        if let Instruction::LoopBegin(i) = instr {
            assert!(*i != 0);
        }
    }
    output
}
