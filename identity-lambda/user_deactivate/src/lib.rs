use domain::boundaries::UserMutationError;
use hvcg_iam_openapi_identity::models::User;
use jsonwebtoken::TokenData;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::{method, HeaderValue, StatusCode};
use lambda_http::{Body, Context, IntoResponse, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Deserialize, Serialize)]
struct TokenPayload {
    // Despite the struct field being named `username`, it is going to come
    // from a JSON field called `cognito:username`.
    #[serde(rename(deserialize = "cognito:username"))]
    username: String,
    #[serde(rename(deserialize = "cognito:groups"))]
    groups: Vec<String>,
}

pub async fn deactivate_user(
    request: Request,
    context: Context,
) -> Result<impl IntoResponse, Error> {
    println!("Request {:?}", request);
    println!("Request Method {:?}", request.method());

    if request.method() != method::Method::POST {
        println!("Request method is not in post method");
        return Ok(empty_response(request));
    }

    let lambda_user_request: Option<User> = request.payload().unwrap_or(None);
    if lambda_user_request.is_none() {
        return Ok(empty_response(request));
    }

    let serialized_user = serde_json::to_string(&lambda_user_request).unwrap();
    println!("serialized_user: {}", serialized_user);

    let user_response: Option<controller::openapi::identity_user::User>;
    let result = controller::deactivate_user(lambda_user_request.unwrap().id.unwrap()).await;

    let status_code: u16 = if result.is_ok() { 200 } else { 500 };

    user_response = result.map(Some).unwrap_or_else(|e| {
        println!("{:?}", e);
        None
    });

    let invoked_function_arn = context.invoked_function_arn;
    println!("invoked_function_arn: {}", invoked_function_arn);
    let user_table_name = if invoked_function_arn.contains("prod") {
        "prod-sg_UserTable"
    } else {
        "dev-sg_UserTable"
    }
    .to_string();

    let deactivate_user_dynamodb_result = db_cognito::deactivate_user_to_dynamodb(
        Option::from(&user_response),
        user_table_name.parse().unwrap(),
    )
    .await;
    println!(
        "Deactivate user dynamodb result: {}",
        deactivate_user_dynamodb_result
    );

    if !deactivate_user_dynamodb_result {
        println!("Error while updating to dynamodb")
    }

    let response: Response<Body> = Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .status(status_code)
        .body(
            serde_json::to_string(&user_response)
                .expect("unable to serialize serde_json::Value")
                .into(),
        )
        .expect("unable to build http::Response");
    println!("user response {:?}", serde_json::to_string(&user_response));

    Ok(response)
}

fn empty_response(_req: Request) -> Response<Body> {
    Response::builder()
        .header(CONTENT_TYPE, "application/json")
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
        .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
        .status(StatusCode::BAD_REQUEST)
        .body(
            serde_json::to_string(&User {
                id: None,
                username: "".to_string(),
                email: None,
                phone: None,
            })
            .expect("unable to serialize user_json::Value")
            .into(),
        )
        .expect("unable to build http::Response")
}
