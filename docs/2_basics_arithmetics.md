- [Basics Arithmetics](#basics-arithmetics)
  - [Expressions](#expressions)
  - [Multiple expresions in a Block](#multiple-expresions-in-a-block)
  - [Examples of Arithmetic operators](#examples-of-arithmetic-operators)
  - [Excercise](#excercise)
    - [String conversion](#string-conversion)
    - [Celsius to Fahrenheit](#celsius-to-fahrenheit)

# Basics Arithmetics

In thi chapter we reviewing some basic concepts of Expressions, fell free to skip the theory if this is something you know. 

## Expressions 

In programming, an expression is a combination of values, variables, operators, and function calls that produces a new value. Expressions can be as simple as a literal value or a variable, or they can be more complex involving operations and function invocations.

literal values like numbers, strings, booleans, and others are expressions on their own. They represent a specific value and can be used directly or as part of larger expressions.

For example, both the number `5` and the string `"hello"` are considered expressions.

Here are examples of how the number `5` and the string `"hello"` can be used as expressions in Rust:

```rust
let x = 5; // Number 5 is an expression assigned to variable x

let message = "hello"; // String "hello" is an expression assigned to variable message

let sum = 3 + 2; // The expression 3 + 2 performs addition and the result is assigned to variable sum

println!("The value is: {}", 5); // The expression 5 is used as an argument to the println! macro
```

In each of these examples, `5` and `"hello"` are standalone expressions that represent specific values. They can be used in assignments, function arguments, mathematical operations, and other contexts just like any other expression in Rust.

Expressions play a fundamental role in Rust programming as they combine values, operators, and function calls to produce new values or perform operations.


In Rust, expressions follow a syntax that allows you to combine various elements to compute and produce a resulting value. Here are a couple of examples of expressions in Rust:

1. **Arithmetic Expression**:
```rust
let x = 5 + 3 * 2;
```
In this example, the expression `5 + 3 * 2` involves arithmetic operations (`+` and `*`) along with literal values (`5` and `3`). The result of this expression is computed and assigned to the variable `x`.

1. **Function Call Expression**:
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

let result = add(10, 20);
```
In this example, the expression `add(10, 20)` is a function call expression. It calls the function `add` with arguments `10` and `20`. The returned value from the function call is assigned to the variable `result`.

3. **Conditional Expression**:
```rust
let x = 5;
let y = if x > 0 { "positive" } else { "negative" };
```
In this example, the expression `if x > 0 { "positive" } else { "negative" }` is a conditional expression. It evaluates the condition `x > 0` and based on the result, either returns the string `"positive"` or `"negative"`. The resulting value is assigned to the variable `y`.

Expressions can be used in various contexts, such as assignments, function arguments, return statements, and more. Rust's expressive syntax allows you to create complex expressions by combining different elements according to the language's rules.

## Multiple expresions in a Block

In Rust, expressions can be separated by semicolons (`;`) within a block, but it's not always required or recommended. The presence or absence of a semicolon at the end of an expression has significance in Rust.

1. **With Semicolon**: When an expression is followed by a semicolon, it becomes a statement. Statements do not produce a value and are typically used for their side effects (e.g., performing an action, like assigning a value to a variable or calling a function). The result of the expression is discarded.

Here's an example:

```rust
let x = 5;
let y = {
    let z = 3;
    z + 1; // Expression as a statement, no value is returned
};
```

In this case, `z + 1;` is an expression used as a statement within the block. The value of `z + 1` is computed but discarded because the statement doesn't assign or use the result.

2. **Without Semicolon**: When an expression is not followed by a semicolon, it is considered an expression statement. Expression statements evaluate the expression and produce a value, which can be used or returned.

Here's an example:

```rust
let x = 5;
let y = {
    let z = 3;
    z + 1 // Expression statement, value is returned
};
```

In this case, `z + 1` is an expression statement. The value of `z + 1` is the last expression in the block and is implicitly returned as the value of `y`. It is not followed by a semicolon.

It's important to note that in Rust, blocks are expressions themselves, and the value of a block is the value of the last expression within it.

While semicolons are typically used to separate statements in Rust, it's crucial to understand the distinction between expression statements and statements that discard the result.

## Examples of Arithmetic operators

Here's an example showcasing the use of each arithmetic operator in Rust:

```rust
fn main() {
    let a = 10;
    let b = 5;

    // Addition
    let sum = a + b;
    println!("Sum: {}", sum);

    // Subtraction
    let difference = a - b;
    println!("Difference: {}", difference);

    // Multiplication
    let product = a * b;
    println!("Product: {}", product);

    // Division
    let quotient = a / b;
    println!("Quotient: {}", quotient);

    // Remainder (Modulus)
    let remainder = a % b;
    println!("Remainder: {}", remainder);
}
```

In this example, we have two variables `a` and `b` initialized with the values `10` and `5`, respectively. Each arithmetic operator is used to perform the corresponding operation on these variables.

The output of the program will be:

```
Sum: 15
Difference: 5
Product: 50
Quotient: 2
Remainder: 0
```

This demonstrates the basic usage of arithmetic operators in Rust, including addition (`+`), subtraction (`-`), multiplication (`*`), division (`/`), and remainder/modulus (`%`).

## Excercise 

### String conversion

We need to learn how to convert the console input to numbers, it will be usefull for later excerises.

Lets create a program that asks the user for a number and the program responds with a bigger number.

We already know how to ask for user's inputs.
You don't need to know by hard how to read the console (for now), just go back to the basic excerses and copy it from there.


Now, let's focus on the parsing, we need to convert the data from the user, 
which comes as `String`, we need to converte it to integer in order to do arithmetic operations.


```rust
use std::io;

fn main() {
    println!("Introduce a number:");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read the number");

    let trimmed_number = user_input.trim();
    let my_number: i32;

    // Convert the input to an integer
    if let Ok(parsed) = trimmed_number.parse::<i32>() {
        my_number = parsed + 1;
    } else {
        println!("Invalid input. Please enter a valid number.");
        return;
    }

    println!("I have {}, you lose!", my_number);
}
```

Output:

```
Introduce a number:
9
I have 10, you lose!
```

How it works:

1. We assign the trimmed input to the trimmed variable.
2. We declare the number variable without initializing it yet. 
3. We use the `if` let construct to attempt parsing the trimmed string as an `i32`. If parsing is successful, the parsed value is assigned to `parsed`, we increment the value in one, and then assign it to the `my_number` variable . Otherwise, we display an error message and exit the program.
4. Finally, we print the parsed number.

Note that the if let construct is a shorthand way to handle the Result type. It allows you to handle the success case (Ok) and optionally the error case (Err) within a single if statement.


### Celsius to Fahrenheit

Write a program that calculates the conversion of Celsius to Fahrenheit. 
. Now we are going to ask the user for 
a value in Celsius and the program will convert it and print the result in Fahrenheit.

Here is the converison formula: `°F = (°C * 9/5) + 32`

```rust
use std::io;

fn main() {
    println!("Introduce a temperature in Celsius:");
    let mut celsius_string = String::new();

    io::stdin()
        .read_line(&mut celsius_string)
        .expect("Failed to read celsius");

    let trimmed_celsius = celsius_string.trim();
    let fahrenheit: i32;

    // Convert the input to an integer
    if let Ok(parsed) = trimmed_celsius.parse::<i32>() {
        fahrenheit = (parsed * 9 / 5) + 32;
    } else {
        println!("Invalid input. Please enter a valid number.");
        return;
    }

    println!("Temperature in Fahrenheit: {}", fahrenheit);
}
```

Output:

```
Introduce a temperature in Celsius:
18
Temperature in Fahrenheit: 64
```

If we enter an invalid number, for example a letter, we get the warning

```
Introduce a temperature in Celsius:
k
Invalid input. Please enter a valid number.
```

It is common to see the code above with some simplifications, to avoid using extra variables.

```rust
use std::io;

fn main() {
    println!("Introduce a temperature in Celsius:");
    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read celsius");

    // Convert the input to an integer
    let celsius: u32 = match celsius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    println!("Temperature in Fahrenheit: {}", (celsius * 9 / 5) + 32)
}
```

When using the rust code autoformatter, the code might be break down by the `.`
as in 

```rust 
io::stdin()
    .read_line(&mut celsius)
    .expect("Failed to read celsius");
```
