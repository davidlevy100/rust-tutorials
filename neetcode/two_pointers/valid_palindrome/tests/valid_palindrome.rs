use valid_palindrome::is_palindrome;

#[test]
fn examples() {
    assert!(is_palindrome("A man, a plan, a canal: Panama".into()));
    assert!(!is_palindrome("race a car".into()));
}

#[test]
fn edge_cases() {
    assert!(is_palindrome(" ".into()));      // empty after stripping punctuation
    assert!(is_palindrome("".into()));       // truly empty
    assert!(!is_palindrome("0P".into()));    // '0' vs 'p' — alphanumeric, not equal
    assert!(is_palindrome("ab_a".into()));   // '_' is ignored -> "aba"
    assert!(is_palindrome("a".into()));      // single char
}
