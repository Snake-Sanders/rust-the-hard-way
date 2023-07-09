- [Introduction](#introduction)
- [Rust Ecosystem](#rust-ecosystem)
- [Installation](#installation)
- [What is a language](#what-is-a-language)
  - [Examples](#examples)
  - [Keywords](#keywords)
  - [Symbols](#symbols)
  - [Types](#types)


# Introduction

Is it Rust the right tool for your project? Only you have the answer to that question. 
To help you out, here's a short list of scenarios where using Rust language is beneficial, as well as cases where it may not be the most suitable choice:

When to use Rust:

- **Systems Programming**: Rust excels in systems programming, providing memory safety, low-level control, and high performance. It is suitable for building operating systems, device drivers, embedded systems, and other performance-critical applications.

- **Concurrency and Parallelism**: Rust's ownership and borrowing system ensures thread safety and allows for efficient concurrent programming. It is well-suited for developing highly concurrent and parallel systems, such as servers, networking applications, and data processing pipelines.

- **Safety-Critical Applications**: Rust's emphasis on memory safety and lack of null pointer dereferences and data races make it a suitable choice for safety-critical applications, such as aviation, automotive, and medical systems.

- **WebAssembly**: Rust is gaining popularity for WebAssembly (Wasm) development due to its ability to generate efficient and secure code for running in web browsers. It is used in areas like browser extensions, online gaming, blockchain, and more.

- **Cross-Platform Development**: Rust's focus on zero-cost abstractions and broad platform support enables cross-platform development. It is well-suited for creating applications that run seamlessly on multiple platforms, including Windows, macOS, Linux, and embedded systems.

When not to use Rust:

- **Rapid Prototyping or Small Scripts**: Rust's focus on safety and performance can make it less suitable for rapid prototyping or writing small scripts. Other languages with faster development cycles may be more appropriate for these scenarios.

- **Existing Ecosystem Dependencies**: If your project heavily relies on existing libraries or frameworks that are not available or have limited support in Rust, it might be more practical to use a language that offers better integration with those dependencies.

- **Highly Dynamic Applications**: Rust's emphasis on static typing and compile-time checks may introduce some additional development overhead for highly dynamic or exploratory applications where flexibility and rapid iteration are more important than performance and safety.

Ultimately, the decision to use Rust depends on your project requirements, the problem domain, the development team's expertise, and the trade-offs you are willing to make. Considering the unique features and strengths of Rust, it is well-suited for a wide range of use cases, particularly those that prioritize performance, safety, and concurrency.

# Rust Ecosystem

Rust is not just the language, there is a whole ecosystem for Rust development. 
Here are some key elements:

1. **Cargo**: The package manager and build system for Rust. Cargo handles dependency management, project scaffolding, building, testing, and documentation generation.

2. **Rustc**: The Rust compiler. It translates Rust source code into executable binaries or libraries. It's invoked by Cargo during the build process.

3. **Crates**: The units of code distribution in Rust. Crates are packages that contain Rust source code, libraries, or executables. They can be published to the central package registry called crates.io.

4. **crates.io**: The official package registry for Rust. It hosts a vast collection of crates that can be easily installed and used in Rust projects.

5. **Standard Library**: The core library bundled with the Rust distribution. It provides essential data types, APIs, and functionality that are available to every Rust program by default.

6. **Documentation**: The Rust ecosystem emphasizes documentation. Rust provides a built-in documentation tool called `rustdoc` that generates API documentation from code comments in a standardized format. Documentation is also available for crates published on crates.io.

7. **Rustup**: The Rust toolchain installer and version manager. Rustup allows you to install, manage, and switch between different versions of the Rust compiler and associated tools.

8. **Testing**: Rust has a built-in testing framework with the `#[test]` attribute, allowing you to write unit tests for your code. Tests can be run using the `cargo test` command.

9. **IDE Support**: Rust has support for various Integrated Development Environments (IDEs) and editors like Visual Studio Code (with the Rust extension), IntelliJ IDEA (with the Rust plugin), and others. These tools provide code completion, linting, and other features to enhance the development experience.

10. **Rustfmt**: A tool for formatting Rust code according to the community-driven Rust style guidelines. It helps maintain consistent code formatting across projects.

11. **Clippy**: A linter for Rust that provides additional static analysis and linting checks beyond what the compiler offers. Clippy helps identify potential bugs, performance issues, and style improvements in your code.

These elements form the foundation of the Rust ecosystem, supporting efficient development, code sharing, documentation, testing, and code quality. They contribute to the productivity and reliability of Rust programming.

# Installation

This chapter will not explain how to install Rust, for that it is better to check the latest instructions on the official Rust page: www.rust-lang.org.

If you want to follow along the execices on this book without installing Rust on your computer, there is an online editor where you can try out the code snipped.

Go to https://play.rust-lang.org/, play around and then, if you are convinced, install Rust on your local machine.

# What is a language

Before we start writing a program, you need know what a language is.

A programming language is defined by its syntax and semantics. The syntax defines the rules for how programs are written in the language, specifying the structure and arrangement of valid code elements. The semantics define the meaning and behavior of those code elements, describing how they are executed or evaluated.

To build a programming language, a typical approach involves multiple stages, including lexing, parsing, and semantic analysis:

1. **Lexing (Tokenization)**: The first step is to break the source code into meaningful units called tokens. This process is performed by a lexer (also known as a tokenizer). The lexer scans the source code character by character and groups them into tokens based on predefined patterns. Tokens represent the smallest individual units of the language, such as keywords, identifiers, operators, and literals.

2. **Parsing**: Once the code is tokenized, the next step is parsing. Parsing is the process of analyzing the structure of the code according to the language's grammar rules. A parser takes the stream of tokens and builds an abstract syntax tree (AST) that represents the hierarchical structure of the code. The AST captures the relationships between different code elements and their precedence.

3. **Semantic Analysis**: After parsing, the AST is subjected to semantic analysis. This stage involves checking the correctness and meaning of the code beyond its structure. Semantic analysis enforces rules related to variable declarations, type checking, scoping, and other language-specific rules. It helps ensure that the code adheres to the language's semantics and can be executed correctly.

4. **Code Generation/Execution**: Once the code passes the semantic analysis, it can be further processed for execution or code generation. This step involves transforming the code into a runnable form, such as generating machine code, bytecode, or intermediate representation. The generated code can then be executed by a runtime system or virtual machine specific to the language.

Note that these stages can vary in complexity and can involve additional steps depending on the design of the language and the goals of the compiler or interpreter. Additionally, modern language implementations often involve optimization steps, such as code optimization or just-in-time compilation, to improve the performance of the executed code.

Building a programming language requires careful consideration of the language's design, syntax, and semantics. It involves creating tools like lexers and parsers to process the source code, followed by analyzing the code's meaning and transforming it into executable form.

## Examples

Here's a simple example in Rust that demonstrates a token, an abstract syntax tree (AST), and a glimpse of the generated code:

1. **Token** (Lexer Output):
Let's consider the following Rust code snippet:

```rust
let x = 5 + 3;
```

The lexer breaks this code into tokens:

```
Token::Let
Token::Identifier("x")
Token::Equals
Token::IntegerLiteral(5)
Token::Plus
Token::IntegerLiteral(3)
Token::Semicolon
```

In this case, each token represents a meaningful unit in the code, such as the `let` keyword, identifier (`x`), equals sign (`=`), integer literals (`5` and `3`), and a semicolon (`;`).

2. **Abstract Syntax Tree**:
The AST represents the hierarchical structure of the code. For the given code snippet, the AST might look like this:

```
Assignment
  ├─ Identifier: x
  └─ BinaryExpression: Addition
      ├─ IntegerLiteral: 5
      └─ IntegerLiteral: 3
```

This AST represents an assignment operation where the identifier `x` is assigned the sum of `5` and `3`. The AST captures the relationships between the code elements and their precedence.

3. **Generated Code**:
The generated code for this Rust snippet might look like:

```rust
let x = 5 + 3;
```

The generated code is similar to the original code as it's not performing any optimizations or transformations in this simplified example.

Keep in mind that this example is minimal, and in real-world scenarios, the AST and generated code can be more complex, especially for larger programs. However, this gives you a basic understanding of how tokens, AST, and generated code relate to each other in Rust.

## Keywords

Rust has several special keywords that serve specific purposes within the language. Here are some of the important keywords in Rust:

1. **let**: Used to introduce a new variable binding or pattern match.

2. **mut**: Specifies that a variable is mutable, allowing its value to be changed.

3. **const**: Declares a constant value that is immutable throughout the program.

4. **fn**: Begins the declaration of a function.

5. **struct**: Defines a new structure (a custom data type).

6. **enum**: Declares an enumeration, a type that can have multiple possible values.

7. **impl**: Implements methods or traits for a particular type.

8. **self**: Refers to the current instance of a struct or enum within its own methods.

9. **super**: Refers to the parent module or the parent implementation of a trait.

10. **mod**: Declares a new module, allowing you to organize code into logical units.

11. **use**: Imports symbols (functions, types, etc.) into the current scope for easier access.

12. **pub**: Specifies that an item (variable, function, etc.) can be accessed from outside its module.

13. **as**: Performs type casting or provides an alternative name for a symbol during import.

14. **if**, **else**: Conditionally executes code based on a boolean expression.

15. **match**: Performs pattern matching against a value, allowing for branching logic.

16. **loop**: Creates an infinite loop or an iterative loop with the `break` keyword.

17. **while**: Executes a loop as long as a condition is true.

18. **for**: Iterates over a range, an iterator, or a collection of items.

19. **break**: Exits a loop or a match expression prematurely.

20. **continue**: Skips the rest of the current iteration and proceeds to the next iteration in a loop.

21. **return**: Exits a function and optionally returns a value.

22. **unsafe**: Specifies that code is operating in an unsafe context, allowing for low-level operations.

23. **trait**: Defines a trait, which represents a set of behaviors that types can implement.

24. **dyn**: Used in trait objects to indicate dynamic dispatch.

These keywords play crucial roles in defining Rust's syntax and semantics, and understanding their usage is essential for writing idiomatic Rust code.


## Symbols 

In Rust, there are several special symbols, also known as operators or punctuation, used for various purposes, such as arithmetic and logical operations, assignment, comparison, and more. Here are some of the commonly used special symbols in Rust:

1. **Arithmetic Operators**: 
   - `+` Addition
   - `-` Subtraction
   - `*` Multiplication
   - `/` Division
   - `%` Remainder

2. **Comparison Operators**:
   - `==` Equality
   - `!=` Inequality
   - `<` Less than
   - `>` Greater than
   - `<=` Less than or equal to
   - `>=` Greater than or equal to

3. **Assignment Operator**:
   - `=` Assignment

4. **Logical Operators**:
   - `&&` Logical AND
   - `||` Logical OR
   - `!` Logical NOT (Negation)

5. **Bitwise Operators**:
   - `&` Bitwise AND
   - `|` Bitwise OR
   - `^` Bitwise XOR
   - `<<` Bitwise left shift
   - `>>` Bitwise right shift
   - `!` Bitwise NOT (Negation)

6. **Increment/Decrement Operators**:
   - `++` Increment (Not available in Rust)
   - `--` Decrement (Not available in Rust)

7. **Deref Operator**:
   - `*` Derefence operator

8. **Reference Operator**:
   - `&` Reference operator

9. **Range Operator**:
   - `..` Half-open range (inclusive lower bound, exclusive upper bound)
   - `..=` Closed range (inclusive lower and upper bounds)

10. **Indexing Operator**:
    - `[]` Indexing operator for accessing elements of arrays, slices, and other indexable types.

11. **Tuple/Struct Field Access Operator**:
    - `.` Dot operator for accessing fields of tuples and structs.

12. **Method Call Operator**:
    - `.` Dot operator for invoking methods on objects.

13. **Function Call Operator**:
    - `()` Parentheses used for function calls and to group expressions.

14. **Tuple Creation Operator**:
    - `,` Comma used to separate elements in a tuple.

15. **Block Delimiters**:
    - `{ }` Curly braces used to define blocks of code, such as in functions, loops, and conditional statements.

16. **Statement Terminator**:
    - `;` Semicolon used to terminate statements in Rust.

## Types

In Rust, there are several variable types available for representing different kinds of data. Here are some of the commonly used variable types in Rust:

1. **bool**: Represents a boolean value (`true` or `false`).

2. **char**: Represents a single Unicode character.

3. **i8, i16, i32, i64, i128**: Signed integers of various sizes.

4. **u8, u16, u32, u64, u128**: Unsigned integers of various sizes.

5. **isize**: Signed integer whose size is determined by the underlying system architecture.

6. **usize**: Unsigned integer whose size is determined by the underlying system architecture.

7. **f32, f64**: Floating-point numbers of single and double precision, respectively.

8. **array**: A fixed-size collection of elements of the same type.

9. **slice**: A reference to a portion of an array or a sequence of elements.

10. **str**: Represents a string slice, which is an immutable sequence of Unicode characters.

11. **String**: Represents an owned, mutable string.

12. **tuple**: A fixed-size ordered collection of elements of different types.

13. **struct**: A custom-defined data structure with named fields.

14. **enum**: A type that can have multiple possible values.

15. **option**: Represents an optional value that can be either `Some(value)` or `None`.

16. **result**: Represents the result of an operation that can be either `Ok(value)` or `Err(error)`.

17. **function pointers**: Variables that can hold references to functions.

18. **references**: Immutable references to other variables or data structures.

19. **mutable references**: Mutable references to other variables or data structures.

These are some of the primary variable types available in Rust. Each type has its own characteristics, behavior, and use cases. Rust's strong static typing system ensures that variables are used correctly and consistently throughout the program.
