use lambda_http::{handler, lambda, Context, IntoResponse, Request, Body, RequestExt};
use serde_json::json;
use lambda_http::http::method;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use log::{self, info};
use hvcg_iam_openapi_identity::models::User;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(create_user)).await?;
    Ok(())
}

async fn create_user(request: Request, context: Context) -> Result<impl IntoResponse, Error> {
    let lambda_user_request: User = request.payload().unwrap_or_else(|_parse_err| None).unwrap();
        // .unwrap_or_else(|_parse_err| None)
        // .unwrap_or_default();

    println!("{}", format!(
        "{}", lambda_user_request.clone().to_string()
    ));

    if request.method() != method::Method::POST {
        println!("Request method is not in post method");
        return Ok(json!("Request method is in not post method"))

    }

    let result = controller::create_user(&lambda_user_request);

    return Ok(json!("User is created"))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use lambda_http::{RequestExt, Response};
    use hvcg_example_openapi_entity::models::User;

    #[tokio::test]
    async fn create_user_handles() {
        let request = Request::default();
        let expected = json!({
        "message": "Test 2 is me, how are you?"
        })
            .into_response();
        let response = create_user(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body());


        // let user: UserRequest = request.payload()
        //     .unwrap_or_else(|_parse_err| None)
        //     .unwrap_or_default();
    }

}