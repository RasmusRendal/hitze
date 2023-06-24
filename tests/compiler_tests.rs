use hitze::parser::parse;
use hitze::interpreter::interpret;
use hitze::compiler::compile;

fn assert_vecs_equal(vec1: &Vec<u8>, vec2 : &Vec<u8>) {
    assert_eq!(vec1.len(), vec2.len());
    for i in 0..vec1.len() {
        assert_eq!(vec1[i], vec2[i]);
    }
}

fn assert_memory_equal(code: &String) {
    let code = parse(code);
    let mut memory1 = vec![0; u16::max_value() as usize + 1];
    let mut memory2 = vec![0; u16::max_value() as usize + 1];
    interpret(&code, memory1.as_mut_slice(), false);
    let asm = compile(&code);
    asm.run(memory2.as_mut_slice());
    assert_vecs_equal(&memory1, &memory2);
}

#[test]
fn test_code() {
    assert_memory_equal(&"+++".to_string());
    assert_memory_equal(&"+++>+++".to_string());
    assert_memory_equal(&"--".to_string());
    assert_memory_equal(&"+>+<+".to_string());
    assert_memory_equal(&"+++[>+<-]".to_string());
}
