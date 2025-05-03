# rust-ownership-explained

🚀 *Explaining Rust's Ownership model with clear, runnable code examples for beginners.*

[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20or%20Apache--2.0-blue.svg)](LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

## Table of Contents

- [Introduction](#introduction)
- [Core Concepts](#core-concepts)
  - [1. Ownership](#1-ownership)
  - [2. Borrowing](#2-borrowing)
  - [3. Lifetimes](#3-lifetimes)
- [Examples Directory](#examples-directory)
- [Detailed Examples](#detailed-examples)
  - [Example 1: Ownership Transfer](#example-1-ownership-transfer)
  - [Example 2: Borrowing and Mutability](#example-2-borrowing-and-mutability)
  - [Example 3: Ownership in Collections](#example-3-ownership-in-collections)
- [How to Run the Code](#how-to-run-the-code)
- [Common Errors & Fixes](#common-errors--fixes)
- [Further Reading](#further-reading)
- [License](#license)
- [Contributing](#contributing)

## Introduction

Welcome to **rust-ownership-explained**! This repository is designed to help newcomers to Rust understand its ownership model—a core feature that ensures memory safety without a garbage collector. If you're familiar with languages like Java, C++, or Python, you'll see how Rust’s approach is unique and powerful.

**Why Ownership Matters**: In Rust, ownership prevents memory bugs like dangling pointers or double frees at compile time. Unlike garbage-collected languages (e.g., Java), Rust gives you control over memory with zero runtime overhead, making it ideal for systems programming.

## Core Concepts

### 1. Ownership

Every value in Rust has a single owner—a variable that manages its memory. When the owner goes out of scope, Rust automatically frees the memory. Ownership can be transferred (moved) to another variable or function.

### 2. Borrowing

Borrowing lets you access data without taking ownership. There are two types:
- **Immutable borrows** (`&T`): Multiple read-only references are allowed.
- **Mutable borrows** (`&mut T`): Only one exclusive reference is allowed at a time.

The borrow-checker ensures you follow these rules to prevent data races.

### 3. Lifetimes

Lifetimes ensure references don’t outlive the data they point to. They’re a way to tell the compiler how long a reference is valid. We’ll touch on this lightly here—check the Rust Book for deeper dives.

## Examples Directory

The `examples/` folder contains standalone Rust files, each focusing on a specific ownership concept. Run them to see ownership in action!

- [`move_semantics.rs`](./examples/move_semantics.rs): Shows how ownership transfers with moves.
- [`borrowing.rs`](./examples/borrowing.rs): Demonstrates immutable and mutable borrowing.
- [`collections.rs`](./examples/collections.rs): Explores ownership with `Vec` and `HashMap`.

## Detailed Examples

Below are key examples with explanations, code, and outputs. Each is a simplified version of the files in `examples/`.

### Example 1: Ownership Transfer

This shows what happens when ownership moves between variables.

```rust
fn main() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1; // Ownership moves to s2
    // println!("{}", s1); // Error: s1 is no longer valid
    println!("{}", s2); // Works fine
}
```

**Output**:
```
hello
```

**Compiler Error (if uncommented)**:
```
error[E0382]: borrow of moved value: `s1`
```

**Why?** The `String` moved from `s1` to `s2`, making `s1` invalid. Rust prevents use-after-move bugs.

### Example 2: Borrowing and Mutability

This demonstrates immutable and mutable borrowing.

```rust
fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s; // Immutable borrow
    let r2 = &s; // Another immutable borrow
    println!("{} {}", r1, r2); // Works fine
    
    let r3 = &mut s; // Mutable borrow
    r3.push_str(", world");
    println!("{}", r3);
}
```

**Output**:
```
hello hello
hello, world
```

**Compiler Error (if you mix borrows)**:
```rust
let r1 = &s;
let r3 = &mut s; // Error!
println!("{}", r1);
```

<details>
<summary>Error Details</summary>

```
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
```

**Why?** Rust forbids mutable borrows when immutable ones are active to prevent data races.

</details>

✅ **Tip**: Finish using immutable borrows before creating a mutable one.

### Example 3: Ownership in Collections

This shows how collections like `Vec` take ownership.

```rust
use std::collections::HashMap;

fn main() {
    let mut v = Vec::new();
    let s = String::from("hello");
    v.push(s); // Ownership moves to v
    // println!("{}", s); // Error: s was moved
    
    let mut map = HashMap::new();
    let key = String::from("numbers");
    let value = vec![1, 2, 3];
    map.insert(key, value); // map owns key and value
    println!("{:?}", map);
}
```

**Output**:
```
{"numbers": [1, 2, 3]}
```

**Fix for Reuse**: Use `.clone()` to keep the original:
```rust
v.push(s.clone());
println!("{}", s); // Now works
```

⚠️ **Note**: Cloning copies data, so use it sparingly for performance.

## How to Run the Code

### Prerequisites
1. Install Rust (version 1.70+): [rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Verify installation:
   ```bash
   rustc --version
   cargo --version
   ```

### Running Examples
Clone the repo and navigate to the root:
```bash
git clone https://github.com/kaly7dev/rust-ownership-explained.git
cd rust-ownership-explained
```

Each example is a separate binary. Run them with:
```bash
cargo run --bin move_semantics
cargo run --bin borrowing
cargo run --bin collections
```

**Note**: Ensure your `Cargo.toml` lists binaries under `[bin]` for each example.

## Common Errors & Fixes

1. **Error: “borrow of moved value”**
   - **Why?** You used a variable after moving its ownership.
   - **Fix**: Clone the value or restructure to avoid the move.

2. **Error: “cannot borrow as mutable”**
   - **Why?** An immutable borrow is active, blocking the mutable borrow.
   - **Fix**: Ensure all immutable borrows are out of scope.

3. **Error: “cannot borrow as immutable”**
   - **Why?** A mutable borrow is active.
   - **Fix**: Complete the mutable operation first.

4. **Error: “value does not live long enough”**
   - **Why?** A reference outlives its data.
   - **Fix**: Adjust lifetimes or ensure the data lives longer.

## Further Reading

- [The Rust Programming Language - Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust By Example - Ownership](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Rust Playground](https://play.rust-lang.org/) - Test code online

## License

This project is licensed under the [MIT License](LICENSE) or [Apache-2.0 License](LICENSE-APACHE), at your choice.

## Contributing

Want to add a new example or improve one? Awesome! Check out [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines. Submit a pull request with your changes, and let’s make this repo even better.

---

### Bonus: Ownership Diagram

Here’s a simple ASCII diagram of ownership and borrowing:

```
Ownership:
s:   |-----------------| (owns String "hello")
     | s1 = String::from("hello")
     | s2 = s1 (move)
s2:       |-----------| (owns String after move)

Borrowing:
s:   |-----------------| (owns String)
r1:  |-----|            (&s: immutable borrow)
r2:    |---|            (&s: another immutable)
r3:          |-------|  (&mut s: mutable borrow)
```

This shows how `s1` loses ownership after a move to `s2`, and how borrows (`r1`, `r2`, `r3`) have limited lifetimes.