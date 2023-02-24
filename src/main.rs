/// This is the main entry point for the libmake application.
fn main() {
    // Call the `run()` function from the `libmake` module.
    if let Err(err) = libmake::run() {
        eprintln!("Error running libmake: {}", err);
        std::process::exit(1);
    }
}
