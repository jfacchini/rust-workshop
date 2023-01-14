# Implementing (a simplistic) MyIterator

An iterator is a struct that implements a Trait with a method `next()` which returns an `Option`
that can contain an `Item` of a collection iterated over.


```rust
trait MyIterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}
```

## Implement a function that can print a `MyIterator`

## Implement MyFilter iterator

A _Filter_ is an iterator that wraps another iterator and returns its next item whenever this item is satisfying a predicate closure: `my_iterator.filter(|&item| item % 2 == 0);`

=> Implements the method `filter()` on `MyIterator` that take ownership of the current instance of `MyIterator` into a `MyFilter` iterator.

## Implement MyMap iterator

A _Map_ is an iterator that wraps another iterator and returns its next item by applying a mapper function to them item and returns its result: `my_iterator.map(|item| item * 2);`

=> Implements a method `map()` on `MyIterator` that take owernship of the current instance of `MyIterator` into a `MyMap` iterator.

## Implement sum on MyIterator

=> Implement a method `sum()` on `MyIterator`

## Complements

- to accept a closure as parameter, use a generic type
    ```rust
    fn use_closure<F: Fn(String) -> bool>(exec: F) {
    ```
- to execute a closure from a struct field
    ```rust
    fn closure_from_struct(some_struct: SomeStruct) {
        if (some_struct.exec)(String::new()) {
            println!("Closure returned true");
        }
    }
    ```
