use domain::test_func;
use tokio_postgres::{Client, Error, NoTls};

pub mod config;
mod migration;
pub mod test1_gateway;
pub async fn connect() -> Client {
    let config = config::Config::new();
    println!("Connecting with config {:?}", config);
    let result = tokio_postgres::connect(
        format!(
            "user={} password={} host={} port={} dbname={}",
            config.db_user, config.db_password, config.db_host, config.db_port, config.db_name
        )
        .as_str(),
        //         tokio_postgres::connect("postgresql://postgres:password@localhost/test", NoTls).await?;
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
    client
    // p%40ssword
}
pub async fn migrate(mut client: Client) -> Client {
    migration::migrations::runner()
        .run_async(&mut client)
        .await
        .expect("Hey why did I fail?");
    client
}
pub async fn main(mut client: Client) -> Result<Client, Error> {
    println!("Start of main............()");
    // connect to the DB

    // let mut obj = db_pool.get().await?;
    // let client_refinery = obj.deref_mut().deref_mut();
    println!("did we successfully get the client and conn?");
    test_func();
    println!("start migrations");
    migration::migrations::runner()
        .run_async(&mut client)
        .await
        .expect("Hey why did I fail?");
    // embedded::migrations::runner().run_async(&mut client);
    println!("finished migrations");

    let _ = client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ",
        )
        .await;

    let query = client.query_one(
        "
            SELECT * FROM author_initial      
            WHERE id = $1        
    ",
        &[&2],
    );

    let result = query.await;

    // use std::io::{stdin, stdout, Write};
    // let mut s = String::new();
    // print!("Please enter some text: ");
    // let _ = stdout().flush();
    // stdin()
    //     .read_line(&mut s)
    //     .expect("Did not enter a correct string");
    // if let Some('\n') = s.chars().next_back() {
    //     s.pop();
    // }
    // if let Some('\r') = s.chars().next_back() {
    //     s.pop();
    // }
    // println!("You typed: {}", s);

    let row = result.unwrap();
    let author_name = row.get::<&str, &str>("name");
    println!("got row {}", author_name);
    let _ = client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ",
    );
    Ok(client)
}

#[cfg(test)]
mod tests {
    use crate::main;
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
        assert_eq!(2 + 2, 4);
        println!("finished test1");
    }
}
