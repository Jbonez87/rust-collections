# Rust Collections

## Prerequisites

Install `rust` on your local machine if it isn't installed already. You can follow the [Rust Installation Guide](https://doc.rust-lang.org/book/ch01-01-installation.html)

## Running this project

Once `rust` is installed, run the following command in your terminal: `cargo run` and then follow the prompts. This will enable you to test the outputs of the project.

## Editing the Collections

Feel free to fork and edit this project however you see fit. For example, try adding more properties to the `SpreadsheetCell enum` and editing the `match` statement arms that use it. Here's what that could look like:

```rust
pub enum SpreadsheetCell {
    Int(i64),
    Float(f64),
    Text(String),
    // add new Title property here
    Title(String)
}
```
