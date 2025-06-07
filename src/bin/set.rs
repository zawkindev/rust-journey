use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = HashSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);

    a.insert(2);

    a.remove(&2);

    let b: HashSet<i32> = [3, 4, 5].iter().cloned().collect();

    println!("Contains 1: {}", a.contains(&1));

    println!("Union: {:?}", a.union(&b).collect::<HashSet<&i32>>());

    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<HashSet<&i32>>()
    );

    println!(
        "Difference: {:?}",
        a.difference(&b).collect::<HashSet<&i32>>()
    );

    println!(
        "symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<HashSet<&i32>>()
    );

    println!("Is a subset: {}", a.is_subset(&b));
    println!("Is a superset: {}", a.is_superset(&b));
    println!("Is disjoint: {}", a.is_disjoint(&b));

    for i in a.iter() {
        println!("{}", i);
    }
}
