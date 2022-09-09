# Chapter 5 Packages, Crates, Modules and Cargo
## Concepts:
-   **Crates:** A tree of modules that produces a library or executable
-   **Packages:** A Cargo feature that lets you build, test, and share crates
-   **Modules** and **use:** Let you control the organization, scope, and privacy of paths
-   **Paths:** A way of naming an item, such as a struct, function, or module
-  **Workspace**

### Crate:
A _crate_ is the smallest amount of code that the Rust compiler considers at a time. Even if you run `rustc` rather than `cargo` and pass a single source code file, the compiler considers that file to be a crate. A crate can be compiled into a binary or into a library. By default, `rustc` will produce a binary from a crate. This behavior can be overridden by passing the `--crate-type` flag to `lib`.

Crates can contain modules, and the modules may be defined in other files that get compiled with the crate, as we’ll see in the coming sections.

A crate can come in one of two forms: a binary crate or a library crate. _Binary crates_ are programs you can compile to an executable that you can run, such as a command-line program or a server. Each must have a function called `main` that defines what happens when the executable runs.

_Library crates_ don’t have a `main` function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects.

The _crate root_ is a source file that the Rust compiler starts from and makes up the root module of your crate.

**Note:** think of a crate like a *tree*. The tree is always rooted in a *lib.rs* file or *main.rs* file (and two trees if both *lib.rs* and *main.rs* exists). The compiler only knows how to compile the code from the root of the tree. You have to use `mod` to bring in modules in the package that are not located in the *lib/main.rs* , or modules in other packages.


### Package:
A _package_ is a bundle of one or more crates that provides a set of functionality. 

A package can contain other packages. Each package should be compilable by cargo (because it has a *Cargo.toml* file, which tells the cargo how to tell the compiler how to compile).

### Module:
Rust provides a powerful module system that can be used to hierarchically split code in logical units (modules), and manage visibility (public/private) between them.

A module is a collection of items: functions, structs, traits, `impl` blocks, and even other modules.

### Path:


## Modules
**Note** 
>1. You only need to load a file using a `mod` declaration _once_ in your module tree.
>2. `mod` command actually declares a module. If you have a file named `common.rs` or `common/mod.rs` then


### How modules work?
-   **Start from the crate root**:  the compiler first looks in the crate root file.
-   **Declaring modules**: In the crate root file, you can declare new modules. 
	Say, you declare a “garden” module with `mod garden;`. The compiler will look for the module’s code in these places:
    -   Inline, within curly brackets that replace the semicolon following `mod garden`
    -   In the file _src/garden.rs_
    -   In the file _src/garden/mod.rs_
-   **Declaring submodules**: In any file other than the crate root, you can declare submodules. 
	The way the compiler looks for modules is similar to above.
-   **Paths to code in modules**: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate (as long as the privacy rules allow, using the path to the code). 
-   **Private vs public**: Code within a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations.
-   **The `use` keyword**: Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. 

### Separating Modules into Different Files

## Path: referring to items in the module tree
A path can take two forms:
-   An _absolute path_ is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal `crate`.
-   A _relative path_ starts from the current module and uses `self`, `super`, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (`::`).

### Exposing Paths with the `pub` Keyword
>The module tree should be defined in _src/lib.rs_. Then, any public items can be used in the binary crate by starting paths with the name of the package. The binary crate becomes a user of the library crate just like a completely external crate would use the library crate: it can only use the public API. 
>
>Some packages can have both *lib.rs* and *main.rs* . Typically, packages with this pattern of containing both a library and a binary crate will have just enough code in the binary crate to start an executable that calls code with the library crate.

### Starting Relative Paths with `super`

### Making Structs and Enums Public
 
``` shell
demo
├── add
│   └── src
│       └── main.rs
├── adder
│   ├── identity
│   │   └── src
│   │       └── lib.rs
│   ├── plus
│   │   └── src
│   │       └── main.rs
│   └── src
│       └── lib.rs
├── src
│   └── main.rs
└── target
  
10 directories, 5 files
```

### Bringing Paths into Scope with the `use` keyword

### Creating Idiomatic `use` paths
``` rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
```

### Providing New Names with the `as` Keyword
``` rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

### Re-exporting Names with `pub use`

``` rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```
Now, by adding `pub use` before `crate::front_of_house::hosting`, external code (outside the current *scope* that contains the *item*). 

Exporting a Convenient API with `pub use`

### Using Nested Paths to Clean Up Large `use` Lists
``` rust 
use std::{cmp::Ordering,io::{self, Write}};

```

### The Glob Operator
``` rust
use std::collections::*;
```
This `use` statement brings all **public** items defined in `std::collections` into the current scope.


## Crates
### [Rules of Module Filesystems](http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch07-01-mod-and-the-filesystem.html#rules-of-module-filesystems)
Let’s summarize the rules of modules with regard to files:

-   If a module named `foo` has no submodules, you should put the declarations for `foo` in a file named _foo.rs_.
-   If a module named `foo` does have submodules, you should put the declarations for `foo` in a file named _foo/mod.rs_.

### [Creating a Library](https://doc.rust-lang.org/rust-by-example/crates/lib.html)
### [Using a Library](https://doc.rust-lang.org/rust-by-example/crates/using_lib.html)


## Packages and how Cargo manages them

### External Packages

### Conventions

### Tests
Create a `test` directory to store you code for integration tests.
