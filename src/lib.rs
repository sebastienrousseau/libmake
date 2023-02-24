use log::LevelFilter;
use simplelog::{CombinedLogger, Config, TermLogger, TerminalMode};
use std::error::Error;

pub mod args;
pub mod ascii;
pub mod cli;
pub mod generator;
pub mod interface;
pub mod utils;

/// Initializes the logger with a file logger and a terminal logger.
///
/// # Examples
///
/// ```
/// use libmake::run;
/// run();
/// ```
pub fn run() -> Result<(), Box<dyn Error>> {
    // Initialize logging
    let log_config = Config::default();
    let file_logger = simplelog::WriteLogger::new(
        LevelFilter::Debug,
        log_config.clone(),
        std::fs::File::create("libmake.log")?,
    );
    let term_logger = TermLogger::new(
        LevelFilter::Info,
        log_config,
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );
    CombinedLogger::init(vec![term_logger, file_logger])?;

    // Process the ascii art
    ascii::generate_ascii_art("LibMake", "./resources/standard.flf");

    // Process the command-line arguments
    let matches = cli::build_cli()?;
    args::process_arguments(matches);

    Ok(())
}
