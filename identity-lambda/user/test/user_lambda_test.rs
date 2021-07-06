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

#[cfg(test)]
mod tests {
    use hvcg_iam_openapi_identity::models::User;
    use lambda_http::{RequestExt, Response};
    use lambda_http::http::Method;
    use std::collections::HashMap;
    use std::path::PathBuf;

    use super::*;

    #[tokio::test]
    async fn create_user_handles() {
        let user_request = User {
            id: Option::from(Uuid::new_v4()),
            username: "nhuthuynh".to_string(),
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
        // assert_eq!(response.body(), expected.body())
    }

}