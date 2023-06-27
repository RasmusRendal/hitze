use crate::parser::Instruction;
use crate::util::{assert_arrays_equal, print_memory};

pub fn interpret(code: &[Instruction], memory: &mut [u8], mut mp: u16, trace: bool) {
    let mut pc: usize = 0;
    while pc < code.len() {
        if trace {
            println!("Executing instruction {:?}", code[pc]);
            println!("pc: {}, mp: {}", pc, mp);
        }
        match &code[pc] {
            Instruction::MovePointer(i) => {
                if *i < 0 {
                    mp = mp.wrapping_sub(isize::abs(*i) as u16);
                } else {
                    mp = mp.wrapping_add(*i as u16);
                }
                pc += 1;
            }
            Instruction::Add(i) => {
                if *i < 0 {
                    memory[mp as usize] = memory[mp as usize].wrapping_sub(i8::abs(*i) as u8);
                } else {
                    memory[mp as usize] = memory[mp as usize].wrapping_add(*i as u8);
                }
                pc += 1;
            }
            Instruction::ResetByte => {
                memory[mp as usize] = 0;
                pc += 1;
            }
            Instruction::Output(i) => {
                for _ in 0..*i {
                    let c: char = memory[mp as usize].into();
                    print!("{}", c);
                }
                pc += 1;
            }
            Instruction::Input(_) => {
                panic!("Unsupported instruction");
            }
            Instruction::LoopBegin(i) => {
                if memory[mp as usize] == 0 {
                    pc = *i;
                } else {
                    pc += 1;
                }
            }
            Instruction::LoopEnd(i) => {
                if memory[mp as usize] != 0 {
                    pc = *i;
                } else {
                    pc += 1;
                }
            }
            Instruction::Nop(i) => {
                assert_eq!(code[i + pc - 1], Instruction::Nop(1));
                pc += i;
            }
            Instruction::AddRel(rel, mul) => {
                let loc = ((mp as isize) + rel) as u16;
                memory[loc as usize] =
                    memory[loc as usize].wrapping_add(((memory[mp as usize] as i8) * mul) as u8);
                pc += 1;
            }
            Instruction::Call(f) => {
                /*
                println!("Debug stuff:");
                println!("mp = {}", mp);
                println!("{:?}", f.code);
                print_memory(&memory);
                let mut v2 = memory.to_vec();
                interpret(&f.code, &mut v2, mp, false);
                */
                mp = f.run(memory, mp as usize) as u16;
                //assert_arrays_equal(&v2, &memory);
                pc += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underflow() {
        let mut code = vec![];
        code.push(Instruction::MovePointer(-1));
        code.push(Instruction::MovePointer(1));
        code.push(Instruction::Add(1));
        code.push(Instruction::Add(-1));
        let mut memory = [0u8; 4];
        interpret(&code, &mut memory, 0, false);
        assert_eq!(memory[0], 0);
    }
}
