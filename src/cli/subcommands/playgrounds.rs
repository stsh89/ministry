use crate::{cli::CliError, playground};

pub enum Playgrounds {
    BooksPlayground,
    // EcommersePlayground,
}

impl TryFrom<&str> for Playgrounds {
    type Error = CliError;

    fn try_from(value: &str) -> Result<Playgrounds, Self::Error> {
        use Playgrounds::*;

        let playground_name = match value {
            "books" => BooksPlayground,
            name => {
                return Err(CliError {
                    description: format!("not registered playground name: {name}"),
                })
            }
        };

        Ok(playground_name)
    }
}

impl Playgrounds {
    pub fn execute(self) {
        use Playgrounds::*;

        match self {
            BooksPlayground => playground::books::example(),
        }
    }
}
