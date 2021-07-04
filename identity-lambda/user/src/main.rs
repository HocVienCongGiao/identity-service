use lambda_http::{handler, lambda, Context, IntoResponse, Request, Body, RequestExt};
use serde_json::json;
use lambda_http::http::method;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use log::{self, info};
use hvcg_iam_openapi_identity::models::User;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Debug,Deserialize,Default)]
struct UserRequest {
    #[serde(default)]
    id: Uuid,
    #[serde(default)]
    username: String,
    #[serde(default)]
    email: String,
    #[serde(default)]
    phone: String
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(create_user)).await?;
    Ok(())
}

async fn create_user(request: Request, context: Context) -> Result<impl IntoResponse, Error> {
    let lambda_user_request: UserRequest = request_test.payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();

    println!("{}", format!(
        lambda_user_request.id,
        lambda_user_request.username,
        lambda_user_request.email,
        lambda_user_request.phone
    ));

    if request.method() != method::Method::POST {
        return Ok(Response::builder()
            .body(Body::Empty)
            .expect("unable to build http::Response"));
    }

    let result = controller::create_user(User {
        id: Option::from(lambda_user_request.id),
        username: lambda_user_request.username,
        email: Option::from(lambda_user_request.email),
        phone: Option::from(lambda_user_request.phone)
    });

    Ok(IntoResponse::into_response(result))
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