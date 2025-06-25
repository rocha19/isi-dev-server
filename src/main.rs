mod application;
mod domain;
mod frameworks;
mod interfaces;

use crate::frameworks::axum::server::run;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    if let Err(e) = run(port).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
