 # Chapter 11 Smart Pointers
 References, indicated by the `&` symbol, are the most common *pointers* in Rust. References borrow the value they point. 
 *Smart pointer*, on the other hand, are data structures that <u> act like a pointer but also have additional meta data and capabilities.</u>
 Both `String` and `Vec<T>` are common common smart pointers.
 Smart pointers usually implemented using structs. Unlike an ordinary struct, smart pointers implement the `Deref` and `Drop` traits. More on those later.
 The following are three common smart pointers in the standard library:
- `Box<T>` for allocating values on the heap
-   `Rc<T>`, a reference counting type that enables multiple ownership
-   `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

## Using `Box<T>` to Point to Data on the Heap
Three situations where `Box<T>` is often used:
-   When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
-   When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
-   When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
### Enabling Recursive Types with Boxes : the Cons List
A *cons list* is a data structure that comes from the Lisp programming language and is the Lisp version of a linked list. Its name comes from the `cons`function (short for "construct").
For example:
``` rust
(1, (2, (3, Nil)))
```
``` rust
enum List {
	Cons(i32, List),
	Nil,
}
```
*Note that this code won't compile because the `List` type doesn't yet have a known size.*
Instead, use a `Box<List>`, which is the size of `usize`.
``` rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

## Treating Smart Pointers Like Regular References with the `Deref` Trait
Implementing the `Deref` trait allows you to customize the behavior of the *dereference operator* `*`.
### A Common Pointer
``` rust
 fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

### Using `Box<T>` Like a Reference
``` rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```
### Defining Our Own Smart Pointer
``` rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
	fn new(x: T) -> MyBox<T> {
		MyBox(x)
	}
}
```
The `MyBox` type is a *tuple* struct with one element of type `T`. The `MyBox::new` function takes one parameter of type `T` and returns a `MyBox` instance that holds the value passed in.

### Treating a Type Like a Reference by Implementing the `Deref` Trait

To implement a trait, we need to provide implementations for the trait’s required methods.
The `Deref` trait, provided by the standard library, requires us to implement one method named `deref` that borrows `self` and returns a reference to the inner data.
``` rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
The `type Target = T` syntax defines an associated type for the `Deref` trait to use.
Without the `Deref` trait, the compiler can only dereference `&` references. The `deref` method gives the compiler the ability to take a value of any type that implements `Deref` and call the `deref` method to get a `&` reference that it knows how to dereference.
When we entered `*y` in Listing 15-9, behind the scenes Rust actually ran this code:
``` rust
*(y.deref())
```
The reason the `deref` method returns a reference to a value, and that the plain dereference outside the parentheses in `*(y.deref())` is still necessary, is to do with the ownership system. If the `deref` method returned the value directly instead of a reference to the value, the value would be moved out of `self`.
*Note that the `*` operator is replaced with a call to the `deref` method and then a call to the `*` operator just once, each time we use a `*` in our code.*

### Implicit Deref Coercions with Functions and Methods

_Deref coercion_ converts a reference to a type that implements the `Deref` trait into a reference to another type.
For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `&str`. It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the `deref` method converts the type we provided into the type the parameter needs.

``` rust
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```
Here, we're calling the `hello` function with the argument `&m`, which is a reference to a `MyBox<String>`. 
Because we have implemented the `Deref` trait on `MyBox<T>` , Rust can turn `&MyBox<String>` into `&String` by calling `deref`. The standard library provides an implementation of `Deref` on `String` that returns a string slice, and this is in the API documentation for `Deref`. Rust calls `deref` again to turn the `&String` into `&str`, which matches the `hello` function’s definition.
If Rust didn’t implement deref coercion, we would have to write code like this:
``` rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```
The `(*m)` deferences the `MyBox<String>` into a `String`. Then the `&` and `[..]` take a string slice of the `String` that is equal to the whole string to match the signature of `hello`.

When the `Deref` trait is defined for the types involved, Rust will analyze the types and use `Deref::deref` as many times as necessary to get a reference to match the parameter’s type. The number of times that `Deref::deref` needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!

### How Deref Coercion Interacts with Mutability

Similar to how you use the `Deref` trait to override the `*` operator on immutable references, you can use the `DerefMut` trait to override the `*` operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:

-   From `&T` to `&U` when `T: Deref<Target=U>`
-   From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
-   From `&mut T` to `&U` when `T: Deref<Target=U>`

The first two cases are the same as each other except that the second implements mutability. The first case states that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can get a `&U` transparently. The second case states that the same deref coercion happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an immutable one. But the reverse is _not_ possible: immutable references will never coerce to mutable references. 

## Running Code on Cleanup with the `Drop` Trait

The `Drop` trait requires you to implement one method named `drop` that takes a mutable reference to `self`.

We can’t disable the automatic insertion of `drop` when a value goes out of scope, and we can’t call the `drop` method explicitly. If we need to force a value to be cleaned up early, we use the `std::mem::drop` function. The function is in the prelude, so we can simply write `drop(item)` anywhere.


