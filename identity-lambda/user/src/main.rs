mod lib;
type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
use lambda_http::{handler, lambda};
use user::create_user;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(create_user)).await?;
    Ok(())
}
