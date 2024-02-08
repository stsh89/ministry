mod clap_integration;
mod error;
mod playground;

use error::CliError;

pub struct Cli;

pub trait Run {
    fn run() -> Result<(), CliError>;
}

impl Run for Cli {
    fn run() -> Result<(), CliError> {
        clap_integration::run()
    }
}
