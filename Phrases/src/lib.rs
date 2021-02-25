// modulo: grupo de varias funcionalidades

pub mod greetings
{
    // pub mod english
    // {
    //     pub fn hello() -> String { "hello".to_string() }
    //     pub fn goodbye() -> String { "goodbye".to_string() }
    // }
    // pub mod french
    // {
    //     pub fn hello() -> String { "bonjour".to_string() }
    //     pub fn goodbye() -> String { "au revoir".to_string() }
    // }
    

    pub mod english;
    pub mod french;
}


// #[test]
// // #[should_panic]
// // #[ignore]
// fn english_greeting_correct() 
// {
//     assert_eq!("hello", greetings::english::hello());
// }