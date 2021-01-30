// standart collections

#![allow(dead_code)]


pub fn vectors() {

    let mut a = Vec::new();

    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    println!("a[0]={}", a[0]);

    let idx:usize = 0;

    // los indices de arrays no pueden ser de cualquier tipo
    println!("a[0]={}", a[idx]);

    // si idx > maximo --> tira "panic"

    // traer los elementos con get y match
    match a.get(6) {
        Some(x) => println!("a[6]= {}", x),
        None => println!("no existe")
    }

    for x in &a { println!("{}", x); }

    let last_elem = a.pop();
    println!("last elemtn {:?}", last_elem);


    while let Some(x) = a.pop() {
        println!("{}", x);
    }

}

use std::collections::HashMap;

pub fn hash_map() {

    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides", shapes["square".into()]);

    // si la key esta mal, crashea
    // println!("a square has {} sides", shapes["circle".into()]);

    for (key, value) in &shapes {
        println!("{} : {}", key, value);
    }

    // override value
    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);

    // solo lo agrega si no existe, si existe lo modifica
    shapes.entry("circle".into()).or_insert(1);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}
use std::collections::HashSet;
pub fn hash_set() {

    let mut greeks = HashSet::new();

    greeks.insert("gamma");
    greeks.insert("delta");
    println!("{:?}", greeks);

    // si repito un elemento no pasa nada
    greeks.insert("delta");
    println!("{:?}", greeks);

    let added_vega = greeks.insert("vega");
    if added_vega {
        println!("{:?}", greeks);
    }

    // devuelve cero porque ya esta incluído
    let added_vega = greeks.insert("delta");
    if added_vega {
        println!("{:?}", greeks);
    }

    if !greeks.contains("kappa") {
        println!("no la tengo");
    }

    let removed = greeks.remove("delta");
    if removed {
        println!("removido");
        println!("{:?}", greeks);
    }


    // defino varios sets como rangos de valores

    let _1_5: HashSet<_> = (1..=5).collect();
    let _6_10: HashSet<_> = (6..=10).collect();
    let _1_10: HashSet<_> = (1..=10).collect();
    let _2_8: HashSet<_> = (2..=8).collect();

    println!("_1_5: {:?}", _1_5);
    
    // subsets:
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8, _1_10,
        _2_8.is_subset(&_1_10)
    );

    // disjoint
    println!(
        "is {:?} a disjoint of {:?}? {}",
        _1_5, _6_10,
        _1_5.is_subset(&_6_10)
    );


    // union, interjection
    println!(
        "items en {:?} como en {:?} son {:?}",
        _2_8, _6_10,
        _2_8.union(&_6_10)
    );

    println!(
        "items en {:?} y en {:?} son {:?}",
        _2_8, _6_10,
        _2_8.intersection(&_6_10)
    );

}


pub fn iterators() {

    let vec = vec![3, 2, 1];

    for x in vec {
        println!("{}", x);
    }

    // no funciona porque antes le saque los valores
    // println!("vec: {:?}", vec);

    let vec2 = vec![3,2,1];
    // si lo leo como puntero lo puedo volver a utilizar
    for x in &vec2 {
        println!("{}", *x);
    }
    println!("vec2: {:?}", vec2);
    println!("vec2: {:?}", vec2);

    for x in vec2.iter() {
        println!("leo: {}", x);
        // es una referencia y no puedo modificar
        // x += 1;
    }


    let mut vec3 = vec![3,2,1];
    for x in vec3.iter_mut() {
        println!("leo: {}", x);
        // es una referencia y no puedo modificar
        *x += 1;
    }

    println!("{:?}", vec3);

    // recorro el iterador al reves
    for x in vec3.iter().rev()
    {
        println!("leo: {}", x);
    }


    // into iter: move operator
    let vec4 = vec![3,2,1];
    let mut vec5 = vec![1,2,3];
    // extend llama a into_iter y hace move de todo lo que tiene el vec adentro
    // let it = vec.into_iter();
    vec5.extend(vec4);
    println!("{:?}", vec5);
    // vec4 no se puede usar porque se vacio
    //println!("{:?}", vec4);




}