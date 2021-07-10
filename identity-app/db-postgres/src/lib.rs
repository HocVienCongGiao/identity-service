use std::path::PathBuf;
use std::sync::Once;

use tokio_postgres::tls::NoTlsStream;
use tokio_postgres::{Client, Connection, Error, NoTls, Socket};

use domain::test_func;

pub mod config;
mod migration;
pub mod test1_gateway;
pub mod user_gateway;

pub async fn connect() -> Client {
    let config = config::Config::new();
    println!("Connecting with config {:?}", config);
    let result = tokio_postgres::connect(
        format!(
            "user={} password={} host={} port={} dbname={}",
            config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
        )
        .as_str(),
        NoTls,
    )
    .await;

    let (client, connection) = result.unwrap();
    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("Initial DB connection successfully");
    client
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn initialise() {
        INIT.call_once(|| {
            let my_path = PathBuf::new().join(".env.test");
            dotenv::from_path(my_path.as_path()).ok();
            println!("testing env {}", std::env::var("HELLO").unwrap());
        });
    }

    #[tokio::test]
    async fn it_works() {
        initialise();
        let result = 4;
        assert_eq!(result, 4);
        println!("finished test1");
    }
}
