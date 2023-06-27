use crate::compiler::compile;
use crate::parser::Instruction;

pub fn compile_inner_loops(code: &mut [Instruction], compile_depth: usize) {
    let mut i = 0;
    let mut depth: usize = 0;
    while i < code.len() {
        if let Instruction::LoopBegin(j) = code[i] {
            depth += 1;
            if depth == compile_depth {
                let program = compile(&code[i..j + 1]);
                code[i] = Instruction::Call(program);
                code[i + 1] = Instruction::Nop(j - i);
                for k in i + 2..j + 1 {
                    code[k] = Instruction::Nop(1);
                }
                i = j;
                depth -= 1;
            }
        } else if let Instruction::LoopEnd(_) = code[i] {
            depth -= 1;
        }
        i += 1;
    }
}

pub fn optimize(code: &mut [Instruction]) {
    let mut i: usize = 2;
    while i < code.len() {
        match code[i] {
            Instruction::LoopEnd(_) => {
                if let Instruction::Add(_) = code[i - 1] {
                    if let Instruction::LoopBegin(_) = code[i - 2] {
                        // Replace with NOP because removing elements from a vector is a O(n)
                        // operation
                        code[i - 2] = Instruction::ResetByte;
                        code[i - 1] = Instruction::Nop(2);
                        code[i - 0] = Instruction::Nop(1);
                    }
                }
                if i > 4 {
                    // https://github.com/rust-lang/rust/issues/53667
                    if let Instruction::MovePointer(j) = code[i - 1] {
                        if let Instruction::Add(l) = code[i - 2] {
                            if let Instruction::MovePointer(k) = code[i - 3] {
                                if j == -k {
                                    if let Instruction::Add(m) = code[i - 4] {
                                        if let Instruction::LoopBegin(_) = code[i - 5] {
                                            if m < 0 {
                                                code[i - 5] = Instruction::AddRel(k, l as i8);
                                                code[i - 4] = Instruction::ResetByte;
                                                code[i - 3] = Instruction::Nop(4);
                                                code[i - 2] = Instruction::Nop(3);
                                                code[i - 1] = Instruction::Nop(2);
                                                code[i - 0] = Instruction::Nop(1);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn test_optimize_reset() {
        let mut code = vec![];
        code.push(Instruction::Add(1));
        code.push(Instruction::LoopBegin(3));
        code.push(Instruction::Add(1));
        code.push(Instruction::LoopEnd(1));
        code.push(Instruction::Add(1));
        optimize(&mut code);
        assert_eq!(code[0], Instruction::Add(1));
        assert_eq!(code[1], Instruction::ResetByte);
        assert_eq!(code[2], Instruction::Nop(2));
        assert_eq!(code[3], Instruction::Nop(1));
        assert_eq!(code[4], Instruction::Add(1));
    }

    #[test]
    fn test_insert_compiled() {
        let mut code = parse("+++[-]++");
        compile_inner_loops(&mut code, 1);
        println!("{:?}", code);
        assert_eq!(code[0], Instruction::Add(3));
        if let Instruction::Call(_) = code[1] {
        } else {
            assert!(false);
        }
        assert_eq!(code[2], Instruction::Nop(2));
        assert_eq!(code[3], Instruction::Nop(1));
        assert_eq!(code[4], Instruction::Add(2));
    }

    #[test]
    fn test_optimize_addrel() {
        let mut code = vec![];
        code.push(Instruction::Add(4)); // +
        code.push(Instruction::LoopBegin(6));
        code.push(Instruction::Add(-1));
        code.push(Instruction::MovePointer(1));
        code.push(Instruction::Add(1));
        code.push(Instruction::MovePointer(-1));
        code.push(Instruction::LoopEnd(1));
        code.push(Instruction::Add(8));
        optimize(&mut code);
        assert_eq!(code[0], Instruction::Add(4));
        assert_eq!(code[1], Instruction::AddRel(1, 1));
        assert_eq!(code[2], Instruction::ResetByte);
        for i in 3..code.len() - 1 {
            assert_eq!(code[i], Instruction::Nop(code.len() - i - 1));
        }
        assert_eq!(code[code.len() - 1], Instruction::Add(8));
    }
}
