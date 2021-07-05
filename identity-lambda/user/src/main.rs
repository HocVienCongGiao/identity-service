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
        "payload: {}", lambda_user_request.clone().to_string()
    ));

    if request.method() != method::Method::POST {
        println!("Request method is not in post method");
        return Ok(json!("Request method is in not post method"))

    }

    let result = controller::create_user(&lambda_user_request);

    return Ok(json!("User is created"))
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
    use super::*;
    use std::path::PathBuf;
    use lambda_http::{RequestExt, Response};
    use hvcg_iam_openapi_identity::models::User;
    use std::collections::HashMap;

    #[tokio::test]
    async fn create_user_handles() {
        let user_request = User {
            id: Option::from(Uuid::new_v4()),
            username: "nhuthuynh".to_string(),
            email: Option::from("nhut_cargo@gmail.com".to_string()),
            phone: Option::from("0909686868".to_string()),
        };
        let serialized_user = serde_json::to_string(&user_request).unwrap();
        println!("{}", serialized_user);

        let request = Request::new(
            Body::from(Body::from(serialized_user)
        ));
        let test_body = request.body().get(0);

        let lambda_user_request: User = request.payload().unwrap_or_else(|_parse_err| None).unwrap();

        println!("{}", format!(
            "{}", lambda_user_request.clone().to_string()
        ));

        // let response = create_user(request, Context::default())
        //     .await
        //     .expect("expected Ok(_) value")
        //     .into_response();
        // assert_eq!(response.body(), expected.body());


        // from the docs
        // https://docs.aws.amazon.com/lambda/latest/dg/eventsources.html#eventsources-api-gateway-request
        // let input = include_str!("../tests/data/apigw_v2_proxy_request.json");
        // let result = from_str(input);
        // assert!(
        //     result.is_ok(),
        //     format!("event was not parsed as expected {:?} given {}", result, input)
        // );
        // let req = result.expect("failed to parse request");
        // assert_eq!(req.method(), "POST");
        // assert_eq!(req.uri(), "https://id.execute-api.us-east-1.amazonaws.com/my/path?parameter1=value1&parameter1=value2&parameter2=value");
    }

}