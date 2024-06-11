# Heapless Matrix Library Crate

This crate provides a simple and efficient implementation of a matrix data structure in Rust. It offers functionality for creating, manipulating, and performing operations on matrices, such as transposition, summation, and multiplication.

## Features

- **Generic Matrix Type**: Define matrices of any size with fixed dimensions using const generics.
- **Element-wise Operations**: Perform operations such as transposition, summation, and multiplication on matrices.
- **Compile-time Safety**: Ensure that matrix dimensions are enforced at compile-time, preventing runtime errors.
- **Efficient Memory Usage**: Utilizes the `heapless` crate for fixed-capacity vectors, ensuring efficient memory usage and avoiding heap allocations.
- **Suitable for Bare Metal Coding**: This implementation does not require heap allocation, making it suitable for bare metal programming and environments with limited resources.
- **Clear API**: Provides a clear and concise API for working with matrices, making it easy to integrate into your Rust projects.

## Usage

To use this crate in your Rust project, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
matrix-lib = {git = "https://github.com/Spago123/heapless-matrix"}
```
