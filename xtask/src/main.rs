//! This is the main entry point for the xtask crate.
//!
//! The `main()` function serves as the starting point of the executable.
//! It returns a `Result<(), anyhow::Error>`, indicating success or failure.
//! The `xtaskops::tasks::main()` function is called to perform the main tasks.
//!
//! # Errors
//!
//! If an error occurs during the execution of the tasks, an `anyhow::Error` is returned.

fn main() -> Result<(), anyhow::Error> {
    xtaskops::tasks::main()
}
