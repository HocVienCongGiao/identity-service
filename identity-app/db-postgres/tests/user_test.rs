#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Once;

    use db_postgres::connect;
    use tokio_postgres::types::ToSql;
    use uuid::Uuid;

    static INIT: Once = Once::new();

    fn initialise() {
        INIT.call_once(|| {
            let my_path = PathBuf::new().join(".env.test");
            dotenv::from_path(my_path.as_path()).ok();
            println!("testing env {}", std::env::var("HELLO").unwrap());
        });
    }

    #[tokio::test]
    async fn connect_db_test() {
        let connect = connect().await;

        let stmt = (connect)
            .prepare("SELECT * FROM identity__user_view")
            .await
            .unwrap();

        let result = connect.execute(&stmt, &[]).await;

        assert_eq!(result.is_ok(), true)
    }
}
