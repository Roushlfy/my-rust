### 1. `String` and `&str`
#### 1.1 Declaration and Instantiation
`String` is a struct in the std lib.
``` rust
// create an owned String type
let s1 = String::from("hello");
// s1: String(String类), with its ownership, immutable
```
`&str` (slice) doesn't own its value. A *String slice* `&str` is a reference to part of a `String`.
```rust
let data = "hello world";
// data: &str(字符串切片/slice)
```
#### 1.2 Conversion between `String` and `&str`
``` rust
let s2 = data.to_string();
// s2: String
// s2 has the owner ship of its content
// data is still valid
// what to_stgring() performs here is: takes a reference(borrow) of data, reate a copy(not ref) and return both its value adn its ownership

let hello = &data[..5];
let world = &data[6..11];
```
`hello` and `world` are both references to a portion of the `String`
``` rust
let len = hello.len();
//len() takes in &String and return a basic data type —— u32
let world_new = &data[6..len]
// world_new is the same as world
// len is still valid here !
```

#### 1.3Operation on `String` and `&str`

Now consider functions that call on `String` or `&str`
``` rust
fn some_function(s: String) -> &str {}
```
Do NOT do this ! This will lose the ownership of `s`.
NOR should you do this:
``` rust
fn some_function(s: &String) -> &String {}
```
Or you can specify the lifetime of the returned `&String`
Instead, use the following:
``` rust
fn some_function(s: &String) -> &str {}
```
so that we only *borrows* `s` . Or you can perform on `&str` :
``` rust
fn some_function(s: &str) -> &str {}
```