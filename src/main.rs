mod cli;
mod playground;

fn main() {
    match cli::get_command() {
        Ok(command) => command.run(),
        Err(error) => println!("\n{}\n", error.description),
    }
}
