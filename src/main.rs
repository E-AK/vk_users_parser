use std::env;
use vk_users_parser::{parse_config, read_config, search, mongodb_save};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);
    let cfg = read_config(String::from(filename));
    let result = search(&cfg).await;
    mongodb_save(result, &cfg).await;
}
