use std::{fmt::Display, vec};

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

    struct DisplayStruct<T: Display> {
        field: T,
    }

    enum HTTPResp<T> {
        Success(T),
        Error(T),
    }

    let resp1: HTTPResp<i32> = HTTPResp::Success(200);
    let resp2: HTTPResp<&str> = HTTPResp::Success("OK");
    let err1: HTTPResp<i32> = HTTPResp::Error(404);
    let err2: HTTPResp<&str> = HTTPResp::Error("Not Found");

    fn handle_response<T: Display>(response: HTTPResp<T>) {
        match response {
            HTTPResp::Success(value) => println!("Success: {}", value),
            HTTPResp::Error(value) => println!("Error: {}", value),
        }
    }

    handle_response(resp1);
    handle_response(resp2);
    handle_response(err1);
    handle_response(err2);

    trait Sorter<T> {
        fn sort(&self, slice: &mut [T]);
    }

    struct SelectionSorter;

    impl<T: PartialOrd> Sorter<T> for SelectionSorter {
        fn sort(&self, slice: &mut [T]) {
            for i in 0..slice.len() {
                let mut min_index = i;
                for j in i + 1..slice.len() {
                    if slice[j] < slice[min_index] {
                        min_index = j;
                    }
                }
                slice.swap(i, min_index);
            }
        }
    }

    struct BubbleSorter;

    impl<T: PartialOrd> Sorter<T> for BubbleSorter {
        fn sort(&self, slice: &mut [T]) {
            for i in 0..slice.len() {
                for j in 0..slice.len() - 1 {
                    if slice[j] > slice[j + 1] {
                        slice.swap(j, j + 1);
                    }
                }
            }
        }
    }

    let mut numbers = vec![42, 1337, 28, 10];
    let sorter = SelectionSorter;
    sorter.sort(&mut numbers);

    let mut words = vec!["cherry", "apple", "banana"];
    let sorter = BubbleSorter;
    sorter.sort(&mut words);
}
