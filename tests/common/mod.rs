pub fn assert_vecs_equal(expected: &Vec<u8>, actual: &Vec<u8>) {
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
