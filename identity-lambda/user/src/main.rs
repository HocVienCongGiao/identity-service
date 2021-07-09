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
    let lambda_user_request: User = request.payload().unwrap_or_else(|_parse_err| None).unwrap();

    let serialized_user = serde_json::to_string(&lambda_user_request).unwrap();
    println!("serialized_user: {}", serialized_user);

    if request.method() != method::Method::POST {
        println!("Request method is not in post method");
        return Ok(json!("Request method is in not post method"));
    }

    let result = controller::create_user(&lambda_user_request).await;

    println!("result id {}", result.id.unwrap_or_default());

    if result.id.is_none() {
        return Ok(json!("Failed to insert user"));
    }

    return Ok(json!(result));
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use hvcg_iam_openapi_identity::models::User;
    use lambda_http::http::Method;
    use lambda_http::{RequestExt, Response};
    use rand::Rng;

    use super::*;

    #[tokio::test]
    async fn create_user_success() {
        // Given
        let test_suffix = Uuid::new_v4().to_string();

        let user_request = User {
            id: None,
            username: "test_user".to_string() + &*test_suffix,
            email: Option::from("nhut_cargo@gmail.com".to_string() + &*test_suffix),
            phone: Option::from("+84 909686868".to_string() + &*test_suffix),
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        let request_default = Request::default();
        let (mut parts, _) = request_default.into_parts();
        parts.method = Method::POST;

        parts.headers.append("X-TFE-Notification-Signature",
                             "c7cf4bbba3ff2117c2b235e8c3d77d5023311736072c7af4b72b418361bc05249bc86addc4633382ac8191cfa995a272e578a08c49b508bf2c7bccbf5670ba04".parse().unwrap());
        parts
            .headers
            .append("Content-Type", "application/json".parse().unwrap());

        let request = Request::from_parts(parts, Body::from(serialized_user));

        // When
        let response = create_user(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        assert_eq!(response.status(), 200);

        let deserialized_user: User = serde_json::from_slice(response.body()).unwrap();

        assert_eq!(deserialized_user.username, "test_user".to_string() + &*test_suffix);
        assert_eq!(deserialized_user.email, Option::from("nhut_cargo@gmail.com".to_string() + &*test_suffix));
        assert_eq!(deserialized_user.phone, Option::from("+84 909686868".to_string() + &*test_suffix));

    }

    #[tokio::test]
    async fn create_user_failed() {
        // Given

        let user_request = User {
            id: None,
            username: "".to_string(),
            email: None,
            phone: None
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        let request_default = Request::default();
        let (mut parts, _) = request_default.into_parts();
        parts.method = Method::POST;

        parts.headers.append("X-TFE-Notification-Signature",
                             "c7cf4bbba3ff2117c2b235e8c3d77d5023311736072c7af4b72b418361bc05249bc86addc4633382ac8191cfa995a272e578a08c49b508bf2c7bccbf5670ba04".parse().unwrap());
        parts.headers.append("Content-Type", "application/json".parse().unwrap());

        let request = Request::from_parts(parts, Body::from(serialized_user));

        let expected_response = json!("Failed to insert user").into_response();

        // When
        let response = create_user(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        assert_eq!(response.status(), 200);

        assert_eq!(response.body(), expected_response.body())

    }
}
