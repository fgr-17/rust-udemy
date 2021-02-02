// functions

#![allow(dead_code)]
#![allow(unused_variables)]

fn print_value (x: i32) {
    println!("value = {}", x);
}

fn increase (x: &mut i32) {
    *x += 1;
}

fn product(x: i32, y: i32) -> i32 
{
    x * y
}
pub fn functions() {

    let mut z = 1;
    print_value(33);

    increase(&mut z);
    print_value(z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    print_value(p);
}

struct Point
{
    x: f64,
    y: f64
}

struct Line
{
    a: Point,
    b: Point
}

impl Line
{
    fn len(&self) -> f64
    {
        let dx = self.a.x - self.b.x;
        let dy = self.a.y - self.b.y;
        (dx*dx + dy*dy).sqrt()
    }
}

pub fn methods()
{
    let p0 = Point{x: 3.0, y: 4.0};
    let p1 = Point{x: 5.0, y: 10.0};
    let l = Line {a: p0, b: p1};

    println!("len = {}", l.len());

}

fn say_hello() 
{
    println!("hola");
}

pub fn closures() 
{
    let s_h = say_hello;
    s_h();

    // closure: tipo lambda
    let plus_one = |x:i32| -> i32 {x + 1};

    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let mut two = 2;
    {
        let plus_two = |x|
        {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }
        
    // trato de tomar la variable anterior y no me deja
    // let borrow_two = &mut two;
    // porque el closure toma la variable y no la suelta hasta que se destruya, es decir, 
    // cuando termina la función. Esto se puede salvar haciendo un bloque nuevo con la fc
    let borrow_two = &mut two;
    println!("{}", borrow_two);


    // pasaje de valores
    // T: por valor
    // T& por referencia
    // &mut & por ref

    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f);

    // pasaje x valor
    let plus_three = |x: &mut i32| *x += 3;

    // let plus_three2 = |mut x: i32| x += 3;
    // let g = 12;
    // plus_three2(g);
    // println!("g = {}", g);
}

fn is_even(x: u32) -> bool
{
    x % 2 == 0
}

fn greater_than(limit: u32) 
-> impl Fn(u32) -> bool 
{
    // el move va porque al salir de la funcion el limite se destruye
    move |y| y > limit
}

fn sum_even_squares() 
{
    // sum of all even squares

    let limit = 500;
    let mut sum = 0;

    // resuelto como closure
    // let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);


    for i in 0.. {
        let isq = i*i;

        if above_limit(isq) {
            break
        }
        else if is_even(isq){
            sum += isq;
        }
    }

    println!("loop sum = {}", sum);



    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit)
        .filter(|x:&u32| is_even(*x))
        .fold(0, |sum, x| sum + x);

    println!("hof sum = {}", sum2);
}



pub fn higher_order_fn()
{
    // functions that take func
    // f(g) { let x = g();}

    // functions that return functions
    // genereators
    // f() -> g

    sum_even_squares();

}

