mod user_lambda_test;

use hvcg_iam_openapi_identity::models::User;
use lambda_http::http::method;
use lambda_http::{handler, lambda, Body, Context, IntoResponse, Request, RequestExt};
use log::{self, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(create_user)).await?;
    Ok(())
}

async fn create_user(request: Request, context: Context) -> Result<impl IntoResponse, Error> {
    let lambda_user_request: Option<User> = request.payload().unwrap_or(None);
    if lambda_user_request.is_none() {
        return Ok(json!("Not valid user request"));
    }

    let serialized_user = serde_json::to_string(&lambda_user_request).unwrap();
    println!("serialized_user: {}", serialized_user);

    if request.method() != method::Method::POST {
        println!("Request method is not in post method");
        return Ok(json!("Request method is in not post method"));
    }

    let result = controller::create_user(&lambda_user_request.unwrap()).await;

    println!("result id {}", result.id.unwrap_or_default());

    if result.id.is_none() {
        return Ok(json!("Failed to insert user"));
    }

    return Ok(json!(result));
}
