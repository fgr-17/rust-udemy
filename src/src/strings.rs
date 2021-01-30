// strings and characters

#![allow(dead_code)]


pub fn strings() {

    // utf-8
    let s:&'static str = "hola";    // &str es un string slice
    // no se puede modificar ni acceder por letra
    // s = "abc";
    // let h = s[0];

    // puedo iterarlo:
    for c in s.chars() {
        println!("{}", c);
    }
    for c in s.chars().rev() {
        println
        !("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    // Strings viven en heap
    let mut letters = String::new();
    let mut a = 'a' as u8;

    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);

    // conversion &str <-> String
    let _u:&str = &letters;

    // concatenation
    // String + str
    // convierto el segundo a &str
    // let z = &letters + &letters;
    // println!("{}", letters);

    // creo un String para que se pueda manipular
    let mut abc = "hola".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc);
    println!("{}", abc.replace("ola", "chau"));

}

pub fn string_formatting() {

    let name = "Dmitri";
    let greeting = format!("Hi {}", name);
    println!("{}", greeting);


    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{}, {}!", hello, rust);
    println!("{}", hello_rust);

    // argumentos con posicion
    let run = "run";
    let forest = "forest";
    let rfr = format!("{0},{1},{0}!", run, forest);
    println!("{}", rfr);

    // argumentos con nombre
    let info = format!( "the name's {last}. {first} {last}.", first = "james", last = "bond");
    println!("{}", info);

    // todo mezclado
    let mixed = format!("{1} {} {0} {data}", "alpha", "beta", data = "delta");
    println!("{}", mixed);
    // no se puede tener argumentos sin usar
}

use rand::Rng;
use std::io::stdin;

pub fn number_guessing_game() {

    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Ingresar numero: ");
        let mut buffer = String::new();

        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100 {
                            println!("fuera de rango");
                        }
                        else if guess < number {
                            println!("numero bajo");
                        }
                        else if guess > number {
                            println!("numero alto");
                        }
                        else {
                            println!("adivinaste");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("no se pudo leer la entrada {}", e);
                    }
                }

            },
            Err(_) => continue,
        }


    }

}