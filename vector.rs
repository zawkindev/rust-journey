fn main() {
    let _vector = vec![1, 2, 3, 4, 5];
    let mut v_exp = Vec::<i8>::new();
    v_exp.push(1);
    v_exp.push(2);
    v_exp.push(3);

    let mut v_imp = Vec::new();
    v_imp.push("onw");
    v_imp.push("two");
    v_imp.push("three");

    let mut v_cap = Vec::with_capacity(3);
    v_cap.push("1ne");
    v_cap.push("2wo");
    v_cap.push("3hree");
    println!("length: {}", v_cap.len());
    println!("capacity: {}", v_cap.capacity());

    let mut v_iter: Vec<i32> = (1..5).collect();
    println!("vector from iterator: {:?}", v_iter);

    let third: &i32 = &v_iter[2];
    println!("third element: {}", third);

    let is_exist = v_iter.get(9);
    match is_exist {
        Some(_) => println!("it EXISTSS!"),
        None => println!("no 9th element ofc, look at the above"),
    }

    for i in v_iter.iter() {
        println!("element: {}", i);
    }

    enum Variant {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let var_vec = vec![
        Variant::Int(3),
        Variant::Float(4.5),
        Variant::Text(String::from("so sleepy")),
    ];

    for i in var_vec {
        match i {
            Variant::Int(i) => println!("Int ekan: {}", i),
            Variant::Float(f) => println!("Float ekan: {}", f),
            Variant::Text(t) => println!("Text ekan: {}", t),
        }
    }

    let first3 = &v_iter[0..3];
    println!("first 3 values: {:?}", first3);

    for i in &v_iter {
        println!("haha: {}", i);
    }

    v_iter.insert(0, 34);
    println!("new values inserted: {:?}", v_iter);

    let removed = v_iter.remove(3);
    println!("removed: {}", removed);

    let popped = v_iter.pop();
    println!("popped: {:?}", popped);

    v_iter.clear();
    println!("cleard!!!");
}
