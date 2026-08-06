use qubit_utils::is_blank;

#[test]
fn test_is_blank_whitespace_string_returns_true() {
    assert!(is_blank(" \t\n"));
}

#[test]
fn test_is_blank_non_whitespace_string_returns_false() {
    assert!(!is_blank("value"));
}
