use std::fmt::Display;

fn main() {
    fn first_element<T>(slice: &[T]) -> Option<&T> {
        if slice.is_empty() {
            None
        } else {
            Some(&slice[0])
        }
    }

    let numbers = vec![10, 20, 30];
    let floats = vec![1.1, 2.2, 3.3];
    let words = vec!["apple", "banana", "cherry"];

    let first_float = first_element(&floats);
    let first_number = first_element(&numbers);
    let first_word = first_element(&words);

    fn combine<T, U>(x: T, y: U) -> (T, U) {
        (x, y)
    }

    let number_and_float = combine(10, 1.1);
    let word_and_number = combine("apple", 10);

    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let largest_number = largest(&numbers);
    let largest_float = largest(&floats);
    let largest_word = largest(&words);

    fn max<T: Display + PartialOrd>(x: T, y: T) -> T {
        if x > y { x } else { y }
    }

    let max_number = max(14, 23);
    let max_float = max(14.4, 0.23);
    let max_word = max("asuna", "mizuhara");

    struct GenericStruct<T> {
        field: T,
    }

    impl<T> GenericStruct<T> {
        fn get_field(&self) -> &T {
            &self.field
        }
    }

    let number_struct = GenericStruct { field: 10 };
    let float_struct = GenericStruct { field: 1.0 };
    let word_struct = GenericStruct { field: "megumi" };

    impl GenericStruct<f32> {
        fn get_float_field(&self) -> f32 {
            self.field
        }
    }
}
