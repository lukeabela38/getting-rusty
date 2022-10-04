# Generics

Every programming language features tools for effectively handling the duplication of concepts. In Rust, one such tool to handle the duplication of concepts is known as generics.

A generic in Rust is an abstract standin for concrete types or other properties. We can express behaviour of generics or how thez relate to other generics without knowing what will be in their place when compiling and running the code.

Functions take parameters of generic type, instead of the more concrete data types typically discussed such as strings or unsiged 8 bit integers. Think of this in a similar way to functions that pass parameters with unknown values/

Option<T>, Vec<T>, and HashMap<K, V> are all examples of generics! 

## Removing Duplication by Extracting a FUnction

Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.

### Finding the largest Number in a list

```rust

fn main() {
    let number_list = vec![24, 57, 58, 12, 32];
    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }
}

```

A list of integers are stored in a variable and a reference is placed to the first number in the list. We iterate over all the numbers in the list, and if the current number is greater than the number is stored as the largest, replacing the previous reference in the largest variable.

If we wish to find the largest number in two lists, we can either repeat this process, or create a callable function to avoid duplication.


```rust

fn largest(list: &i32) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item:
        }
    }

    return largest:
}
```


The largest function has a parameter called list which represents any concrete slice of i32 values, which we might pass into the function. As a result, when we call the function, the code runs on specific value sthat we pass in.

## Generic Data Types

We can use generics to create definitions for items like function signatures or structs - which we can use with many concrete data types. 

When defining a function that uses generics, we must place generics in the signature of the function where we would typicallz place the data types of the parameters and the return value.

Rewriting the largest function shown earlier: to parameterise the types in a new single function, we need to name the type parameter - let us use T.

```rust
fn largest<T>(list: &[T]) -> &T 
```

We read the function as follows: the function largest is generic over some Type T with a singular parameter named list which accepts a slice of values of type T. The largest function will return a reference to a value of the same type T.

```rust

fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item:
        }
    }

    return largest:
}
```

Notably we now encounter an issue in which the > operator is not necessarily valid for all the possible data types T may be. We may consider restricting T to specific data types. This may be done with the use of a library Rust provides us

```rust
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T
```

This is known as a trait. This specific trait will enable comparisons for specific restricted valid types of T - enabling compilation. In this case PartialOrd is valid for i32 and char

## Defining Structs with Generics

```rust
struct Point<T> {
    x: T,
    y: T,
}
```

Notably, although we are using a generic, we should still ensure the type passed to T is the same for both x and y. i.e. i64 and i64, not i32 and float. If this is something you desire to do, check out multiple generic parameters:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

### Method Definitions

Methods may be implemented on structs and enums

```rust

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
    let p = Point {x: 5, y: 10};

    println!("p.x = {}", p.x());

}
```

## Performance of Code using generics

Using generic types will not make your program slower than when executing with concrete types - Rust accomplishes this through the monoporphization of the code using generic code into specific code by filling in the concrete types that are used when compiled.

# Traits - Defining Shared Behaviour

A trait defines functionality of a particular type and can share with other types. Traits can be used to define shared behaviour in an abstract way - we can use trait bounds to specify that a generic type can be any type that has a specific behaviour.

Note Rust traits are often called interfaces in Java

A type's behaviour consists of the methods we can call on that type. Different types share the same behaviour if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviours necessary to accomplish some purpose.

e.g. a trait
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

The trait is declared using the trait keyword, and then the name of the trait, which in this case is Summary. We have declared this trait as pub, so that crates dependent on this create can make use of this trait too. Inside the curly backets we cans see the method signature which describes of behaviour of the trait implemented by this type.

After the method signature, instead of providing an implementation within the curly bracket, we use a semicolon - each type implementing this trait must provide its own custom behaviour for the body of the method. The compiler will enforce that any type that as the summary trait will have the method summarize defined with this signature exactly.

Implementing a trait on a type . once we declare the desired signature, we can implment it on the a type.

```rust
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

Implementing a trait on a type is similar to implementing regular methods. The difference is that after impl, we put the trait name we want to implement, and then use the for keyword, and then specify the name te want to impkementing the trait for.

```rust

use aggregator::{Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("Hello there"),
        reply: false,
        retweet: true,
    }
}
```

Note that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.

## Creating Default implementations

```rust

pub trait Summary {
    fn summarize_author(&self) -> String;  
    
    //default

    fn summarize(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

impl Summar for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

```

## Lifetimes in Rust

every varibale in rust has a lifetime, which is the scope for which that reference is valid.
Most of the time, lifetimes are implicit and inferred - just like most of the time, types are inferred. We only must annotate types when multiple types are possible.

In a similar way, we must annotate lifetimes when lifetimes of references could be related in a few different ways. Notably, this is not a concept in most other programming languages.

The notion of a lifetime is to prevent dangling references.

## Generic Lifetimes in Functions

If we pass references to stack based variables into a function which returns a similar variable, such as stack based strings, we may find the Rust compiler asking us to include generic lifetime parameters - this is because the compiler is not sure which reference we are asked it to return.

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

This wont compile. When we define the function, we do not know the concrete values that will be passed into the function, so we do not know whether the if or else case will execute. We also do not concretely know the lifetimes of the references that will be passed in, so we cant look at the scope to determine the reference returned will always be valid. 

## Lifetime annotation syntax

Lifetime annotations do not change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. 

Similar to how functions can accept any type when the signature specifies a generic type parameter, functions can accept reference with any lifetime by specifzing a generic lifetime parameter.

```rust
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

## Lifetime Annotations in function signatures

When using life time annotations in function signatures, we want to express that the returned reference will be valid as long as both the parameters are valid.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

In practice, this mandates that the lifetime of teh returned value is as long as the the lifetime of the parameter passed with the shorter lifetime. The important thing to note is that you need to specify lifeparameters for functions.

This is a diminishing requirement, and will be required less and less in future.

Lifetimes work based on 3 rules.

1. The compiler assigns a lifetime parameter to each parameter that is a reference. In other words, a function with one parameters gets one lifetime parameter.

2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.

3. If there are multiple linput lifetime parameters, but one is a reference, the lifetime of the reference variable is assigned to all output lifetime parameters.

Finally, the static lifetime is one which lives for the entire duration of the program.