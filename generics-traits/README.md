# Implementing MyIterator

An iterator is a struct that implements a Trait with a method `next()` which returns an `Option`
that can contain an `Item` of a collection iterated over.


```rust
trait MyIterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}
```

=> Implements MyIterator trait for a Vec.
=> Implements a function that can print a `MyIterator`.

A _Filter_ is an iterator that wraps another iterator and returns its next item whenever this item
is satisfying a predicate closure: `an_iterator.filter(|&item| item % 2 == 0);`

=> Implements a method `my_filter()` on `MyIterator` that wraps the current instance of `MyIterator`
into a `MyFilter` iterator.

A _Map_ is an iterator that wraps another iterator and returns its next item by applying a mapper
function to them item and returns its result: `an_iterator.map(|item| item * 2);`

=> Implements a method `my_map()` on `MyIterator` that wraps the current instance of `MyIterator`
into a `MyMap` iterator.

=> Implements a method `sum()` on `MyIterator`
