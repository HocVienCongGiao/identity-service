mod lib;

use lambda_runtime::{handler_fn, Error};
use user_table::func;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}
