pub fn hello() -> String {
    "Hello!".to_string()
}

#[test]
fn test_hello() {
    assert_eq!(hello(), "Hello!".to_string());
}