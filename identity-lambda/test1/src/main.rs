use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(test1)).await?;
    Ok(())
}

async fn test1(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    controller::openapi::test1::create_test1();
    let pet = hvcg_example_openapi_entity::models::Pet {
        id: None,
        category: None,
        name: "123".to_string(),
        photo_urls: vec![],
        tags: None,
        status: None,
    };
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(json!(pet))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test1_handles() {
        let my_path = PathBuf::new().join(".env.test");
        dotenv::from_path(my_path.as_path()).ok();
        let test = controller::get_test1().await;
        println!("hello {}", test.status);
        let request = Request::default();
        let expected = json!({
        "name":"123","photoUrls":[]
        })
        .into_response();
        let response = test1(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
