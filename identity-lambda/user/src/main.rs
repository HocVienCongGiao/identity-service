use hvcg_iam_openapi_identity::models::User;
use lambda_http::{Body, Context, handler, IntoResponse, lambda, Request, RequestExt};
use lambda_http::http::method;
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
    let lambda_user_request: User = request.payload().unwrap_or_else(|_parse_err| None).unwrap();
        // .unwrap_or_else(|_parse_err| None)
        // .unwrap_or_default();

    let serialized_user = serde_json::to_string(&lambda_user_request).unwrap();
    println!("serialized_user: {}", serialized_user);

    if request.method() != method::Method::POST {
        println!("Request method is not in post method");
        return Ok(json!("Request method is in not post method"))

    }

    let result = controller::create_user(&lambda_user_request).await;

    println!("result id {}", result.id.unwrap_or_default());

    if result.id.is_none() {
        return Ok(json!("Failed to insert user"))
    }


    return Ok(json!(result))
    // Example code
    // async fn func(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    //     Ok(match event.query_string_parameters().get("first_name") {
    //         Some(first_name) => format!("Hello, {}!", first_name).into_response(),
    //         _ => Response::builder()
    //             .status(400)
    //             .body("Empty first name".into())
    //             .expect("failed to render response"),
    //     })
    // }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use hvcg_iam_openapi_identity::models::User;
    use lambda_http::{RequestExt, Response};
    use lambda_http::http::Method;

    use super::*;

    #[tokio::test]
    async fn create_user_handles() {
        let user_request = User {
            id: None,
            username: "test_user".to_string() + &*Uuid::new_v4().to_string(),
            email: Option::from("nhut_cargo@gmail.com".to_string()),
            phone: Option::from("0909686868".to_string()),
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        let request_default = Request::default();
        let (mut parts, _)
            = request_default.into_parts();
        parts.method = Method::POST;

        parts.headers.append("X-TFE-Notification-Signature",
                             "c7cf4bbba3ff2117c2b235e8c3d77d5023311736072c7af4b72b418361bc05249bc86addc4633382ac8191cfa995a272e578a08c49b508bf2c7bccbf5670ba04".parse().unwrap());
        parts.headers.append("Content-Type","application/json".parse().unwrap());

        let request = Request::from_parts(parts, Body::from(serialized_user));
        let expected = json!({"message": "Validation successful"}).into_response();
        let response = create_user(request, Context::default()).await.expect("expected Ok(_) value").into_response();

        assert_eq!(response.body(), expected.body())
    }

}