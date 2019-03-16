extern crate phrases;

#[test]
fn test_greetings_hello() {
    assert_eq!(phrases::english::greetings::hello(), "Hello!".to_string());
}

#[test]
fn test_farewells_goodbye() {
    assert_eq!(phrases::english::farewells::goodbye(), "Goodbye!".to_string());
}