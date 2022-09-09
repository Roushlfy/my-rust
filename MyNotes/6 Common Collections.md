### 6.1 Vectors `Struct std::vec::Vec`

#### 6.1.1 Creating a New Vector
There are three common ways to create a new `vector`
``` rust
let v1 = vec![1,2,3];

let v2 = Vec::new();
v2.push(1);
v2.push(2);
v2.push(3);

let v3 = Vec::from([1,2,3]);
```
Creating a `Vec` with same elements 
``` rust
// Method 1
let v4 = vec![0;5];
// Method 2
let v5 = Vec::with_capacity(5);
v5.resize(5,0);
```

#### 6.1.2 Vector operations — indexing
``` rust
let v6 = vec![1,2,3,4];
println!("{}",v[0]); // prints 2 to the stdout

println!("{}",v[4]);// this will cause a panic!
```

Use `get()` and `get_mut()` 
``` rust

```

#### 6.1.3 Vector operations — slicing
to get a *slice*, use `&`
``` rust
fn operate_on_slice(slice: &[usize]) {
	//pass in a slice of vector
	// slice does not own the vector
}
let v7 = v6.clone();
operate_on_slice(&v7);

// another way
let u: &[usize] = &v7;
// yet another way
let u: &[_] = &v7;
```
In Rust, it's more common to pass **slices** as arguments rather than vectors when you just want to provide **read access**.
The same logic goes for `Sting` and `&str` .

### 6.2 HashMap

#### 6.2.1 Creating a New Hash Map

### 6.2.2 Accessing Values in a Hash Map
Function `.get(&key)` returns the value bound to the key
```rust
for (key, value) in &HashMap {
	key;
	value;
}
```
#### 6.2.3 Hash Maps and Ownership

#### 6.2.4 Updating a Hash Map
1. Overwriting a Value
2. Adding a Key and Value Only if a Key Isn't Present
3. Updating a Value Based on the Old Value
#### 3.5 Iteration on Hash Maps