use valid_anagram::is_anagram;

#[test]
fn examples() {
    assert!(is_anagram("anagram".into(), "nagaram".into()));
    assert!(!is_anagram("rat".into(), "car".into()));
}

#[test]
fn edge_cases() {
    assert!(is_anagram("".into(), "".into()));        // both empty
    assert!(!is_anagram("a".into(), "ab".into()));    // different lengths
    assert!(!is_anagram("aacc".into(), "ccac".into())); // same letters, wrong counts
    assert!(is_anagram("aabbcc".into(), "baccab".into()));
}
