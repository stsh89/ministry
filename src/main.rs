//! A unified Rust platform for a wide variety of functionalities

mod cli;
mod github;
mod palace;

#[tokio::main]
async fn main() {
    if let Err(error) = cli::run().await {
        println!("{}", error.description);
    };
}
