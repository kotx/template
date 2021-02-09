mod config;

use tokio;

#[tokio::main]
async fn main() {
    let config = config::load("config.toml").await.unwrap();

    println!("{:?}", config);
}
