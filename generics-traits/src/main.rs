use std::fmt::Display;

trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn filter(self /*, predicate closure*/) -> MyFilter {
        todo!()
    }

    fn map(self /*, mapping closure*/) -> MyMap {
        todo!()
    }

    fn sum(self) -> i32 {
        todo!()
    }
}

impl<T> MyIterator for Vec<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(self.remove(0))
        }
    }
}

struct MyFilter {}

struct MyMap {}

fn print_iterator<T: Display>(mut iterator: impl MyIterator<Item = T>) {
    // Remember that MyIterator is not integrated to Rust
    // you will not be able to use `for elt in iterator {`
    todo!()
}

fn main() {
    let enumeration = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    print_iterator(enumeration.clone());

    // let filtered = enumeration.clone().filter(|&item| item % 2 == 0);
    // print_iterator(filtered);

    // let mapped = enumeration.clone().map(|item| item * 2);
    // print_iterator(mapped);

    // let total = enumeration.clone().sum();
    // println!("Total: {}", total);
}
