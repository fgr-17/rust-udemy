extern crate rand;
extern crate phrases;

use rand::Rng;

fn main()
{

    let mut rng = rand::thread_rng();
    let b:bool =  rng.gen();

    println!("boolean: {}", b);


    println!("English: {}, {}", 
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye(),
    );

    use phrases::greetings::french;

    println!("French: {}, {}",
        french::hello(),
        french::goodbye()
    );



}