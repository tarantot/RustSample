pub mod greetings {
    pub mod english;
    pub mod french {
        pub fn hello () -> String {"bonjour".to_string()}
        pub fn goodbye () -> String {"au revoir".to_string()}
    }
}

#[test]
fn english_greeting_correct () {
    assert_eq!("hello", greetings::english::hello()); // pass
    // assert_eq!("hellsgffsgo", greetings::english::hello()); // fail
}

#[test]
#[should_panic] // test is expected to fail
fn english_greeting_incorrect () {
    assert_eq!("hellsgffsgo", greetings::english::hello());
}

#[test]
#[ignore] // test will be ignored
fn english_greeting_ignore () {
    assert_eq!("whoareyou", greetings::english::hello());
}