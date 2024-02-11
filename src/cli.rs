use clap::{Arg, Command};

use crate::palace::establishment;

pub struct CliError {
    pub description: String,
}

pub async fn run() -> Result<(), CliError> {
    let command = Command::new("ministry")
        .about("")
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
        .subcommand_required(true);

    let matches = command.get_matches();

    match matches.subcommand() {
        Some(("playground", arg_matches)) => match arg_matches.subcommand() {
            Some(("run", arg_matches)) => {
                let name = arg_matches
                    .get_one::<String>("name")
                    .cloned()
                    .unwrap_or_default();

                let experiment_name = match name.as_str() {
                    "book" => establishment::developer::ExperimentName::Book,
                    name => {
                        return Err(CliError {
                            description: format!("Playground `{name}` not found."),
                        })
                    }
                };

                establishment::developer::run_code_experiment(
                    establishment::developer::CodeExperimentParameters { experiment_name },
                );
            }
            Some(("list", _arg_matches)) => {
                let experiment_names = establishment::developer::list_code_experiment_names();

                experiment_names
                    .iter()
                    .for_each(|name| println!("{}", name));
            }
            _ => unimplemented!(),
        },
        Some(("siesta", arg_matches)) => match arg_matches.subcommand() {
            Some(("read_zen", _arg_matches)) => {
                let result = establishment::developer::read_zen_wisdom().await;

                let Ok(inscription) = result else {
                    return Err(CliError {
                        description: "Error while reading zen wisdom".to_string(),
                    });
                };

                println!("{}", inscription.text);
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };

    Ok(())
}
