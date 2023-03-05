use libmake::run;

/// This is the main entry point for the libmake application.
fn main() {
    // Call the `run()` function from the `libmake` module.
    if let Err(ref e) = run() {
        eprintln!("Error running libmake: {}", e);
        std::process::exit(1);
    }
}
