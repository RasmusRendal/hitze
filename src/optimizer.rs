use crate::parser::Instruction;

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
                        code[i - 1] = Instruction::Nop;
                        code[i - 0] = Instruction::Nop;
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
                                            if m == -1 && l >= 1 {
                                                code[i - 5] = Instruction::AddRel(k, l as u8);
                                                code[i - 4] = Instruction::ResetByte;
                                                code[i - 3] = Instruction::Nop;
                                                code[i - 2] = Instruction::Nop;
                                                code[i - 1] = Instruction::Nop;
                                                code[i - 0] = Instruction::Nop;
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
        assert_eq!(code[2], Instruction::Nop);
        assert_eq!(code[3], Instruction::Nop);
        assert_eq!(code[4], Instruction::Add(1));
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
            assert_eq!(code[i], Instruction::Nop);
        }
        assert_eq!(code[code.len() - 1], Instruction::Add(8));
    }
}
