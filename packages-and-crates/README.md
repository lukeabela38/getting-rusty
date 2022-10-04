# Packages and Crates

A crate is the smallest amount of code the rust compiler can consider at any one point. A single source code file can be considered to be a crate. A crate may contain modules, and this modules may be defined in other files which are then compiled with the crate.

The crate may be a binary crate or a library crate. Binary crates are programs you can compule to an executable that you can then run - think of a command line argument program. Each must have a function called main which defines what happens when the executable runs.

Library crates do not have a main function, and they dont compile to an executable - instead they define functionality to be used in multiple projects. 

A crate can be equated to the general programming concept of a library.

The rate root is a source file that the rust compiler starts from and makes up the root module of your crate. A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file which describes how to build the crates.

Cargo itself is a package that contains the binary crate for the cli tool used to build rust code. Cargo also contains a library crate that the binary crate depends on.

A package can contain as many binary crates as possible, but at most only one library crate. A package must contain at least one crate.

When running cargo new, we note a Cargo.toml file, which follows a convention in which src/main.rs is the crate root of a binary crate with the same name as the package - likewise crate knows that if the package contains src/lib.rs, the package contains a library crate with lib.rs the root crate.

## Modules to control Scope and Privacy

When compiling a crate, the compile looks for src/main.rs and src/lib.rs for code to compile. In the crate root file, you can declare modules. The compiler will look for the module's code.

An any other file than the crate root, you can declare submodules. (i.e. modules in modules)

Once a module is part of the crate, you can refer to the code in that module from anywhere else in that same crate, as long as the privacy rules allow using the path to the code. 

Private vs public: code within a module is private from its parent modules - to make a module public declare it using pub mod.

The use keyword within a scope creates shortcuts to items to reduce repetition of long paths.

## Grouping related code in modules

modules let us organise code within a crate for readability and easy reuse. Modules also allow us to control the privacy of items because code within a module is prive by default. Private items are internal implementation details not available for outside use. We can choose to make modules and items within them public, which exposes them to allow external code to use and depend on them.

Let us create a library!
```bash
cargo new --lib restaurant;
```

We can note th e presence of src/lib.rs
Check out [here](rust-stuff/restaurant/src/lib.rs)

## Paths for Referring to Items in the Module Tree

To show Rust where to find an item in a module tree, we can use a path in the same way we find a path when navigating a filesystem - to call a function, we need to know its path.

A path may be absolute, or relative.
An absolute path is the full path starting from a crate root, for code from an external crate, the absolute path begins with the crate name, and for the code from the current crate, it starts with the literal crate,

A relative path starts from the current module and uses self, super, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by doubel colons (::)

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    create::front_of:house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
```

Note the use of the pub keywords.

Relative Paths with Super

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // go into parent module, and look for deliver_order functionality
    }

    fn cook_order() {}
}
```