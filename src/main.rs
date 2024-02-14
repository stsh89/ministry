//! A unified Rust platform for a wide variety of functionalities

mod cli;
mod datastax;
mod github;
mod palace;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    if let Err(error) = cli::run().await {
        println!("ERROR. {}", error.description);
    };
}
