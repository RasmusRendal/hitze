use crate::parser::Instruction;

pub fn interpret(code: &Vec<Instruction>, memory: &mut [u8], trace: bool) {
    let mut pc: usize = 0;
    let mut mp: u16 = 0;
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
                panic!("Unsupported instruction");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_underflow() {
        let mut code = vec![];
        code.push(Instruction::PointerDecrement(1));
        code.push(Instruction::PointerIncrement(1));
        code.push(Instruction::Minus(1));
        code.push(Instruction::Plus(1));
        let mut memory = [0u8; 4];
        interpret(&code, &mut memory, false);
        assert_eq!(memory[0], 0);
    }
}
