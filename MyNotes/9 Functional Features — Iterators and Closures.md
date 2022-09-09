# Chapter 9 Functional Language Features: Iterators and Closures
## Closures: Anonymous Functions that Capture Their Environment
- Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. 
- You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context. 
- Unlike functions, closures can capture values from the scope in which they’re defined.

### Capturing the Environment with Closures

## Iterators
Iterators are *lazy*.
### The Iterator `Trait` and the `next` Method
``` rust
pub trait Iterator {
	type Item;

	fn next(&mut self) -> Option<Self::Item>;
}
```
Syntax `type Item` and `Self::Item` are defining an *[[15 Advanced Features]]* with this trait.
We can call the `next` method on iterators directly. (Similar to python).
Other methods that returns an iterator:
`iter_mut()` which returns a mutable iterator and `into_iter()` which takes the ownership of the element.
### Methods that Consume the Iterator
Methods that call `next` are called _consuming adaptors_, because calling them uses up the iterator. 
However, since the `iter()` method only borrows the element in the container, calling consuming adaptors (like `sum()`) will not take the ownership of the element in the container, which is pretty natural.

### Methods that Produce Other Iterators
_Iterator adaptors_ are methods defined on the `Iterator` trait that don’t consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.
The `map` method returns a new iterator that produces the modified items.
``` rust
let v1: Vec<i32> = vec![1, 2, 3];

v1.iter().map(|x| x + 1);
```
However, the code will generate a warning, because iterators are *lazy*. 
To actually "use" the iterators, trying calling `collect()` method.
``` rust
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
```
**Note: you'd better explicitly tell the compiler the type of the return value of the `collect()` method, because it can return different type of containers.**

### Using Closures that Capture Their Environment
For example, the `filter()`method.
The `filter()` method takes a closure. The closure gets an item from the iterator and returns a Boolean. If the closure returns `true`, the value will be included in the iteration produced by `filter`. If the closure returns `false`, the value won’t be included.

## Comparing Perfromance: Loops vs. Iterators
 Iterators are one of Rust’s _zero-cost abstractions_, by which we mean using the abstraction imposes no additional runtime overhead. This is analogous to how Bjarne Stroustrup, the original designer and implementor of C++, defines _zero-overhead_ in “Foundations of C++” (2012):

> In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.

Here is an example from the book *The Rust Programming Language* :
``` rust
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

## Time for Examples:
Here are a few examples that appear in my own project `wordle`.
``` rust
// to ensure a string is valid ascii code
fn sanitize_word(word: &&str) -> String {
    word.trim()
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect()
}

// examine every line in the word.txt and returns a list of words in a vector
fn word_list(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .map(sanitize_word) 
        .filter(|line| line.len() == WORD_LENGTH)
        .collect()
}
```

### How to read the documentation for an iterator ?
First off, the names matters. Quite often, you can guess their functions directly from their names. Then go to the details of its definition, derivation, etc., to look up for APIs.