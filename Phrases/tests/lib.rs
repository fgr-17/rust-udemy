#[cfg(test)]
mod tests
{

    extern crate phrases;
    #[test]
    // #[should_panic]
    // #[ignore]
    fn english_greeting_correct() 
    {
        assert_eq!("hello", phrases::greetings::english::hello());
    }
}
