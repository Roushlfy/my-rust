# Chapter 10 Cargo and Crates.io
## Customizing Builds with Release Profiles
Cargo has two main profiles: the `dev` profile Cargo uses when you run `cargo build` and the `release` profile Cargo uses when you run `cargo build --release`. The `dev` profile is defined with good defaults for development, and the `release` profile has good defaults for release builds.

For example:
```rust
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
The `opt-level` setting controls the number of optimizations Rust will apply to your code, with a range of 0 to 3. Applying more optimizations extends compiling time, so if you’re in development and compiling your code often, you’ll want fewer optimizations to compile faster even if the resulting code runs slower. The default `opt-level` for `dev` is therefore `0`. When you’re ready to release your code, it’s best to spend more time compiling. You’ll only compile in release mode once, but you’ll run the compiled program many times, so release mode trades longer compile time for code that runs faster. That is why the default `opt-level` for the `release` profile is `3`.

### Making Useful Documentation Comments
Comments in Rust code begin with `//`. Documentation comments begin with `///`.
What's the difference between comments and documentation comments?
In a nutshell, 
### Commonly Used Sections
- Panics: if the function will cause a panic, specify it in the documentation.
- Errors: if the function returns a `Result`, specify the kinds of errors that might occur.
- Safety: if the function is unsafe to call, write a section explaining why.
### Documentation Comments as Tests

### Commenting Contained Items
The style of `//!` means we are documenting the item that **contains** this comment rather than an item that follows this comment. 
E.g., in the *src/lib.rs* file, these comments describe the entire crate.

### Exporting a Convenient Public API with `pub use`
### Adding Metadata to a New Crate
### Deprecating Versions from Crates.io with `cargo yank`

## Cargo Workspaces