#![allow(dead_code)]
#![allow(unused_imports)]

mod sh;
mod ds;
mod pm;
mod sc;
mod strings;
mod fns;
mod tr;
mod lam;

use std::mem;

use lam::{lifetime, mutex_demo, reference_counted_vars};
use tr::dynamic_dispatch;

fn datatypes()
{

    println!("-------- datatypes --------\n");
    let a: u8 = 123;  // variables inmutables x def
    println!("a = {}", a);

    let mut b: i8 = 0; // variables normales

    println!("b = {}", b);
    b = 113;
    println!("b = {}", b);

    let c = 123456789;

    println!("c = {}, toma {} bytes", c, mem::size_of_val(&c));

    // usize isize
    let z: isize = 123;
    let size_of_z:usize = mem::size_of_val(&z);
    
    println!("z = {}, {}-bit OS", z, size_of_z * 8);

    let d: char = 'x';


    // char
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // f32 f64
    let e: f32 = 2.5;
    println!("e ={} is a f32, size = {} bytes", e, mem::size_of_val(&e));

    let f: f64 = 2.5352;
    println!("f ={} is a f64, size = {} bytes", f, mem::size_of_val(&f));

    let g: bool = true;
    println!("g ={} is a bool, size = {} bytes", g, mem::size_of_val(&g));


}

fn operators() 
{
    let mut a = 1+2*3;
    a+=1;
    println!("{}", a);

    let a3 = i32::pow(a, 3);
    println!("a^3 = {}", a3);
    

}

const CONSTANTE:i8 = 25;

static mut VAR_GLOBAL:i32 = 14;

fn main()
{
    // datatypes();
    // operators();
    // println!("constante {}", CONSTANTE);
    // unsafe {
    //     println!("var global {}", VAR_GLOBAL);
    // }

    //sh::stack_and_heap();
    // sh::if_statement();
    // sh::while_and_loop();

    //ds::structures();
    // ds::colors();
    // ds::slices();
    // ds::tuples();
    // pm::pattern_matching();
    // ds::generics();

    // sc::vectors();
    // sc::hash_map();
    // sc::hash_set();
    // sc::iterators();
    // strings::strings();
    // strings::string_formatting();
    // strings::number_guessing_game();
 
    // fns::functions();
    // fns::methods();
    // fns::closures();
    // fns::higher_order_fn();

    // tr::traits();
    // tr::trait_parameters();
    // tr::into();
    // tr::drop_trait();
    // tr::operator_overloading();
    // tr::static_dispatch();
    // tr::dynamic_dispatch();
    // tr::vector_diff_objects();

    // lam::ownership();
    // lam::borrowing();
    // lam::lifetime();
    // lam::lifetime_in_struct_impl();
    // lam::reference_counted_vars();
    // lam::atomic_rc();
    lam::mutex_demo();
}