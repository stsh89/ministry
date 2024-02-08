//! # Command line interface

mod clap_integration;
mod error;

use crate::{log_server, playground};
use error::CliError;

pub struct Cli {
    command: CliCommand,
}

pub enum CliCommand {
    PlaygroundListCommand,
    PlaygroundRunCommand { name: Playground },
    LogServer,
}

pub enum Playground {
    BooksPlayground,
}

impl From<Playground> for String {
    fn from(value: Playground) -> Self {
        use Playground::*;

        match value {
            BooksPlayground => "books",
        }
        .to_string()
    }
}

impl TryFrom<String> for Playground {
    type Error = CliError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use Playground::*;

        let playground = match value.as_str() {
            "books" => BooksPlayground,
            name => return Err(CliError::playground_not_found(name)),
        };

        Ok(playground)
    }
}

impl Cli {
    pub fn run(&self) {
        use CliCommand::*;

        match &self.command {
            PlaygroundListCommand => list_playgrounds(),
            PlaygroundRunCommand { name } => run_playground(name),
            LogServer => log_server(),
        }
    }

    fn log_server_command() -> Self {
        Self {
            command: CliCommand::LogServer,
        }
    }

    fn playground_list_command() -> Self {
        Self {
            command: CliCommand::PlaygroundListCommand,
        }
    }

    fn playground_run_command(name: &str) -> Result<Self, CliError> {
        let name = name.to_string().try_into()?;
        let command = CliCommand::PlaygroundRunCommand { name };

        Ok(Self { command })
    }
}

pub fn get_command() -> Result<Cli, CliError> {
    clap_integration::get_command()
}

fn run_playground(name: &Playground) {
    use Playground::*;

    match name {
        BooksPlayground => playground::books::example(),
    }
}

fn list_playgrounds() {
    use Playground::*;

    let playgrounds: Vec<String> = vec![BooksPlayground.into()];

    println!("{:?}", playgrounds);
}

fn log_server() {
    log_server::run()
}
