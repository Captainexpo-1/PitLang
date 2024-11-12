# PitLang Documentation

## Introduction

PitLang is a simple, interpreted programming language designed for educational purposes and small scripting tasks. It features a straightforward syntax and supports basic programming constructs such as variables, functions, conditionals, loops, and arrays.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Syntax](#syntax)
3. [Data Types](#data-types)
4. [Variables](#variables)
5. [Functions](#functions)
6. [Control Flow](#control-flow)
7. [Arrays](#arrays)
8. [Standard Library](#standard-library)
9. [Examples](#examples)

## Getting Started

To run a PitLang script, use the following command:

```sh
cargo run -- <script.pit> -eval
```

## Syntax

PitLang syntax is inspired by JavaScript and Python. Here are some basic rules:

- Statements end with a semicolon (`;`).
- Blocks of code are enclosed in curly braces (`{}`).
- Comments start with `//` for single-line comments.

## Data Types

PitLang supports the following data types:

- `Number`: Floating-point numbers.
- `Boolean`: `true` or `false`.
- `String`: Text enclosed in double quotes (`"`).
- `Array`: Ordered collections of values.
- `Null`: Represents the absence of a value.

## Variables

Variables are declared using the `let` keyword:

```rust
let x = 10;
let name = "PitLang";
```

## Functions

Functions are declared using the `fn` keyword:

```rust
fn add(a, b) {
    return a + b;
}
```

Functions can be called with arguments:

```rust
let result = add(5, 3);
```

## Control Flow

### If Statements

```rust
if (condition) {
    // code to execute if condition is true
} else {
    // code to execute if condition is false
}
```

### While Loops

```rust
while (condition) {
    // code to execute while condition is true
}
```

## Arrays

Arrays are ordered collections of values:

```rust
let numbers = [1, 2, 3, 4, 5];
```

Arrays support methods like `push`, `get`, and `set`:

```rust
numbers.push(6);
let first = numbers.get(0);
numbers.set(1, 10);
```

## Objects

Objects are collections of key-value pairs:

```rust
let person = {
    name: "John",
    age: 30,
    greet: fn(this) { // `this` needs to be passed as an argument
        std.print("Hello, my name is " + this.name);
    },
    birthday: fn(this) {
        this.age = this.age + 1;
        std.print("Happy Birthday! I am now " + this.age.to_string() + " years old.");
    }
};

std.print(person.name); // "John"
std.print(person.age); // 30

// Need to pass `person` as an argument to methods because `this` is not automatically bound to the object
person.greet(person); // "Hello, my name is John"
person.birthday(person); // "Happy Birthday! I am now 31 years old."
person.birthday(person); // "Happy Birthday! I am now 32 years old."
```

## Standard Library

PitLang includes a small standard library with useful functions:

### `std.print`

Prints values to the console:

```rust
std.print("Hello, World!");
std.print("1",2,"More",[4,5,6]) // Can print multiple values
```

### `std.time`

Returns the current time in seconds since the Unix epoch:

```rust
let currentTime = std.time();
```

### `std.random`

Returns a random number between 0 and 1:

```rust
let randomNumber = std.random();
```

## Examples

### Prime Number Sieve

```rust
fn sieve(n) {
    let is_prime = [];
    let i = 0;
    while (i <= n) {
        is_prime.push(true);
        i = i + 1;
    }
    is_prime.set(0, false);
    is_prime.set(1, false);
    let p = 2;
    while (p * p <= n) {
        if (is_prime.get(p)) {
            let multiple = p * p;
            while (multiple <= n) {
                is_prime.set(multiple, false);
                multiple = multiple + p;
            }
        }
        p = p + 1;
    }
    let primes = [];
    let num = 2;
    while (num <= n) {
        if (is_prime.get(num)) {
            primes.push(num);
        }
        num = num + 1;
    }
    return primes;
}
let primes_up_to_100 = sieve(100);
std.print(primes_up_to_100);
```
