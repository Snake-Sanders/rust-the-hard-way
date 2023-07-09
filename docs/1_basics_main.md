- [Basics](#basics)
- [Input \& Output](#input--output)
  - [Using console output: Print](#using-console-output-print)
  - [Using console input: io::stdin](#using-console-input-iostdin)
- [String interpolation](#string-interpolation)
- [String trimming](#string-trimming)
- [Comments](#comments)
- [Code formatter](#code-formatter)

# Basics

A program is a set of instructions executed in sequence on one CPU core. Like a list of steps in a recepy to bake a cake.
The simplest progam is without instructions:

```rust
fn main() {
}
```

In Rust, the `main` function serves as the entry point of the program. When the program is executed, the code inside the `main` function will be executed.

Let's break down the syntax of the `main` function:

- `fn`: This keyword is used to define a function in Rust.
- `main`: The name of the function is `main`. By convention, Rust expects the entry point of the program to be named `main`.
- `()`: The parentheses indicate that the function takes no arguments. In Rust, `()` is the unit type, representing an empty tuple.
- `->`: The arrow (`->`) is used to specify the return type of the function, but you might wonder, where is the arrow? 
  Since no value is returned, the arrow is ommitted, here is the equivalent `fn main() -> () {}`
- `{}`: curly brackets {} are used to define blocks of code. These blocks are also known as scopes or code blocks and are used to group multiple statements together, treating them as a single unit of code

In this example, the `main` function is empty, meaning it doesn't contain any statements or code. It's the simplest possible `main` function that can be compiled and executed successfully.

While this example doesn't demonstrate any practical functionality, it serves as the starting point for building more complex Rust programs. You can gradually add code, define variables, call functions, and perform operations within the `main` function.

# Input & Output

In order to start our learning journey, we will need the escential means to communicate with a program. We want to send information to the program and receive results from it.
This is when `Input` and `Output` come into play. They refer to as interfaces which enabled a user to access the input and output streams in a computer program. These streams are associated with the keyboard and console (or terminal) by default.

- Standard Input (stdin): This is the default input stream for a program. It typically represents the source of data for the program to read. By default, stdin is connected to the keyboard, enabling the program to read input from the user. It allows the user to provide data, such as text or other information, to the program during its execution.

- Standard Output (stdout): This is the default output stream for a program. It represents the destination where the program writes its output or results. By default, stdout is connected to the console or terminal, allowing the program to display text or other data as output. The output can be displayed to the user, redirected to a file, or piped to another program for further processing.


In Rust, the `std::io` module provides functionalities to read from stdin (`std::io::stdin()`) and write to stdout (`std::io::stdout()`). These can be used to interact with the standard input/output streams within a Rust program.

## Using console output: Print
Let add one instruction to our empty main to see something in the console. If you are running this code on the web, the restult panel is showing you the content of the terminal. 


```rust
fn main(){
    print!("hello")
}
```

This will print in the output console the text: `hello`. 

But Hey! where is the `std::io` for sending the greeting to the output console?

Well, `print!` is a macro itself, we will see that later on. This macro is defined in the `std::fmt` module, which is part of the standard library. 
The `print!` macro uses the `std::io::stdout` stream for writing output to the console, but you don't need to import the `std::io` module explicitly to use `print!`.

Let's add a couple of more instructions to it.


```rust
fn main(){
    print!("Oh!..."); print!("Hey");
}
```

Output:

```
Oh!...Hey
```

Notice there is a semicolon `;` after each instruction, this is how we separate them when there are more than one in a code block. 

Let's add a some more.

```rust
fn main(){
    print!("Oh!..."); 
    println!("Hey");
    println!("How are you?");
    println!("Long time no see!");
}
```

Output:

```
Oh!...Hey
How are you?
Long time no see!
```
Puting the instructions in different lines does not make the output to be printed in different lines, this has to do with the type of print instruction we are using. Notice the difference between `print!` and `println!`, the later inserts a new line after printing the content.

We could have written all these three instructions in one line but for the sake of readability it is commonly done in individual lines.

We just have learned the basics of how to use the output to print in the console, now let's see how to read data from the console.

## Using console input: io::stdin

This time we are going to need to bring in the `io::stdin` library to out program. In Rust, there is no built-in macro equivalent to `println!` for input. Reading input from the user typically involves interacting with the `std::io` module and using its functions directly.

In order to have the library functions accesible within our code, we need to tell Rust which full path of the module we want to use.

In this case we use the special keword `use` followed by the library path and name.

```rust
use std::io;
```

`io` is the module name (input and output) and `std` is the name of the package (standard library) that includes `io`.

We use the `std::io::stdin()` function to obtain a handle to the standard input stream, and then use the `read_line()` method to read user input. Here's an example:

```rust
use std::io;

fn main() {
    let mut user_name = String::new();
    println!("Enter your name:");
    // wait for a name
    io::stdin().read_line(&mut user_name).expect("Failed to read user_name");
    println!("Hello, {}!", user_name);
}
```

Wow here a lot goin on. Here is the overview:

1. the line `let mut user_name = String::new();` creates a reference to the standar input interface. To keep it simple, this receives what the user types on the keyboard. All the keystrokes will be available here. 
2. It prints in the console `Enter your name:`
3. `// wait for a name` is just a developer's comment, ignore this for now.
4. the line `io::stdin().read_line...` starts waiting for the user to press some keywords until the user presses Enter.
5. If everything when according to the plan, the user introduced a name and pressed Enter. Now the name is in the variable `user_name` and this is printed in the console output.

```
Enter your name:
John
Hello, John
!
```

**Don't panick!**

We don't want to go too fast, it is ok if most of it does not make sense. The important concept we need is how to read something from the console and how to print something out. This is an important building block for further excersise, we want them to be interative. Everything will be clear at some point in our progress.

`let mut user_name = String::new();`

- `let`: In Rust, the `let` keyword is used to introduce a new variable binding.
- `mut`: The `mut` keyword is short for "mutable" and is used to declare a mutable variable. Mutable variables allow their values to be modified after they are initially assigned.
- `user_name`: This is the name of the variable being declared. You can choose any valid identifier as the variable name.
- `String::new()`: The `String::new()` function creates a new, empty instance of the `String` type. It returns a `String` object, which represents a mutable sequence of characters.

Putting it all together, the line `let mut user_name = String::new();` declares a new mutable variable named `user_name` of type `String` and initializes it with a new, empty `String` object.

The `user_name` variable is then used to store the user input when reading from the standard input stream. By using `String::new()` to create an initially empty `String`, you provide a mutable container that can be updated with the user's input.

In this example, the `std::io::stdin()` function is used to obtain a handle to the standard input stream. Then, the `read_line()` method is called on the input stream to read user input into a mutable `String` variable called `user_name`. Finally, the program outputs a greeting message using the user's input.

Keep in mind that reading input from the user involves error handling as well. In the example above, the `expect()` method is used to handle any potential errors that may occur during the input process. It's a good practice to handle errors appropriately in your own programs.


In the following sections we try to fill some of the gaps, there are a lot of concepts to learn, but we go step by step.

# String interpolation

The line of code, `println!("Hello, {}!", user_name);`, demonstrates string interpolation in Rust.

String interpolation is a technique used to insert or substitute values into a string, allowing you to create dynamic strings that incorporate variable values. In Rust, string interpolation is achieved using the curly brackets `{}` as placeholders within a string literal.

In the example:

- The string literal `"Hello, {}!"` represents the static part of the output message you want to print.
- The curly brackets `{}` serve as placeholders where the value of the `user_name` variable will be inserted.

Here's how it works:
1. The `println!` macro is called with the string literal and the `user_name` variable as arguments.
2. During execution, the `println!` macro replaces the `{}` placeholder in the string literal with the value of `user_name`.
3. The resulting string is printed to the console.

For example, if the `user_name` variable contains the value `"John"`, the output would be:
```
Hello, John!
```

# String trimming

By the way...

You might have noticed that in the previous excersice the exclamation mark after `John` was in a new line. Why is that?
This is because `io::stdin().read_line()` reads the complete line including the enter keystroke.
 
To fix this we could trim the trailing enter with the function, wait for it... trim()

Change the line as follows:

```rust 
println!("Hello, {}!", user_name.trim());
```

now the output looks better:

```
Enter your name:
John
Hello, John!
```

# Comments

Let's introduce the concept of comments before move to the next chapter.

In Rust, you can write comments to provide explanations, document your code, or temporarily disable parts of your code. There are two types of comments you can use:

1. **Line Comments**: Line comments start with `//` and continue until the end of the line. They are used for single-line comments.

   ```rust
   // This is a line comment
   ```

2. **Block Comments**: Block comments start with `/*` and end with `*/`. They can span multiple lines and are used for longer comments or comments that cover a block of code.

   ```rust
   /*
   This is a block comment
   that spans multiple lines
   */
   ```

# Code formatter

In Rust, you can use the `rustfmt` tool to automatically format your code according to the Rust style guidelines. `rustfmt` is a powerful code formatter that helps maintain consistent and readable code across projects. Here's how you can use it:

1. Install `rustfmt`: If you haven't installed `rustfmt` yet, you can do so by running the following command in your terminal:
   ```
   $ rustup component add rustfmt
   ```

2. Format a single file: To format a single Rust source file, run the following command in your terminal:
   ```
   $ rustfmt path/to/your/file.rs
   ```

   This command will format the specified file in-place, modifying the file directly.

3. Format an entire directory: If you want to format all Rust files in a directory and its subdirectories, use the `--recursive` flag:
   ```
   $ rustfmt --recursive path/to/your/directory
   ```

   This command will recursively format all Rust files in the specified directory.

4. Format with cargo: If you prefer to use `cargo` to format your code, you can run the following command in your project directory:
   ```
   $ cargo fmt
   ```

   This command will format all the Rust files in your project according to the Rust style guidelines.

By using `rustfmt` or `cargo fmt`, you can easily format your Rust codebase, ensuring consistent code style and improving code readability. It's recommended to integrate code formatting into your development workflow to maintain a consistent coding style across your projects.
