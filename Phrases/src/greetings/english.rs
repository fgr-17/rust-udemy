//! Este modulo tiene frases ejemplo
//!
//! # Examples
//! ```
//! let username = "John";
//! prinln!("{}, {}!,
//!     phrases::greetings::english::hello(),
//!     username);
//! ```
//!


// 3 slashes: documentacion md
/// Applies to code that follows it
/// In this case, it's our hello() function
pub fn hello() -> String { "hello".to_string() }
pub fn goodbye() -> String { "goodbye".to_string() }