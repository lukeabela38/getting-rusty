# Functional Programming in Rust

Rust is inspired by many languages, including functional programming. Programming in a functional style includes using functions as values by passing them in arguments, and returning them from other others, assigning them to variables to be executed later on, and so on.

## Closures

In Rust, closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place, and then call the closure somewhere else to evaluate it in a different context. unlike functions, closures can capture values from the scope in which they are defined. These are great for code reuse!

## Capturing the envrironment with Closures
