mod cli;
mod playground;

fn main() {
    let result = cli::run();

    if let Err(cli_error) = result {
        println!("{}", cli_error.description);
    }
}
