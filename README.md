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
cargo run --release -- <script.pit> arg1 arg2 ...
```

Alternatively, you can build the project and run the executable directly:

```sh
cargo build --release
./target/release/pitlang <script.pit>
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
- `Object`: Collections of key-value pairs. Similar to dictionaries in Python or objects in JavaScript.

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
if condition {
    // code to execute if condition is true
} else if another_condition {
    // code to execute if another_condition is true
} else {
    // code to execute if all conditions are false
}
```

### Loops

```rust
while condition {
    // code to execute while condition is true
}
```

```rust
for let i = 0; i < 5; i = i + 1 {
    // code to execute for each iteration
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

### Standard Methods

- `std.time()`: Returns the current time in seconds since the Unix epoch.
- `std.random()`: Returns a random number between 0 and 1.
- `std.print(...)`: Prints values to the console.
- `std.println(...)`: Prints values to the console with a newline at the end.
- `std.argv()`: Returns the command line arguments as an array of strings.
- `std.get_line()`: Reads a line from stdin.
- `std.write_file(filename, content)`: Writes the content to the specified file.
- `std.read_file(filename)`: Reads the contents of the specified file.
- `std.exit(code)`: Exits the program with the given exit code.

### String Methods

- `str.to_string()`: Converts a value to a string.
- `str.to_number()`: Converts a string to a number.
- `str.length()`: Returns the length of a string.
- `str.split(separator)`: Splits a string into an array of substrings using the specified separator.
- `str.trim()`: Removes whitespace from the beginning and end of a string.
- `str.replace(old, new)`: Replaces occurrences of the old substring with the new substring.
- `str.find(substring)`: Returns the index of the first occurrence of the substring in the string, or -1 if not found.

### Array Methods

- `arr.push(value)`: Adds a value to the end of the array.
- `arr.pop()`: Removes and returns the last element of the array.
- `arr.get(index)`: Returns the value at the specified index.
- `arr.set(index, value)`: Sets the value at the specified index.
- `arr.length()`: Returns the length of the array.
- `arr.find(value)`: Returns the index of the first occurrence of the value in the array, or -1 if not found.
- `arr.copy()`: Returns a copy of the array.

### Number Methods

- `num.to_string()`: Converts the number to a string.
- `num.round()`: Rounds the number to the nearest integer.
- `num.floor()`: Rounds the number down to the nearest integer.
- `num.ceil()`: Rounds the number up to the nearest integer.

## Examples

### Hello World

```rust
std.print("Hello, World!");
```

### FizzBuzz

```rust
fn fizzbuzz(n) {
    for let i = 1; i <= 100; i = i + 1 {
        if i % 3 == 0 && i % 5 == 0 {
            std.print("FizzBuzz");
        } else if (i % 3 == 0) {
            std.print("Fizz");
        } else if (i % 5 == 0) {
            std.print("Buzz");
        } else {
            std.print(i);
        }
    }
}
```

### Sieve of Eratosthenes

```rust
fn sieve(n) {
    let is_prime = []; // Initialize an empty array to store prime status
    let i = 0;
    while i <= n {
        is_prime.push(true); // Assume all numbers are prime initially
        i = i + 1;
    }
    is_prime.set(0, false); // 0 is not a prime number
    is_prime.set(1, false); // 1 is not a prime number
    let p = 2;
    while p * p <= n {
        if is_prime.get(p) { // If p is a prime
            let multiple = p * p;
            while multiple <= n {
                is_prime.set(multiple, false); // Mark multiples of p as not prime
                multiple = multiple + p;
            }
        }
        p = p + 1;
    }
    let primes = []; // Initialize an empty array to store prime numbers
    let num = 2;
    while num <= n {
        if is_prime.get(num) { // If num is a prime
            primes.push(num); // Add num to the primes array
        }
        num = num + 1;
    }
    return primes.get(-1); // Return the last prime number in the array
}
let primes_up_to_100 = sieve(100); // Find all prime numbers up to 100
std.print(primes_up_to_100); // Print the largest prime number up to 100
```
