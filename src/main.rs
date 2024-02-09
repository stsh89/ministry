//! # A unified Rust platform for a wide variety of functionalities
//!
//! A list of the current features:
//! - Custom playgrounds

mod cli;
mod github;
mod log_server;
mod playground;

#[tokio::main]
async fn main() {
    match cli::get_command() {
        Ok(command) => command.run(),
        Err(error) => println!("\n{}\n", error),
    }
}