## `Rc<T>` , the Reference Counted Smart Pointer
You have to enable multiple ownership explicitly by using the Rust type `Rc<T>`, which is an abbreviation for _reference counting_. The `Rc<T>` type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.
**Note**
> `Rc<T>` is only for use in single-threaded scenarios.

## Using `Rc<T>` to Share Data

You have to enable multiple ownership explicitly by using the Rust type `Rc<T>`, which is an abbreviation for _reference counting_.

``` rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```
This will not compile because `c` used `a` after the ownership was given to `b`.
Instead, use `Rc<T>`
``` rust
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```
We need to add a `use` statement to bring `Rc<T>` into scope because it’s not in the prelude.
We could have called `a.clone()` rather than `Rc::clone(&a)`, but Rust’s convention is to use `Rc::clone` in this case. The implementation of `Rc::clone` doesn’t make a deep copy of all the data like most types’ implementations of `clone` do. The call to `Rc::clone` only increments the reference count, which doesn’t take much time.

>Is `clone()`  deep copy ? 

### Cloning an `Rc<T>` Increases the Reference Count
```rust
Rc::strong_count(&a)
```

This function is named `strong_count` rather than `count` because the `Rc<T>` type also has a `weak_count`.

Via immutable references, `Rc<T>` allows you to share data between multiple parts of your program for reading only.

## [`RefCell<T>` and the Interior Mutability Pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#refcellt-and-the-interior-mutability-pattern)
_Interior mutability_ is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.
To mutate data, the pattern uses `unsafe` code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.
The `unsafe` code involved is then wrapped in a safe API, and the outer type is still immutable.

### Enforcing Borrowing Rules at Runtime with `RefCell<T>`
What makes `RefCell<T>` different from a type like `Box<T>`?
With references and `Box<T>`, the borrowing rules’ invariants are enforced *at compile time*. With `RefCell<T>`, these invariants are enforced _at runtime_. With references, if you break these rules, you’ll get *a compiler error*. With `RefCell<T>`, if you break these rules, your program will *panic and exit*.
>`RefCell<T>` is only for use in single-threaded scenarios.
#### `Box<T>`, `Rc<T>` and `RefCell<T>`
-   `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>` have single owners.
-   `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>` allows only immutable borrows checked at compile time; `RefCell<T>` allows immutable or mutable borrows checked at runtime.
-   Because `RefCell<T>` allows mutable borrows checked at runtime, you can mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is immutable.

### [Interior Mutability: A Mutable Borrow to an Immutable Value](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#interior-mutability-a-mutable-borrow-to-an-immutable-value)
There are situations in which it would be useful for a value to mutate itself in its methods but appear immutable to other code. Code outside the value’s methods would not be able to mutate the value.

#### A Use Case for Interior Mutability: Mock Objects
- Test  Doubles
	Sometimes during testing a programmer will use a type in place of another type, in order to observe particular behavior and assert it's implemented correctly. This placeholder type is called a _test double_.
- Mock Objects
	_Mock objects_ are specific types of test doubles that record what happens during a test so you can assert that the correct actions took place.

``` rust
pub trait Messenger {
	fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
	messenger: &'a T,
	value: usize,
	max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where 
	T: Messenger,
{
	pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
		LimitTracker {
			messenger,
			value: 0,
			max,
 		}
	}

	pub fn set_value(&mut self, value:usize) {
		self.value = value;

		let percentage_of_max = self.value as f64 / self.max as f64;

		if percentage_of_max >= 1.0 {
			self.messenger.send("Error: You are over your quota!");
		} else if percentage_of_max >= 0.9 {
			self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
		} else if percentage_of_max >= 0.75 {
			self.messenger.send("Warning: You've used up over 75% of your quota!");
		}
	}
}
```
A library to keep track of how close a value is to a maximum value and warn when the value is at certain levels

Now, try to run the code above, and you will get a compile error

``` rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}
```
An attempt to implement a `MockMessenger` that isn’t allowed by the borrow checker

However, the compiler does't allow you to borrow `MockMessenger` as immutable while changing its element `sent_messages`.

How about changing the signature of the `sned()` method to `&mut self` ? Unfortunately, this isn't in line with the signature of the trait `Messenger`.

Now, it's time for `RefCell<T>` to show up. We'll store  the `sent_messages` within a `RefCell<T>`.
``` rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
```
For the implementation of the `send` method, the first parameter is still an immutable borrow of `self`, which matches the trait definition. We call `borrow_mut` on the `RefCell<Vec<String>>` in `self.sent_messages` to get a mutable reference to the value inside the `RefCell<Vec<String>>`, which is `Vec<String>`

We call `borrow` on the `RefCell<Vec<String>>` to get an immutable reference to the vector.

