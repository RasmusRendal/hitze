use crate::parser::Instruction;

pub fn optimize(code: &mut Vec<Instruction>) {
    // You can take the man out of C++, but you can't take C++ out of the man
    let mut i: usize = 2;
    while i < code.len() {
        match unsafe { code.get_unchecked(i) } {
            Instruction::LoopEnd(_) => {
                if let Instruction::Add(_) = unsafe { code.get_unchecked(i - 1) } {
                    if let Instruction::LoopBegin(_) = unsafe { code.get_unchecked(i - 2) } {
                        unsafe {
                            // Replace with NOP because removing elements from a vector is a O(n)
                            // operation
                            *code.get_unchecked_mut(i - 2) = Instruction::ResetByte;
                            *code.get_unchecked_mut(i - 1) = Instruction::Nop;
                            *code.get_unchecked_mut(i - 0) = Instruction::Nop;
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
}
