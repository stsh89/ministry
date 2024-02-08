mod cli;
mod playground;

use cli::Run;

fn main() {
    let result = cli::Cli::run();

    if let Err(cli_error) = result {
        println!("\n{}\n", cli_error.description);
    }
}
