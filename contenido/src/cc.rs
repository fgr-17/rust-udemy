// Concurrency

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]


use std::thread;
use std::time;

pub fn spawning_joining_threads() 
{

    // disparo un nuevo thread 
    // como argumento recibe un closure
    let handle = thread::spawn(|| {
        for _ in 1.. 10 
        {
            print!("+");
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    for _ in 1.. 10 
    {
        print!("-");
        thread::sleep(time::Duration::from_millis(300));
    }

    handle.join();
}
