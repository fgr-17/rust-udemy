#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::mem;
use std::io::stdin;


struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap() {

    let p1 = origin();
    let p2 = Box::new(origin());

    println!("p1 ocupa {} bytes", mem::size_of_val(&p1));
    println!("p2 ocupa {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("{}", p3.x);

        
}

pub fn if_statement() {

    let temp = 35;

    if temp > 30 {
        println!("hace la caleur");
    }
    else if temp < 10 {
        println!("hace un chiflete");
    }
    else {
        println!("esta todo bien");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};


    println!("today is {}", day)

}

pub fn while_and_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;
        if x == 64 {continue;}

        println!("x = {}", x);

    }
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10 { break;}
    }

}


pub fn for_loop() {

    for x in 1..11 {
        if x == 3 {continue;}
        if x == 8 { break;}

        println!("{}", x);
    }

    for (i, y) in (30..41).enumerate() {
        println!("{}: {}", i, y);
    }

}


pub fn match_statement () {
    let country_code = 43214;

    // obliga a manejar todos los casos
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown",
        _  => "invalid"
    };

    println!("the country with code {} is {}", country_code, country);


    let x = false;

    let s = match x {
        true => "yes",
        false => "no"
    };

    println!("{}: {}", x, s);

}

enum State {
    Locked, 
    Failed,
    Unlocked
}
pub fn combination_lock() {

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();


    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }

            }
            State::Failed => {
                println!("Failed");
                entry.clear();
                state = State::Locked;
                continue;

            }
            State::Unlocked => {
                println!("unlocked");
                return;
            }
        }


    }


}