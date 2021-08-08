mod lib;
type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
use lambda_http::{handler, lambda_runtime};
use user::func;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}
