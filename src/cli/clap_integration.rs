use super::{error::CliError, CliCommand};
use clap::{Arg, ArgMatches, Command};

const PLAYGROUND_COMMAND: &str = "playground";
const LOG_SERVER_COMMAND: &str = "log_server";
const GITHUB_COMMAND: &str = "github";

pub fn get_command() -> Result<CliCommand, CliError> {
    let command = command();
    let command_name = command.get_name().to_string();
    let arg_matches = command.get_matches();
    let subcommand = arg_matches.subcommand();

    let Some((subcommand, arg_matches)) = subcommand else {
        return Err(CliError::missing_subcommand(&command_name));
    };

    match subcommand {
        PLAYGROUND_COMMAND => get_playground_command(arg_matches),
        LOG_SERVER_COMMAND => Ok(get_log_server_command()),
        GITHUB_COMMAND => get_github_command(arg_matches),
        name => Err(CliError::command_not_found(name)),
    }
}

fn get_github_command(arg_matches: &ArgMatches) -> Result<CliCommand, CliError> {
    let subcommand = arg_matches.subcommand();

    let Some((subcommand, _arg_matches)) = subcommand else {
        return Err(CliError::missing_subcommand(PLAYGROUND_COMMAND));
    };

    let cli = match subcommand {
        "zen" => get_github_zen_command(),
        "octocat" => get_github_octocat_command(),
        name => return Err(CliError::subcommand_not_found(PLAYGROUND_COMMAND, name)),
    };

    Ok(cli)
}

fn get_github_octocat_command() -> CliCommand {
    todo!()
}

fn get_github_zen_command() -> CliCommand {
    todo!()
}

fn get_playground_command(arg_matches: &ArgMatches) -> Result<CliCommand, CliError> {
    let subcommand = arg_matches.subcommand();

    let Some((subcommand, arg_matches)) = subcommand else {
        return Err(CliError::missing_subcommand(PLAYGROUND_COMMAND));
    };

    let cli = match subcommand {
        "list" => get_playground_list_command(),
        "run" => get_playground_run_command(arg_matches)?,
        name => return Err(CliError::subcommand_not_found(PLAYGROUND_COMMAND, name)),
    };

    Ok(cli)
}

fn get_log_server_command() -> CliCommand {
    CliCommand::log_server_command()
}

fn get_playground_list_command() -> CliCommand {
    CliCommand::playground_list_command()
}

fn get_playground_run_command(arg_matches: &ArgMatches) -> Result<CliCommand, CliError> {
    let empty_playground_name = String::new();
    let name = arg_matches
        .get_one::<String>("name")
        .unwrap_or(&empty_playground_name);

    let cli = CliCommand::playground_run_command(name)?;

    Ok(cli)
}

fn command() -> clap::Command {
    clap::Command::new("ministry")
        .subcommand(github_command())
        .subcommand(log_server_command())
        .subcommand(playground_command())
        .subcommand_required(true)
}

fn github_command() -> Command {
    Command::new(GITHUB_COMMAND)
        .about("Interact with Github API")
        .subcommand_required(true)
        .subcommand(github_zen_command())
        .subcommand(github_octocat_command())
}

fn github_zen_command() -> Command {
    Command::new("zen").about("Receive an inspiring message")
}

fn github_octocat_command() -> Command {
    Command::new("octocat").about("Receive a text message from Octocat")
}

fn log_server_command() -> Command {
    Command::new(LOG_SERVER_COMMAND).about("Start the server to check the incoming request data")
}

fn playground_command() -> Command {
    Command::new(PLAYGROUND_COMMAND)
        .about("Run any experimental code")
        .subcommand_required(true)
        .subcommand(playground_list_command())
        .subcommand(playground_run_command())
}

fn playground_list_command() -> Command {
    Command::new("list").about("List names of all playgrounds")
}

fn playground_run_command() -> Command {
    let name_arg = Arg::new("name")
        .required(true)
        .help("The name of the playground to run");

    Command::new("run")
        .about("Run playground code")
        .arg(name_arg)
}
