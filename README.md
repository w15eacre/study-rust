# study-rust

Learning the Rust programming language

This repository is dedicated to exploring and practicing Rust through small projects.

## 📌 Project 1: Calculator

The first project is a simple calculator.  
It consists of three main stages:

1. **Tokenization** — the calculator parses a math expression into tokens (numbers, operators, brackets, etc.).
2. **Math expession parse** - Analyzes and transform a sequence of tokens.
3. **Conversion to Reverse Polish Notation (RPN)** — using the Shunting Yard algorithm.
4. **Evaluation** — the calculator evaluates the expression in RPN form.

---

✅ Features:
- Supports basic operators: `+`, `-`, `*`, `/`
- Handles parentheses
- Parses floating-point numbers

---

✍️ Example:

```rust
let expression = "(12.5 + 3) * 2";
let result = calculator::evaluate(expression).unwrap();
println!("Result: {}", result); // Output: 31.0
