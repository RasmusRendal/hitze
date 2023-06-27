pub fn assert_arrays_equal(expected: &[u8], actual: &[u8]) {
    assert_eq!(expected.len(), actual.len());
    let mut correct = true;
    for i in 0..expected.len() {
        if expected[i] != actual[i] {
            println!("expected[{}] = {}", i, expected[i]);
            println!("actual[{}] = {}", i, actual[i]);
            correct = false;
        }
    }
    assert!(correct);
}

pub fn print_memory(memory: &[u8]) {
    for (i, item) in memory.iter().enumerate() {
        if *item != 0 {
            println!("mem[{}] = {}", i, item);
        }
    }
}
