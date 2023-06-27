use hitze::interpreter::interpret;
use hitze::optimizer::optimize;
use hitze::parser::parse;
use hitze::util::assert_arrays_equal;

#[test]
fn test_parser() {
    let mut expected_vec = vec![0; u16::max_value() as usize + 1];
    expected_vec[2] = 6;
    expected_vec[3] = 6;
    let mut memory = vec![0; u16::max_value() as usize + 1];
    let mut code = parse("+++[->++[->+>+<<]<]");
    optimize(&mut code);
    interpret(&code, memory.as_mut_slice(), 0, false);
    assert_arrays_equal(&expected_vec, &memory);
}
