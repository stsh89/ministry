use crate::playground;

use super::error::CliError;

pub struct ListCli;

pub struct RunCli {
    name: Playground,
}

enum Playground {
    BooksPlayground,
}

impl ListCli {
    pub fn new() -> Self {
        ListCli
    }

    pub fn run(&self) {
        use Playground::*;

        let playgrounds: Vec<String> = vec![BooksPlayground.into()];

        println!("{:?}", playgrounds);
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

impl RunCli {
    pub fn new(name: &str) -> Result<Self, CliError> {
        let name = TryInto::<Playground>::try_into(name.to_string())?;

        Ok(RunCli { name })
    }

    pub fn run(&self) {
        use Playground::*;

        match self.name {
            BooksPlayground => playground::books::example(),
        }
    }
}
