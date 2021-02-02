// data structures

#![allow(dead_code)]

use std::mem;

struct Point {

    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point

}

fn origin()-> Point {
    let ret = Point{x: 0.0, y:0.0};
    return ret;
}

fn print_point(p:&Point) {
    print!("({},{})", p.x, p.y);
}

fn print_line(l:&Line) {
    print_point(&l.start);
    print!("->");
    print_point(&l.end);
    print!("\n");

}

pub fn structures() {

    let o = origin();
    let p = Point {x: 3.2, y: 4.2};


    let l = Line {start: o,end: p};

    print_line(&l);
}

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8,u8,u8),
    Cmyk{cyan:u8, magenta:u8,yellow: u8, black: u8},
}

pub fn colors() {
    // let c:Color = Color::Red;
    // let c:Color = Color::RgbColor(1,2,4);
    let c:Color = Color::Cmyk{cyan:0, magenta: 12, yellow: 42, black: 255};

    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
        Color::RgbColor(0,0,0) 
        // |    Color::Cmyk{cyan: _, magenta:_,  yellow:_, black:255}=> println!("black"),
        |    Color::Cmyk{black:255, ..}=> println!("black"), // solo analizo el color negro
        Color::RgbColor(r,g,b) => println!("rgb({},{},{})",r,g,b),
        
        _ => println!("otro"),
    }

}
// 32 bits union
union IntOrFloat {
    i: i32,
    f: f32
}


fn process_value(iof: IntOrFloat) {
    unsafe {
        match iof {
            IntOrFloat {i:42} => {
                println!("entero 42");
            }
            IntOrFloat {f} => {
                println!("es un float {}", f);
            }
        }
    }
}

pub fn unions() {
    let mut iof = IntOrFloat { i: 123};

    iof.i = 321;

    // rust obliga a ver el bloque como unsafe porque no se sabe como se escribio
    let value = unsafe { iof.i};

    println!("iof.i = {}", value);

    process_value(IntOrFloat{f:42.0});


}

pub fn option_type() {
    let x = 3.0;
    let y = 0.0;

    //
    let result = 
        if y!= 0.0 { Some(x/y)} else { None};

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("cannot divide by 0")
    }
    if let Some(z) = result {
        println!("result = {}", z);
    }

    // while let Some(_z) = result {
        
    // }
}

pub fn arrays() {
    // let mut a:[i32;5] = [1,2,3,4,5]; // info redundante de tamaño y tipo

    let mut a = [1,2,3,4,5];

    println!("a tiene {} elementos, el primero es {}", a.len(), a[0]);

    a[0] = 21;

    println!("a tiene {} elementos, el primero es {}", a.len(), a[0]);

    println!("{:?}", a);

    if a != [1,2,3,4,2] {
        println!("no coincide");
    }

    // completo todo el array de una
    let b = [1u16; 10];

    for i in 0..b.len() {
        println!("b[{}] = {}", i, b[i]);
    }

    println!("b ocupa {} bytes", mem::size_of_val(&b));

    let mtx:[[f32;3];2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];

    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i,j,mtx[i][j]);
            }
        }
    }



}

fn use_slice(slice: &mut [i32]) {
    println!("first elem Slice = {}, len {}", slice[0], slice.len());
    slice[0] = 1234;
}

pub fn slices() {

    // part of array: size is not available at compile time

    let mut data = [1,2,3,4,5];
    use_slice(&mut data[1..4]);
    // si lo paso como mut, lo puedo modificar en la fc
    use_slice(&mut data);
}


fn sum_and_product(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}

pub fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x,y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} =  {3}", x, y, sp.0, sp.1);

    // destructuring
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    // tuple of tuples
    let sp2 = sum_and_product(4, 7);
    let comb = (sp, sp2);

    println!("{:?}", comb);
    println!("last elem {}", (comb.1).1);

    let foo = (true, 42.0, -1i8);
    println!("foo {:?}", foo);

    // single element tuple
    let tuple1 = (42, );
    println!("tuple1 {:?}", tuple1);
}


// -------------- generics ---------------

struct PointGen<T> {
    x: T, 
    y: T
}

struct LineGen<T> {
    start: PointGen<T>,
    end: PointGen<T>
}

pub fn generics() {

    let a = PointGen { x:0f64, y:0f64};
    let b = PointGen { x: 1.2, y:3.2};
    let _c:PointGen<i32> = PointGen { x: 13, y:31};

    let _my_line = LineGen {start: a, end: b};

}

