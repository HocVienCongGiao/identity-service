use pg_embed::fetch::{Architecture, FetchSettings, OperationSystem, PG_V13};
use pg_embed::postgres::{PgEmbed, PgSettings};
use std::time::Duration;

pub async fn start_postgres() -> PgEmbed {
    let config = db_postgres::config::Config::new();

    // /// Postgresql settings
    let pg_settings = PgSettings {
        // /// Where to store the postgresql executables
        executables_dir: "data/postgres".to_string(),
        // /// Where to store the postgresql database
        database_dir: "data/db".to_string(),
        port: 6543,
        user: config.db_user,
        password: config.db_password,
        // /// If persistent is false clean up files and directories on drop, otherwise keep them
        persistent: false,
        start_timeout: Duration::from_secs(15),
        // /// If migration sql scripts need to be run, the directory containing those scripts can be
        // /// specified here with `Some(path_to_dir), otherwise `None` to run no migrations.
        // /// To enable migrations view the **Usage** section for details
        migration_dir: None,
    };

    let postgres_os: OperationSystem;
    match config.os_type.as_str() {
        "linux" => postgres_os = OperationSystem::Linux,
        "darwin" => postgres_os = OperationSystem::Darwin,
        _ => postgres_os = OperationSystem::Windows,
    }
    println!("os is {}", postgres_os.to_string());
    // /// Postgresql binaries download settings
    let fetch_settings = FetchSettings {
        host: "https://repo1.maven.org".to_string(),
        operating_system: postgres_os,
        architecture: Architecture::Amd64,
        version: PG_V13,
    };

    // /// Create a new instance
    let mut pg = PgEmbed::new(pg_settings, fetch_settings);

    // async block only to show that these methods need to be executed in an async context
    // async {
    // /// Download, unpack, create password file and database cluster
    let is_setup = pg.setup().await;
    println!("is_setup {:?}", is_setup);
    // /// start postgresql database
    let started = pg.start_db().await;
    println!("isStarted {:?}", started);

    // /// create a new database
    // /// to enable migrations view the **Usage** section for details
    let _ = pg.create_database(config.db_name.as_str()).await;

    // /// drop a new database
    // /// to enable migrations view [Usage] for details
    // /// to enable migrations view the **Usage** section for details
    // pg.drop_database("database_name").await;

    // /// check database existence
    // /// to enable migrations view [Usage] for details
    // /// to enable migrations view the **Usage** section for details
    let exists = pg.database_exists("database_name").await;
    println!("DB Exists? {:?}", exists);

    // /// run migration sql scripts
    // /// to enable migrations view [Usage] for details
    // /// to enable migrations view the **Usage** section for details
    // pg.migrate("database_name").await;
    // };
    // /// get the base postgresql uri
    // /// `postgres://{username}:{password}@localhost:{port}`
    let pg_uri: &str = &pg.db_uri;
    println!("pg_uri is {}", pg_uri);
    // /// get a postgresql database uri
    // /// `postgres://{username}:{password}@localhost:{port}/{specified_database_name}`
    let pg_db_uri: String = pg.full_db_uri("database_name");
    println!("full_db_uri is {}", pg_db_uri);

    // stop postgresql database
    // pg.stop_db();
    return pg;
}
