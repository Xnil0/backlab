# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Code Architecture

This project is a Rust workspace containing two primary crates:
- `bakbon`: The core application logic.
- `kortex`: A supporting library or component.

## Common Commands

- **Build the entire workspace:**
  ```sh
  cargo build
  ```

- **Run all tests:**
  ```sh
  cargo test
  ```

- **Run a single test:**
  ```sh
  cargo test --test <TEST_NAME>
  ```

- **Lint the codebase:**
  ```sh
  cargo clippy --all-targets --all-features
  ```
