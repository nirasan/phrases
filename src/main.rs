extern crate phrases;

fn main() {
    assert_eq!(phrases::english::hello(), "Hello!".to_string());
    assert_eq!(phrases::english::greetings::hello(), "Hello!".to_string());
    assert_eq!(phrases::english::farewells::goodbye(), "Goodbye!".to_string());
}