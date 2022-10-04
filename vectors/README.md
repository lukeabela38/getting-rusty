# Collections

The Rust standard library include very useful data structures called collections - most other data types represent one specific value, but collections can contain multiple values.

Data in these collections is stored on the heap, which means the amount of data does not need to be known at compile time, and can grow or shrink as the program runs. 

1. Vector: store a variable number of values next to each other.
2. String: A collection of characters - the String.
3. Hash map: associate a value with a particular key. A specific implementation of the more general data structure "map"

## Vectors:

### Creating a Vector

The first collection type is ```Vec<T>``` known as the vector, which allows you to store more than one value in a single data structure which puts all the values next to each other in memory. Vectors can only store values of the same type.

Creating a new vector:
```rust
let v: Vec<i32> = Vec::new();
```

Rust as of yet does not know what kind of elements we intend to store. When we create the vector to hold a specific type, we specify the type within angle brackets - in this case i32.

When you want to create a vector with initial vectors - and Rust will infer the type of value you desire to store.

```rust
let v = vec![1, 2, 3];
```

### Updating a Vector

Create and add a vector:

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

Similar to other variables, we must use the mit keyword.

### Reading Elements of a Vector

Referecing a value stored in a vector by indexing or the get method.

Indexing

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

```

### Iterating over Values in a Vector

```rust
let v = vec![50, 51, 52];
for i in &v {
    println("{}", i);
}
```

For mutable changes:

```rust
let mut v = vec![50, 51, 52];
for i in &mut v {
    *i += 50;
}
```

## Strings:

Strings are implemented as a collection of bytes, plus some functions to work with bytes as text. 

### Creating a new mutable string

```rust
let mut s = String::new();
```

```rust
let data = "initial contents";

let s = data.to_string();

let s = "initial contents".to_string();
```

or

```rust
let s = String::from("initial contents");
```

### Updating a String

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2);
```

### Adding Strings

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;
```

### Indexing into Strings

## Hash Maps

The last of the common collections is the hash map. The type of the HashMap<K, V> stores a mapping of keys of type K to values of type v using a hashing function, which determines how it places these keys and values into memory.

Hasp maps are great for when you want to look up data but not by using an index, but instead by a key. 

### Creating a new hash map

```rust
use std::collections:HashMap;

let mut scores = HaspMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

```

Similar to vectors, hash maps store their data in the heap. Hash maps are homogeneous - i.e. all of the keys must have the same type as each other, and all of the values must have the same type.


### Accessing values in a hash map

```rust
let team_name = String::from("Blue");
let score = scores.get(&team_name):

```

### Iterating over key/value pair in a hash map

```rust
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### Overwriting a Value

```rust
use std::collections::HashMap;

let mut scores = hashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```


### Adding a Key and Value only if key is not Present

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);

```

### Updating a value based on an old value

```rust

use std::collections::HashMap;

let text = "What a wonderful world";

let mut map = HashMao::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

### Hashing Function

By default, the hash map uses a hashing function called SipHash that can provide resistance to Denial of Service attacks involving hash tables. 

## Summary

Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access or modify data. 

