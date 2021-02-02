// lifetime and memory:


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