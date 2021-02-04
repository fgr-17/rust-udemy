// lifetime and memory:
#![allow(unused_mut)]

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