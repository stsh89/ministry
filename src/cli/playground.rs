use super::error::CliError;
use crate::playground;

pub struct ListCommand;

pub struct RunCommand {
    name: Playground,
}

enum Playground {
    BooksPlayground,
}

impl ListCommand {
    pub fn new() -> Self {
        ListCommand
    }

    pub fn run(&self) {
        use Playground::*;

        let playgrounds: Vec<String> = vec![BooksPlayground.into()];

        println!("{:?}", playgrounds);
    }
}

impl RunCommand {
    pub fn new(name: &str) -> Result<Self, CliError> {
        let name = TryInto::<Playground>::try_into(name.to_string())?;

        Ok(RunCommand { name })
    }

    pub fn run(&self) {
        use Playground::*;

        match self.name {
            BooksPlayground => playground::books::example(),
        }
    }
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
            _ => {
                return Err(CliError {
                    description: "Playground not found".to_string(),
                })
            }
        };

        Ok(playground)
    }
}
