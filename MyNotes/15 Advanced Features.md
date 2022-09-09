# Chapter 15 Advanced Features
## Unsafe Rust
- 解引用裸指针
- 调用不安全的函数或方法
- 访问或修改可变静态变量
- 实现不安全的trait

### Using `extern` Functions to Call External Code

Rust has the keyword `extern` that facilitates the creation and use of a _Foreign Function Interface (FFI)_. An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

> **Calling Rust Functions from Other Languages**
> 
> We can also use `extern` to create an interface that allows other languages to call Rust functions. Instead of an creating a whole `extern` block, we add the `extern` keyword and specify the ABI to use just before the `fn` keyword for the relevant function. We also need to add a `#[no_mangle]` annotation to tell the Rust compiler not to mangle the name of this function. _Mangling_ is when a compiler changes the name we’ve given a function to a different name that contains more information for other parts of the compilation process to consume but is less human readable. Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.
> 
> ``` rust
> #[no_mangle]
> pub extern "C" fn call_from_c() {
> 	println!("Just called a Rust function from C!");
> }
> ```

### Accessing or Modifying a Mutable Static Variable
The names of static variables are in `SCREAMING_SNAKE_CASE` by convention. 
Static variables can only store references with the `'static` lifetime.

- A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory. Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used. 
- Another difference is that static variables can be mutable. Accessing and modifying mutable static variables is _unsafe_.




### Implementing an Unsafe Trait
A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify. 
We declare that a trait is `unsafe` by adding the `unsafe` keyword before `trait` and marking the implementation of the trait as `unsafe`.

``` rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

As an example, recall the `Sync` and `Send` marker traits : the compiler implements these traits automatically if our types are composed entirely of `Send` and `Sync` types. If we implement a type that contains a type that is not `Send` or `Sync`, such as raw pointers, and we want to mark that type as `Send` or `Sync`, we must use `unsafe`. Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads; therefore, we need to do those checks manually and indicate as such with `unsafe`.

## Advanced Traits
### Specifying Placeholder Types in Trait Definitions with Associated Types
*Associated types* connect a type *placeholder* with a trait such that the *trait method* definitions can use these placeholder types in their signatures. That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.

Example: `Iterator` Trait
Definition:
``` rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

The type `Item` is a placeholder, and the `next` method will return values of type `Option<Self::Item>`.

> What's the difference between generics and associated types?

Why not define the `Iterator` type like this ?

 ``` rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

The difference lies in that  we have to annotate the types in each implementation. 

### Default Generic Type Parameters and Operator Overloading

You specify a default type when declaring a generic type with the `<PlaceholderType=ConcreteType>` syntax.

Example: *operator overloading*

Rust doesn’t allow you to create your own operators or overload arbitrary operators. But you can overload the operations and corresponding traits listed in `std::ops` by implementing the traits associated with the operator. For example, we can overload the `+` operator by implementing the `Add` trait.

``` rust
use std::ops::Add;// only traits listed in std::ops can be overloaded

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

The default generic type in this code is within the `Add` Trait.
Definition:
``` rust
trait Add<Rhs=Self> {
	type Output;

	fn add(self, rhs: Rhs) -> Self::Output;
}
```
Explanation:
`Rhs = Self`: this syntax is called *default type parameters*.
The `Rhs` generic type parameter (short for “right hand side”) defines the type of the `rhs` parameter in the `add` method. If we don’t specify a concrete type for `Rhs` when we implement the `Add` trait, the type of `Rhs` will default to `Self`, which will be the type we’re implementing `Add` on.
Here's an example where we want to customize the `Add` trait:
``` rust
use std::ops::Add;

struct Milimeters(u32);
struct Meters(u32);

impl Add<Meters> for Milimeters {
	type Output = Milimeters;

	fn add(self, other: Meters) -> Milimeters {
		Milimeters(self.0 + other.0 * 1000)
	}
}
```

### Fully Qualified syntax for Disambiguation: Calling Methods with the Same Name
``` rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
}
```
The code above compiles.
However, if we want to use the function`baby_name` of  trait `Animal`:
``` rust
fn main() {
    println!("A baby dog is called a {}", Animal::baby_name());
}
```
Compile Error:
> cannot satisfy `_: Animal`

The compiler cannot resolve `Animal` is implemented for which instance.

Instead use :
``` rust
fn main() {
	pirntln!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
```

The syntax:
> \<Type as Trait>::function(receiver_if_method, next_arg, ...);

### Using **Supertraits** to Require One Trait's Functionality Within Another Trait
This technique is similar to adding a **trait bound** to the trait.

### Using the Newtype Pattern to Implement External Traits on External Types
> We're only allowed to implement a trait on a type if *either the trait or the type are local to our crate*.

However, it's possible to get around this restriction using the *newtype pattern*, which involves creating a new type in a *tuple struct*.

``` rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

We can define a new `struct` `Wrapper` and implement `fmt::Display` for `Wrapper` , so that we c

## Advanced Types

## Advanced Functions and Closures

## Macros