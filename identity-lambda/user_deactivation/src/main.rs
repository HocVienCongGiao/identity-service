mod lib;
type Error = Box<dyn std::error::Error + Sync + Send + 'static>;
use lambda_http::{handler, lambda_runtime};
use user_deactivation::func;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Start deactivate user");
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}
