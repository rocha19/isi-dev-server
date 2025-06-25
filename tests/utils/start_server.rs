use std::net::TcpListener;
use std::sync::OnceLock;
use tokio::time::{Duration, sleep};
use tracing_subscriber::FmtSubscriber;

static INIT: OnceLock<()> = OnceLock::new();

pub fn init_tracing() {
    INIT.get_or_init(|| {
        let subscriber = FmtSubscriber::new();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global default subscriber");
    });
}

pub async fn start_test_server() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);

    tokio::spawn(async move {
        if let Err(e) = isi_dev::frameworks::axum::server::run(port).await {
            eprintln!("Erro ao iniciar o servidor: {}", e);
        }
    });

    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(5) {
        if let Ok(_) = tokio::net::TcpStream::connect(("127.0.0.1", port)).await {
            return port;
        }
        sleep(Duration::from_millis(200)).await;
    }

    panic!("Server did not start in time");
}

pub fn get_port() -> u16 {
    dotenv::dotenv().ok();
    let port: u16 = dotenv::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a number");

    port
}
