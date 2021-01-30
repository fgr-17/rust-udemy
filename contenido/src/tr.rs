// traits

#![allow(dead_code)]
#![allow(unused_variables)]

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