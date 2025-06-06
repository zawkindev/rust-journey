fn main() {
    let arr_irr = [1, 2, 3, 4, 5];
    let arr_exp: [i8; 5] = [1, 2, 3, 4, 5];
    println!("inferred: {:?}", arr_irr);
    println!("explicit: {:?}", arr_exp);

    let same = [3; 5];
    println!("same: {:?}", same);

    let arr_2d: [[i8; 2]; 2] = [[1, 2], [3, 4]];
    println!("2d: {:?}", arr_2d);

    let address_index: usize = 2;
    let haha = arr_exp[address_index];
    println!("haha: {:?}", haha);

    let arr_size = arr_2d.len();
    println!("size of 2d: {:?}", arr_size);

    let slice = &arr_irr[1..3];
    let rest = &arr_irr[1..];
    let all = &arr_irr[..];
    println!("slice of arr: {:?}", slice);
    println!("rest of arr: {:?}", rest);
    println!("all of arr: {:?}", all);

    for element in arr_irr.iter() {
        println!("element of arr: {}", element);
    }

    for (i, element) in arr_irr.iter().enumerate() {
        println!("{}th element: {}", i, element);
    }

    for i in 3..arr_irr.len() {
        println!("index: {}, element: {}", i, &arr_irr[i]);
    }
}
