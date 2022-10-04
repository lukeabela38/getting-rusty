# ENUMS

Structs allow the grouping together of related fields and data - e.g. a rectangle with width and height.

Enums are different. Enums express the idea of having a set of values, and one of those values being true.

E.g. a child's game: a child is presented with an object and must guess the shape. The shape is one of a circle, triangle, or rectangle.

The grouping of circle, triangle, and rectangle is the enum. In such situations, we enumerate over all possibilities and hence ENUM.

## Declaring Enums

```rust
enum Shapes {
    rectangle,
    circle,
    triangle,
}
```


Shapes is now a custom data type we can use elsewhere in our code!

```rust
let shape1 = Shapes::rectangle;
let shape2 = Shapes::circle;
```

## Enums and Functions

As Shapes is now a data type, we can use it to pass parameters into functions:

```rust
fn which(myshape: Shapes) {}
```

You can call this in a variety of ways!

```rust
which(Shapes::rectangle);
which(Shapes::circle);
which(Shapes::triangle);
```

## Storing data inside an Enum

```rust
enum Shapes {
    rectangle(usize, usize),
    circle(usize),
    triangle(usize, usize),
}

let rec1 = Shapes::rectangle(4, 4);
```

## Enums Applied

```rust
enum Message {
    Quit, // no associated data
    Move {x: i32, y: i32}, // named fields similar to struct
    Write(String), // includes a single string
    ChangeColor(i32, i32, i32), //includes three i32 values
}

impl Message {
    fn call(&self) {
        // define method body here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## Expressing Null Values using Enums

The following is defined in the standard library

```rust 
enum Option<T> {
    None,
    Some(T),
}
```

```rust
let some_number = Some(5);
let some_char = Some('e');

let absent_number: Option<i32> = None;
```

The option enum is able to infer the data type from the data we pass to it.
Option<T> is generally considered to be superior to an explicit None value.

When declaring variables such as i32, the Rust compiler will always ensure there is a value that is not none present. We can proceed with confidence not having to check for null as we typically would have to in C.

It is only when using option that we have to worry about about not having a value - the compuler will ensure the case is handled.

When working with a value which may be T, you must proceed by using the Option Enum - and then explicitly handle the case in which the value may be null.

### Match Control Flow Construct

Rust has an extremely powerful control flow construct known as MATCH.

This allows us to compare a value against a series of patterns and then execute the code based on the pattern matches.

Patterns can be made up of literal values, variable names, wildcards and others.

```rust

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 { // returning a non boolean type

    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

```

We list the match keyword followed by an expression, which in this case is the value coin. As opposed to if, which must return a boolean value, the match operator can return any type.

Next are the match arms, each arm is composed of some pattern and the code. Each arm is separated from one another via commas.

When the match expression executes, the resulting value is compared against each pattern, in order - if a match is found, the code associated with the matched arm is returned.

## Patterns that bind to values

```rust
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

```

### Handling other uses cases

```rust

let dice_roll = 9;
match dice_roll {
    3 => add_hat(),
    other => move_player(other),
}

```

```rust

let dice_roll = 9;
match dice_roll {
    3 => add_hat(),
    _ => (), // do nothing
}

```

## Control Flow with if let

The if let szntax allows zou to combine if and let into a less verbose waz of handling values that match one pattern whilst ignoring the rest. 

Consider the following:

```rust 
let config_max = Some(3u8):
match config_max {
    Some(max) => prinln!("The maximum is configured to be {}", max),
    _ => (),

}
```

We can simplify this by writing it as follows:

```rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
}
```

We can consider the if let syntax to be syntax sugar for a match that runs runs code when the value matches one pattern, but then ignores all the others.

Notably, since it is an if statement, you can also include else statements.