#### Keeping Track of Borrows at Runtime with `RefCell<T>`
When creating immutable and mutable references, we use the `&` and `&mut` syntax, respectively. 
With `RefCell<T>`, we use the `borrow` and `borrow_mut` methods, which are part of the safe API that belongs to `RefCell<T>`. The `borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut` returns the smart pointer type `RefMut<T>`. Both types implement `Deref`, so we can treat them like regular references.

The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. Every time we call `borrow`, the `RefCell<T>` increases its count of how many immutable borrows are active. When a `Ref<T>` value goes out of scope, the count of immutable borrows goes down by one. Just like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable borrows or one mutable borrow at any point in time.

If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation of `RefCell<T>` will panic at runtime.

Example:

``` rust
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

#### Downsides of `RefCell<T>`
- Choosing to catch borrowing errors at runtime rather than compile time, as we've done here, means you'd potentially be finding mistakes in your code later in the development process: possibly not until your code was deployed to production. 
- Also, your code would incur a small runtime performance penalty as a result of keeping track of the borrows at runtime rather than compile time.

### [Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt)

A common way to use `RefCell<T>` is in combination with `Rc<T>`. If you have an `Rc<T>` that holds a `RefCell<T>`, you can get a value that can have *multiple owners* _and_ that you can *mutate* !


**Note**
>`RefCell<T>` does not work for multithreaded code! `Mutex<T>` is the thread-safe version of `RefCell<T>`. Use `Mutex<T>` instead.



## [Reference Cycles Can Leak Memory](#reference-cycles-can-leak-memory)
*Attention here: why the list should be implemented like this?*
*Are there other ways to implement the cons list?*

``` rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<Listd>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

```
A cons list definition that holds a `RefCell<T>` so we can modify what a `Cons` variant is referring to.
>`Cons(i32,RefCell<Rc<List>>)` vs `Cons(Rc<RefCell<i32>>,Rc<List>)`

Instead of having the ability to modify `i32` value, we want to modify the `List` value a `Cons` variant is pointing to.

Another solution for avoiding reference cycles is reorganizing your data structures so that some **references express ownership and some references don’t**.

### Preventing Reference Cycles: Turning an `Rc<T> ` into  `Weak<T>`

We’ve demonstrated that calling `Rc::clone` increases the `strong_count` of an `Rc<T>` instance, and an `Rc<T>` instance is only cleaned up if its `strong_count` is 0. You can also create a _weak reference_ to the value within an `Rc<T>` instance by calling `Rc::downgrade` and passing a reference to the `Rc<T>`.

When you call `Rc::downgrade`, you get a smart pointer of type `Weak<T>`. Instead of increasing the `strong_count` in the `Rc<T>` instance by 1, calling `Rc::downgrade` increases the `weak_count` by 1. The `Rc<T>` type uses `weak_count` to keep track of how many `Weak<T>` references exist, similar to `strong_count`. The difference is the `weak_count` doesn’t need to be 0 for the `Rc<T>` instance to be cleaned up.

Because the value that `Weak<T>` references might have been dropped, to do anything with the value that a `Weak<T>` is pointing to, you must make sure the value still exists. Do this by calling the `upgrade` method on a `Weak<T>` instance, which will return an `Option<Rc<T>>`. You’ll get a result of `Some` if the `Rc<T>` value has not been dropped yet and a result of `None` if the `Rc<T>` value has been dropped. Because `upgrade` returns an `Option<Rc<T>>`, Rust will ensure that the `Some` case and the `None` case are handled, and there won’t be an invalid pointer.

### Create a Tree Data Structure: a `Node` with Child Nodes

A struct named `Node` that holds its own `i32` value as well as references to its children `Node` values:

``` rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}
```

We want a `Node` to own its children, and we want to share that ownership with variables so we can access each `Node` in the tree directly.
To do this, we define the `Vec<T>` items to be values of type `Rc<Node>`.
We also want to modify which nodes are children of another node, so we have a `RefCell<T>` in `children` around the `Vec<Rc<Node>>`.

### Adding a Reference from a child to its Parent
To make the child node aware of its parent, we need to add a `parent` field to our `Node` struct definition.

Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well. However, a child should not own its parent: if we drop a child node, the parent should still exist. This is a case for **weak references**!

So instead of `Rc<T>`, we’ll make the type of `parent` use `Weak<T>`, specifically a `RefCell<Weak<Node>>`.

``` rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```


``` rust
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);//why there's a "&" ?

    println!("leaf parent = {:?}", leaf.parent.borrow().upgradde());//why use upgrade() ?
}
```
Once we have the `Node` instance in `branch`, we can modify `leaf` to give it a `Weak<Node>` reference to its parent.

We use the `borrow_mut` method on the `RefCell<Weak<Node>>` in the `parent` field of `leaf`, and then we use the `Rc::downgrade` function to create a `Weak<Node>` reference to `branch` from the `Rc<Node>` in `branch.`

We can also tell this by looking at the values we get from calling `Rc::strong_count` and `Rc::weak_count`.

### Visualizing Changes to `strong_count` and `weak_count`
``` rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
```