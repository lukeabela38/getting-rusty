# Structs in Rust

A struct or a structure is a custom data type that lets you package together and name multiple related values which make up a meaningful group.

A struct can be thought of as the data attributes of an object. 

## Defining and Instantiating Structs

Structs are broadly similar to tuples, in that both can hold multiple related values. Structs may also hold multiple data types - however with structs we will assign a name to each value ensuring greater clarity.

### Defining a Struct

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

When using a struct after it has been defined, we must create an instance of that struct by specifying concrete values for each of the fields.

```rust 
fn main() {
    let mut user1 = User { email: String::from("someone@example.com"), username: String::from("someone123"), active: true, sign_in_count: 1,
    };
}
```

To retrieve data we can use the the dot notation. Similarly if the data is mutable, we can change the value using the dot notation.

```rust
user1.email = String::from("some@email.any");
```

Notably Structs may be mutable or immutable, however with Structs, individual parts of the data structure may not be individually made mutable or immutable. The whole struct must be mutable or immutable.


### Defining structs through functions:

```rust

fn main() {
    let user1 = build_user(String::from("any@one.any"), String::from("anyone123"),
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

If we want to vary only one field between instantiations of our Struct:

```rust

fn main() {

    let user1 = User {
        email: String::from("any@one.any"),
        username: String::from("any123"), 
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("any@another.really"),
        ..user1
    }
}

```

It should be noted that depending on the nature of the data you copy from user1 to user2, user1 may become invalid. If you transfer data which is only stored on the stack, user1 will continue to be valid. However, data which resides on the heap will still to be transfered to user2, but due to its nature, this will render user1 invalid.

## Structs without named fields

Structs may be declared a more tuple like fashion:

```rust

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

## Unit Structs

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

```
