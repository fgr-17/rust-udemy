// lifetime and memory:

#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

pub fn ownership() {
    let v = vec![1,2,3];
    
    // let v2 = v;
    // esto viola la seguridad de rust
    // porque puede llevar a una race condition
    // esta asignación fuerza una 'move semantic'
    // println!("{:?}", v);

    // let foo = |_v:Vec<i32>| ();
    // foo(v);
    //  si se lo paso a una fc tampoco me deja usarlo
    // println!("{:?}", v);

    // para valores primitivos no pasa lo mismo porque la variable es dueña del 
    // sector de memoria
    let u = 1;
    // si quisiera que u este en el heap
    // let u = Box::new(1);
    // full copy
    let _u2 = u;
    println!("u = {}", u);

    // como puedo arreglar eso para funciones?
    let print_vector = |x:Vec<i32>| -> Vec<i32>
    {
        println!("{:?}", x);
        // puedo devolver el valor despues de usarlo por ejemplo
        x
    };

    let vv = print_vector(v);
    println!("{:?}", vv);

}

pub fn borrowing() {

    let print_vector = |x:&Vec<i32>|
    {
        println!("{:?}", x);
    };

    let v = vec![1,2,3];
    print_vector(&v);  
    // si lo paso como referencia lo puedo volver a usar
    println!("{:?}", v);

    let mut a = 40;
    { // si b lo uso dentro de un scope diferente, cuando termina, libera a la var a
        let b = &mut a;

        *b += 2;
    }
    // a mi me dejo sin problemas hacer esto 
    println!("a = {}", a);

    let mut z = vec![3,2,1];

    for i in &z {
        println!("i = {}", i);
        // no me deja tomarlo (borrow) como inmutable y mutable a la vez
        // z.push(4);
    }

}

struct Person {
    name: String
}

impl Person 
{
    // fn get_ref_name(&self) -> &String
    // {
    //     &self.name
    // }
    // version explicita con el lifetime igual para todos:
    fn get_ref_name<'a>(&'a self) -> &'a String
    {
        &self.name
    }

}

// struct Company {
// es necesario especificar el lifetime del struct si tiene un puntero adentro
// de esta manera le digo que cuando el puntero deje de existir, la estructura
// también deje de ser válida
struct Company<'z>
{
    name: String,
    ceo: &'z Person
}


pub fn lifetime() {
    let boss = Person{name: String::from("Elon Musk")};
    let _tesla = Company{name: String::from("Tesla"), ceo:&boss};

    let mut z: &String;
    {
        let p = Person{name: String::from("Fede")};
        z = p.get_ref_name();
    }
    // me tira error la sgte linea: "borrowed value doesn't live long enough"
    // println!("z: {}", z);

}

struct Person2 <'a> {
    // missing lifetime specifier
    // name: &str
    name: &'a str
}

// se puede usar a o cualquier otra letra
impl<'b> Person2<'b>
{
    fn talk(&self)
    {
        println!("hola, soy {}", self.name);
    }
}

pub fn lifetime_in_struct_impl() 
{
    let person = Person2 {name: "Fede"};
    person.talk();
}

// clase generica que permite llevar una cuenta
// de las referencias de cada variable
use std::rc::Rc;

struct Person3
{
    // name: String
    name: Rc<String>
}


impl Person3
{
    // fn new(name: String) -> Person3
    fn new(name: Rc<String>) -> Person3
    {
        Person3 {name: name}
    }

    fn greet(&self) 
    {
        println!("hola, soy {}", self.name)
    }
}

fn rc_demo() 
{
    // let name = "Fede".to_string();
    // let person = Person3::new(name);
    let name = Rc::new("Fede".to_string());
    println!("name: {}, #strong pointers: {}", name, Rc::strong_count(&name));
    {
        let person = Person3::new(name.clone());
        println!("name: {}, #strong pointers: {}", name, Rc::strong_count(&name));
        person.greet();
    }
    // esto no se puede hacer porque movi al crear Person3
    // println!("{}", name);
    // al hacer clone si puedo volver a usarla
    println!("{}", name);
    println!("name: {}, #strong pointers: {}", name, Rc::strong_count(&name));

}

pub fn reference_counted_vars()
{
    rc_demo();
}