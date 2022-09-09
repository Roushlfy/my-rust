# Chapter 8 Generic Types, Traits and Lifetimes

## 8.1 Generic Data Types
#### 8.1.1 Write a generic function
``` rust
fn largest<T>(list: &[T]) -> &T { 
	let mut largest = &list[0]; 
	for item in list { 
		if item > largest { 
			largest = item; 
		} 
	} 
	largest 
}
```
``` rust
fn largest<T>(list: &[T]) -> &T { 
	let mut largest = &list[0]; 
	for &item in list.iter() { 
		if item > largest { 
			largest = item; 
		} 
	} 
	largest 
}
```
#### 8.1.2 Useful generics:
``` rust
enum Option<T> { 
	Some(T), 
	None, 
}

enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

#### 8.1.3 In Method Definitions
We can implement methods on structs and enums and use generic types in their definitions.
We have to declare `T` just after `impl` so we can use `T` to specify that we're implenment methods on the type `Point<T>`
``` rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
}
```

We can also specify constraints on generic types when defining methods on the type. For example, we can implement methods only on `Point<32>` instances rather than on `Point<T>`
``` rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
This code means the type `Point<f32>` will have a `distance_from_origin` method; other instances of `Point<T>` where `T` is not of type `f32` will not have this method defined.


``` rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```
The purpose of this example is to demonstrate a situation in which some generic parameters are declared with `impl` and some are declared with the method definition. Here, the generic parameters `X1` and `Y1` are declared after `impl` because they go with the struct definition. The generic parameters `X2` and `Y2` are declared after `fn mixup`, because they’re only relevant to the method.

### 8.2 Traits: defining shared behaviors
#### 8.2.1 Defining a trait:
``` rust
// A Summary trait that consists of the behavior provided by a summarze method
pub trait Summary {
	fn summarize(&self) -> String;
}
```
Or you can also implement the trait on its defining /  declaration
``` rust

pub trait Summary {
	fn summarize(&self) -> String{
		String::from("Summarized")
	};
}
```
#### 8.2.2 Implementing a **Trait** on a **Type**
use the `impl` and `for` keyword
``` rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
 
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### 8.2.3 Default implementations:
``` rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

```

#### 8.2.4 Traits as Parameters
use the `impl Trait` syntax:
``` rust
pub fn nofity(item: &impl Summary) {
	println!("Breaking news! {}", item.summarize());
}
```
Instead of a concrete type for the `item` parameter, we specify the `impl` keyword and the trait name.
**This parameter accepts any type that implements the specified trait.**
In the body of `notify`, we can call any methods on `item` that come from the `Summary` trait.
We can call `notify` and pass in any instance of `NewsArticle` or `Tweet`. 
Code that calls the function with any other type, such as a `String` or an `i32`, won’t compile because those types don’t implement `Summary`.

##### Trait Bound Syntax
The `impl Trait` syntax works is actually syntax sugar for a **trait bound**
``` rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```
For multiple parameters, the complicated version of `impl trait` can be more convenient. Compare the following two versions:
``` rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
	// do something
}

pub fn notify<T: Summary>(item1: &T, item2: &T) {
	// do something
}
```

##### Specifying Multiple Trait Bounds with the `+` Syntax
Specify in the `notify` definition that `item` must implement **both** `Display` **and** `Summary` by using the `+` syntax:
``` rust
pub fn notify(item: &(impl Summary + Display)) {
	// do something
}
```
The `+` syntax is also valid with trait bounds on generic types:
``` rust
pub fn notify<T: Summary + Display>(item: &T) {
	// do something
}
```

##### Clearer Trait Bounds with `where` Clauses
Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying trait bounds inside a `where` clause after the function signature. So instead of writing this:
``` rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
```
use a `where` clause:
``` rust
fn some_function<T, U>(t: &T, u: &U) -> i32 {
	where T: Display + Clone,
          U: Clone + Debug
{
```

#### 8.2.5 Returning Types that Implement Traits
We can also use the `impl Trait` syntax in the return position to return a value of some type that implements a trait.
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```
By using `impl Summary` for the return type, we specify that the `returns_summarizable` function returns some type that implements the `Summary` trait without naming the concrete type. In this case, `returns_summarizable` returns a `Tweet`.
The ability to specify a return type only by the trait it implements is especially useful in the context of **closures** and **iterators**.

However, you can only use `impl Trait` if you’re returning a single type. For example, this code that returns either a `NewsArticle` or a `Tweet` with the return type specified as `impl Summary` wouldn’t work, even though both `NewsArticle` and `Tweet` implement the `Summary` trait.
``` rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

#### 8.2.6 Using Trait Bounds to Conditionally Implement Methods
By using a trait bound with an `impl` block that uses generic type parameters, we can implement methods **conditionally** for types that implement the specified traits. 
For example, the type `Pair<T>` in the following code block always implements the `new` function to return a new instance of `Pair<T>`. But in the next `impl` block, `Pair<T>` only implements the `cmp_display` method if its inner type `T` implements the `PartialOrd` trait that enables comparison **and** the `Display` trait that enables printing.
``` rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
That being said, you can only call `.cmp_display()` on `Pair<T>` instances if the type `T` implements both the `Display` and the `PartialOrd` trait.

**We can also conditionally implement a trait for any type that implements another trait.** 
Implementations of a trait on any type that satisfies the trait bounds are called **blanket implementations** and are extensively used in the Rust standard library. For example, the standard library implements the `ToString` trait on any type that implements the `Display` trait. The `impl` block in the standard library looks similar to this code:
``` rust
impl<T: Display> ToString for T {
    // --snip--
}
```
### 8.3 Lifetime
