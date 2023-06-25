use crate::parser::Instruction;

pub fn optimize(code: &mut Vec<Instruction>) {
    let mut i: usize = 2;
    while i < code.len() {
        match code[i] {
            Instruction::LoopEnd(_) => {
                if let Instruction::Add(_) = code[i - 1] {
                    if let Instruction::LoopBegin(_) = code[i - 2] {
                        code.remove(i - 2);
                        code.remove(i - 2);
                        code.remove(i - 2);
                        code.insert(i - 2, Instruction::ResetByte);
                        i = i - 1;
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
        assert_eq!(code.len(), 3);
    }
}
