# Chapter 14 Patterns and Matching
## All the Places Patterns Can Be Used
### `match` Arms
### Conditional `if let` Expressions
### `while let` Conditional Loops
``` rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
	println!("{}",top);
}
```
Method `pop()` returns an `Option<T>`

### `for` Loops
### `let` statement
``` rust
let PATTERN = EXPRESSION
```

## Refutability: Whether a Pattern Might Fail to Match

## Pattern Syntax
### Matching Literals
### Matching Named Variables
### Matching Multiple Patterns by `|`
### Matching Ranges of Values with `..=`
### Destructing to Break Apart Values
#### Destructing Structs
#### Destructing Enums
#### Destructing Nested Structs and Enums
#### Destructing Structs and Tuples
### Ignoring Values in a Pattern
#### Ignoring an Entire Value with  `_`
#### Ignoring Parts of a Value with a Nested `_`
#### Ignoring Remaining Parts of a Value with `..`
### Extra Conditionals with Match Guards
A *match guard* is an additional `if` condition, specified after the *pattern* in a `match` arm, that must also match for that arm to be chosen.
The match guard can use variables created in the match arm.
`if n == y` is NOT a pattern but  an expression. It will NOT consume the ownership of `n` or `y`.
### `@` Bindings


