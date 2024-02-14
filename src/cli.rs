use crate::{
    datastax::{Datastax, DatastaxConfig},
    palace::{
        assistant::Secretary,
        establishment::{self, student::NewIdeaParameters},
    },
};
use clap::{Arg, Command};
use serde::Deserialize;
use std::fs;

pub struct CliError {
    pub description: String,
}

#[derive(Deserialize)]
struct CliConfig {
    datastax: CliDatastaxConfig,
}

#[derive(Deserialize)]
struct CliDatastaxConfig {
    database_id: String,
    region: String,
    keyspace: String,
    application_token: String,
}

impl CliConfig {
    fn datastax(&self) -> DatastaxConfig {
        DatastaxConfig {
            database_id: self.datastax.database_id.to_string(),
            database_region: self.datastax.region.to_string(),
            database_keyspace: self.datastax.keyspace.to_string(),
            database_application_token: self.datastax.application_token.to_string(),
        }
    }
}

pub async fn run() -> Result<(), CliError> {
    let command = Command::new("ministry")
        .about("")
        .arg(
            Arg::new("config")
                .long("config")
                .default_value("config.json"),
        )
        .subcommand(
            Command::new("playground")
                .subcommand(Command::new("run").arg(Arg::new("name").required(true)))
                .subcommand(Command::new("list"))
                .subcommand_required(true),
        )
        .subcommand(
            Command::new("siesta")
                .subcommand(Command::new("read_zen"))
                .subcommand_required(true),
        )
        .subcommand(Command::new("notebook").subcommand(
            Command::new("write").arg(Arg::new("thought").long("thought").required(true)),
        ))
        .subcommand_required(true);

    let matches = command.get_matches();
    let config_path = matches
        .get_one::<String>("config")
        .cloned()
        .unwrap_or_default();

    match matches.subcommand() {
        Some(("playground", arg_matches)) => match arg_matches.subcommand() {
            Some(("run", arg_matches)) => {
                let name = arg_matches
                    .get_one::<String>("name")
                    .cloned()
                    .unwrap_or_default();

                establishment::developer::run_code_experiment(&name).map_err(|err| CliError {
                    description: err.description,
                })?;
            }
            Some(("list", _arg_matches)) => {
                let code_experiments = establishment::developer::list_code_experiments();

                code_experiments
                    .iter()
                    .for_each(|(name, _)| println!("{}", name));
            }
            _ => unimplemented!(),
        },
        Some(("siesta", arg_matches)) => match arg_matches.subcommand() {
            Some(("read_zen", _arg_matches)) => {
                let result = establishment::developer::show_zen_quote().await;

                let Ok(inscription) = result else {
                    return Err(CliError {
                        description: "Error while reading zen wisdom".to_string(),
                    });
                };

                println!("{}", inscription.text);
            }
            _ => unimplemented!(),
        },
        Some(("notebook", arg_matches)) => match arg_matches.subcommand() {
            Some(("write", arg_matches)) => {
                let thought = arg_matches
                    .get_one::<String>("thought")
                    .cloned()
                    .unwrap_or_default();

                let config_string = fs::read_to_string(config_path).map_err(|err| CliError {
                    description: err.to_string(),
                })?;
                let config: CliConfig =
                    serde_json::from_str(&config_string).map_err(|err| CliError {
                        description: err.to_string(),
                    })?;

                let provider = Datastax::new(config.datastax()).map_err(|err| CliError {
                    description: err.to_string(),
                })?;
                let secretary = Secretary::new(provider);

                let result = establishment::student::note_new_idea(NewIdeaParameters {
                    notebook: secretary,
                    thought: &thought,
                })
                .await;

                match result {
                    Ok(idea) => println!("{}", idea.thought()),
                    Err(error) => {
                        return Err(CliError {
                            description: error.to_string(),
                        })
                    }
                };
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    Ok(())
}
