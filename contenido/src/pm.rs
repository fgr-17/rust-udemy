fn how_many(x:i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        _z @ 9..=11 => "lots of",
        _ if (x%2 == 0)=> "a few",
        _ => "some"

    }
}


pub fn pattern_matching() {

    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (0, 4);

    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        // (ref mut x, 0) => println!("y axis, x = {}", x), // para cambiar el valor de x
        (x, y) => println!("({},{})", x, y)
        // (_, y) => println!("(?,{})", y) // si no uso la variable x
        
    }

}