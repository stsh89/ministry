mod clap_integration;
mod error;
mod playground;

use error::CliError;

pub enum Command {
    PlaygroundListCommand(playground::ListCommand),
    PlaygroundRunCommand(playground::RunCommand),
}

impl Command {
    pub fn run(self) {
        use Command::*;

        match self {
            PlaygroundListCommand(cmd) => cmd.run(),
            PlaygroundRunCommand(cmd) => cmd.run(),
        }
    }
}

pub fn get_command() -> Result<Command, CliError> {
    clap_integration::run()
}
