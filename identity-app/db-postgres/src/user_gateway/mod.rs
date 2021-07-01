use async_trait::async_trait;
use tokio_postgres::Client;

mod mutation;
mod query;

pub struct UserRepository {
    pub client: Client,
}

#[async_trait]
impl domain::boundaries::UserDbGateway for UserRepository {
    async fn exists_by_username(&self, username: String) -> bool {
        let result = query::find_one_by_username(&(*self).client, username.clone()).await;
        println!("second block_on for row");
        if result.is_err() {
            return false;
        }
        let row = result.unwrap();
        let name_found: String = row.get("username");
        println!("ROW IS {}", name);
        name_found == name
    }

    async fn insert(&self, user: User) -> bool {
        let result = mutation::save(
            &(*self).client, user).await;
        return result.is_ok();
    }
}
