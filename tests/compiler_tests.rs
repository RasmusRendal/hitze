use hitze::compiler::compile;
use hitze::interpreter::interpret;
use hitze::optimizer::optimize;
use hitze::parser::parse;

fn assert_vecs_equal(vec1: &Vec<u8>, vec2: &Vec<u8>) {
    assert_eq!(vec1.len(), vec2.len());
    let mut correct = true;
    for i in 0..vec1.len() {
        if vec1[i] != vec2[i] {
            println!("intmem[{}] = {}", i, vec1[i]);
            println!("jitmem[{}] = {}", i, vec2[i]);
            correct = false;
        }
    }
    assert!(correct);
}

fn assert_memory_equal(code: &str) {
    let mut code = parse(code);
    let mut memory1 = vec![0; u16::max_value() as usize + 1];
    let mut memory2 = vec![0; u16::max_value() as usize + 1];
    interpret(&code, memory1.as_mut_slice(), false);
    optimize(&mut code);
    let asm = compile(&code);
    asm.run(memory2.as_mut_slice());
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
    // Hello World
    assert_memory_equal("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>>---++++++++++>><-<+++-------------->>+>++");
    // Hello World (Golfed)
    assert_memory_equal("+[-->-[>>+>-----<<]<--<---]>->>>+>>+++[>]<<<<+++------<<->>>>+");
    assert_memory_equal("++>+++++<[->+<]");
    assert_memory_equal(include_str!("../examples/long.bf"));
}
