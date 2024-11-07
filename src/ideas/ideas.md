Ideas for language

initially it is a simple language that is easy to learn and use and is evaluated with a tree walk interpreter

it is a dynamically typed language

it is a functional language

```rust
// Example code
let x = 10;
let y = 20;

fn add(a, b) {
    return a + b;
}

let z = add(x, y);

print(z);

fn fibonacci(n){
    if n <= 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

```

tokens

- `let` - declare a variable
- `fn` - declare a function
- `return` - return a value from a function
- `if` - if statement
- `else` - else statement
- `()` - parenthesis
- `{}` - curly braces
- `;` - semicolon
- `+` - addition operator
- `-` - subtraction operator
- `*` - multiplication operator
- `/` - division operator
- `=` - assignment operator
- `==` - equality operator
- `!=` - not equal operator
- `<=` - less than or equal operator
- `>=` - greater than or equal operator
- `<` - less than operator
- `>` - greater than operator
- `&&` - and operator
- `||` - or operator
- `!` - not operator
