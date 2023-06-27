use hitze::interpreter::interpret;
use hitze::parser::parse;
use hitze::runner;
mod common;
use common::assert_vecs_equal;

fn assert_memory_equal(code: &str) {
    let memory2 = runner::run(code);
    let mut memory1 = vec![0; u16::max_value() as usize + 1];
    interpret(&parse(code), memory1.as_mut_slice(), false);
    assert_vecs_equal(&memory1, &memory2);
}

#[test]
fn test_code() {
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
fn test_long() {
    // This takes too long to run for the parser, so the result is hardcoded
    let mut expected_vec = vec![0; u16::max_value() as usize + 1];
    expected_vec[1] = 202;
    let memory = runner::run(include_str!("../examples/long.bf"));
    assert_vecs_equal(&expected_vec, &memory);
}
