#[cfg(test)]

mod tests {
    extern crate phrases;

    #[test]
    fn french_greeting_correct () {
        assert_eq!("bonjour", greetings::french::hello()); // pass
    }

    #[test]
    #[should_panic] // test is expected to fail
    fn french_greeting_incorrect () {
        assert_eq!("hellsgffsgo", greetings::french::hello());
    }

    #[test]
    #[ignore] // test will be ignored
    fn french_greeting_ignore () {
        assert_eq!("je ne mange pa sis jour", greetings::english::hello());
    }
}