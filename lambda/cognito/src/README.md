// https://docs.rs/rusoto_cognito_idp/0.46.0/rusoto_cognito_idp/
use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
lambda::run(handler(cognito)).await?;
Ok(())
}

async fn cognito(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
// `serde_json::Values` impl `IntoResponse` by default
// creating an application/json response
Ok(json!({
"message": "Hey cognito!"
}))
}

#[cfg(test)]
mod tests {
use super::*;

    #[tokio::test]
    async fn cognito_handles() {
        let request = Request::default();
        let expected = json!({
        "message": "Hey cognito!"
        })
        .into_response();
        let response = cognito(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
