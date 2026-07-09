use climbing_stairs::climb_stairs;

#[test]
fn examples() {
    assert_eq!(climb_stairs(2), 2);
    assert_eq!(climb_stairs(3), 3);
}

#[test]
fn edge_cases() {
    assert_eq!(climb_stairs(1), 1);  // only one way
    assert_eq!(climb_stairs(4), 5);
    assert_eq!(climb_stairs(5), 8);  // Fibonacci-shaped
}
