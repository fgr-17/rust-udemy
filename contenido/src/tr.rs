// traits

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_imports)]

trait Animal {
    // funcion generadora: devuelve self con mayus
    fn create(name:&'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self)
    {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

impl Animal for Human
{
    fn create(name:&'static str) -> Human
    {
        Human{name: name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    // si no está implementada se usa la original del trait
    fn talk(&self) 
    {
        println!("{} says hello!", self.name());
    }
}

struct Cat {
    name: &'static str
}

impl Animal for Cat
{

    fn create(name:&'static str) -> Cat
    {
        Cat{name: name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    // si no está implementada se usa la original del trait
    fn talk(&self) 
    {
        println!("{} says miau!", self.name());
    }
}

trait Summable <T>{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {

    fn sum(&self) -> i32 {
        let mut result:i32 = 0;

        for x in self { result += *x};
        result
    }
}

pub fn traits() {
    // todas formas equivalentes
    // let h = Human{name: "Juan"};
    // let h = Human::create("Juan");
    let h:Human = Animal::create("Juan");
    h.talk();

    let c = Cat{name: "cati"};
    c.talk();

    // le agrego una función a un tipo que ya existe
    let a = vec![1,2,3];
    println!("sum vec = {}", a.sum());
}

use std::fmt::Debug;

// hago que implementen el Debug trait
#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 
    {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area (&self) -> f64 
    {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// funcion que recibe un trait como parametro
// 1) uso + para sumar mas traits
// fn print_info(shape: impl Shape + Debug) 
// 2) lo pongo como generic type, la facilidad es que
// puedo agregar mas parametros (shape:T, shape2: T, etc...)
// fn print_info<T: Shape + Debug> (shape: T)
// 3) lo pongo como generic type y especifico al final
fn print_info<T> (shape: T)
  where T: Shape + Debug
{
    println!("{:?}", shape);
    println!("the shape area is: {}", shape.area());
}

pub fn trait_parameters() 
{
    let c = Circle{radius: 1.0};
    print_info(c);

    let s = Square{side: 1.0};
    print_info(s);
}

struct Person 
{
    name: String
}

impl Person
{
    // fn new(name: &str) -> Person
    // {
    //     Person {name: name.to_string()}
    // }

    // diferentes sintaxis
    // fn new<S: Into<String>>(name: S) -> Person
    fn new<S>(name: S) -> Person
        where S: Into<String>
    {
        Person {name: name.into()}
    }
}

// me deja pasar parametros q se auto convierten
pub fn into() 
{
    let john = Person::new("John");

    let name: String = "Jane".to_string();
    // convierto el String a una referencia str
    // let jane = Person::new(name.as_ref());
    // con el into no hace falta el as_ref
    let jane = Person::new(name);
}

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature{name: name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

pub fn drop_trait() 
{
    let goblin = Creature::new("Fede");
    println!("Fin de juego");
    // drop no se puede llamar como metodo, es como un destructor
    // se puede llamar como funcion
    drop(goblin);

    let mut clever: Creature; 
    {
        let goblin2 = Creature::new("bicho");
        println!("siga el baile");
        clever = goblin2;
        println!("endofscope");
    }

}

// los operadores son traits
use std::ops::{Add, AddAssign, Neg};


#[derive(Debug, PartialEq, Eq)]
struct Complex <T>
{
    re: T,
    im: T
}

impl<T> Complex<T> 
{
    fn new(re: T, im: T) -> Complex<T> 
    {
        // se pone el :: porque sino trata al < como menor
        Complex::<T> {re, im}
    }
}

// Implementación para <i32>
// impl Add for Complex<i32> {
//     // el tipo que devuelve
//     type Output = Complex<i32>;

//     // Self con mayus se refiere el tipo de dato
//     fn add(self, rhs: Self) -> Self::Output {
//         Complex {
//             re: self.re + rhs.re,
//             im: self.im + rhs.im
//         }
//     }
// }

// agrego el where porque el tipo T tiene que soportar
// si o si el operador +
impl<T> Add for Complex<T> 
  where T: Add<Output = T>
{
    // el tipo que devuelve
    type Output = Complex<T>;

    // Self con mayus se refiere el tipo de dato
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T>
  where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }

}

impl<T> Neg for Complex<T>
  where T: Neg<Output=T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}


// operador Eq
// partial eq
// full eq: x = x
// NAN != NAN -> float no soporta con full eq

// use std::cmp::{PartialEq};

// impl<T> PartialEq for Complex<T>
//  where T: PartialEq
// {
//     fn eq(&self, rhs: &Self) -> bool {
//         self.re == rhs.re && self.im == rhs.im
//     }

// }


// // se puede agregar full equality si el tipo lo soporta
// // no hace falta implementarlo porque se deduce ?
// impl<T: Eq> Eq for Complex<T> where T: Eq {}

// Eq y PartialEq estan comentados porque también se puede usar derive
// al definir el struct y hace una comparación elemento a elemento

pub fn operator_overloading() 
{
    // let mut a = Complex::new(1, 2);
    // let mut b = Complex::new(3, 4);
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("a + b ={:?}", a+b);

    let mut c = Complex::new(1.0, 2.0);
    let d = Complex::new(1.0, 1.0);
    c += d;
    println!("c = {:?}", -c);

    let e = Complex::new(1.0, 2.0);
    let f = Complex::new(1.0, 1.0);
    // println!("e == f? -> {}", e == f);
    // el desigual no hace falta implementarlo porque se 
    // deduce como el contrario del igual
    println!("e == f? -> {}", e != f);

}

use std::mem;

trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32: {}", *self)
    }
}

impl Printable for String
{
    fn format(&self) -> String
    {
        format!("String: {}", *self)
    }
}

fn print<T: Printable>(z: T)
{
    println!("{}", z.format());
} // monomorphisation

pub fn static_dispatch()
{
    let a = 123;
    let b = "hola".to_string();

    // println!("a: {}", a.format());
    // println!("b: {}", b.format());
    // es static dispatch porque la decision de cual print llamar
    // de define en tiempo de compilación
    print(a);
    print(b);
}
