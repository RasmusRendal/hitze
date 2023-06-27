use hitze::compiler::compile;
use hitze::interpreter::interpret;
use hitze::optimizer::optimize;
use hitze::parser::{parse, Instruction};
use hitze::runner;
use hitze::util::assert_arrays_equal;

fn compare_interpreter_run(code: &str, compile_depth: usize) {
    let memory2 = runner::run(code, compile_depth);
    let mut memory1 = vec![0; u16::max_value() as usize + 1];
    interpret(&parse(code), memory1.as_mut_slice(), 0, false);
    assert_arrays_equal(&memory1, &memory2);
}

fn compare_interpreter_jit(code: &str) {
    let mut memory1 = vec![0; u16::max_value() as usize + 1];
    let mut memory2 = vec![0; u16::max_value() as usize + 1];
    interpret(&parse(code), memory1.as_mut_slice(), 0, false);
    let mut code = parse(code);
    optimize(&mut code);
    let program = compile(&code);
    program.run(&mut memory2, 0);
    assert_arrays_equal(&memory1, &memory2);
}

fn assert_memory_equal(code: &str) {
    compare_interpreter_jit(code);
    for compile_depth in 0..5 {
        compare_interpreter_run(code, compile_depth);
    }
}

#[test]
fn test_code() {
    assert_memory_equal("+<+>+<+>+<+>+");
    assert_memory_equal("+++");
    assert_memory_equal("+++>+++");
    assert_memory_equal("--");
    assert_memory_equal("+>+<+");
    assert_memory_equal("+++[>+<-]");
    assert_memory_equal("<+");
    assert_memory_equal("<>+");
    assert_memory_equal("+++[->++[->+>+<<]<]");
    // Hello World
    assert_memory_equal("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>>---++++++++++>><-<+++-------------->>+>++");
    // Hello World (Golfed)
    assert_memory_equal("+[-->-[>>+>-----<<]<--<---]>->>>+>>+++[>]<<<<+++------<<->>>>+");
    // Addition
    assert_memory_equal("++>+++++<[->+<]");
    // Multiplication
    assert_memory_equal("++>+++++<[->+++<]");
    // Multiplication (negative)
    assert_memory_equal("++>+++++<[->---<]");
}

#[test]
fn test_specific() {
    let mut initial_memory = vec![0; u16::max_value() as usize + 1];
    let code = [
        Instruction::LoopBegin(6),
        Instruction::MovePointer(2),
        Instruction::Add(1),
        Instruction::MovePointer(1),
        Instruction::Add(-5),
        Instruction::MovePointer(-2),
        Instruction::LoopEnd(0),
    ];
    initial_memory[2] = 113;
    initial_memory[3] = 32;
    initial_memory[4] = 87;
    initial_memory[65534] = 251;
    initial_memory[65535] = 195;
    let initial_mp = 65535;
    let mut interpreter_memory = initial_memory.to_vec();
    let mut jit_memory = initial_memory.to_vec();
    interpret(&code, &mut interpreter_memory, initial_mp, false);
    let program = compile(&code);
    program.run(&mut jit_memory, initial_mp as usize);
    assert_arrays_equal(&interpreter_memory, &jit_memory);
}

#[test]
fn test_long() {
    // This takes too long to run for the parser, so the result is hardcoded
    let mut expected_vec = vec![0; u16::max_value() as usize + 1];
    expected_vec[1] = 202;
    let memory = runner::run(
        include_str!("../examples/long.bf"),
        runner::DEFAULT_COMPILE_DEPTH,
    );
    assert_arrays_equal(&expected_vec, &memory);
}
