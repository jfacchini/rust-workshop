use std::fmt::Display;

trait MyIterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // Replace ? with correct Generic type parameter
    // fn filter<?>(self, predicate: ?) -> MyFilter<?, ?> {
    //     MyFilter {
    //         iterator: self,
    //         predicate,
    //     }
    // }

    // fn map(self /*, mapping closure*/) -> MyMap<?, ?> {
    //     todo!()
    // }

    // fn sum(mut self) -> i32 {
    //     todo!()
    // }
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

struct MyFilter<I, P> {
    iterator: I,
    predicate: P,
}

struct MyMap<I, M> {
    iterator: I,
    mapper: M
}

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

    // let mapped = enumeration.clone().map(|item| item * 1.0);
    // print_iterator(mapped);

    // let total = enumeration.clone().sum();
    // println!("Total: {}", total);

    // let filtered_mapped_total = enumeration.clone()
    //     .filter(|&item| item % 2 == 0)
    //     .map(|item| item * 2)
    //     .sum();
    // println!("Filtered Mapped total is: {}", filtered_mapped_total);
}
