use valid_parentheses::is_valid;

#[test]
fn examples() {
    assert!(is_valid("()".into()));
    assert!(is_valid("()[]{}".into()));
    assert!(!is_valid("(]".into()));
    assert!(!is_valid("([)]".into())); // correct counts, wrong nesting
}

#[test]
fn edge_cases() {
    assert!(is_valid("".into()));       // empty is valid
    assert!(is_valid("{[]}".into()));   // nested
    assert!(!is_valid("(".into()));     // unclosed opener
    assert!(!is_valid(")".into()));     // closer with no opener
    assert!(!is_valid("){".into()));
}
