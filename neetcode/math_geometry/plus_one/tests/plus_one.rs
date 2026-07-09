use plus_one::plus_one;

#[test]
fn examples() {
    assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
}

#[test]
fn edge_cases() {
    assert_eq!(plus_one(vec![9]), vec![1, 0]);          // single-digit carry
    assert_eq!(plus_one(vec![9, 9, 9]), vec![1, 0, 0, 0]); // carry grows the vec
    assert_eq!(plus_one(vec![1, 9]), vec![2, 0]);       // carry stops midway
}
