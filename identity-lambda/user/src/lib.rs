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
pub async fn create_user(request: Request, context: Context) -> Result<impl IntoResponse, Error> {
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

    let empty_header_value = HeaderValue::from_str("").unwrap();

    let auth_header_value = request
        .headers()
        .get("authorization")
        .unwrap_or(&empty_header_value);

    if !auth_header_value.is_empty() {
        let auth_header_str = auth_header_value.to_str().unwrap();
        let username: String;
        let groups: Vec<String>;
        let jwt_token = &auth_header_str.to_string()[7..];
        let token_data: TokenData<TokenPayload> =
            jsonwebtoken::dangerous_insecure_decode(jwt_token).unwrap();
        let token_payload = token_data.claims;
        username = token_payload.username;
        groups = token_payload.groups;
        println!("Actual username {:?}", username);
        println!("Actual groups include {:?}", groups);
    }

    let serialized_user = serde_json::to_string(&lambda_user_request).unwrap();
    println!("serialized_user: {}", serialized_user);

    let status_code: u16;

    let user_response: Option<controller::openapi::identity_user::User>;
    let result = controller::create_user(&lambda_user_request.unwrap()).await;

    match result {
        Ok(_) => status_code = 200,
        Err(UserMutationError::UniqueConstraintViolationError(..)) => status_code = 503,
        Err(UserMutationError::InvalidUser) => status_code = 405,
        Err(UserMutationError::InvalidEmail) => status_code = 405,
        Err(UserMutationError::InvalidPhone) => status_code = 405,
        Err(UserMutationError::ExistedUser) => status_code = 400,
        Err(UserMutationError::UnknownError) | Err(UserMutationError::IdCollisionError) => {
            status_code = 500
        }
    }

    user_response = result.map(Some).unwrap_or_else(|e| {
        println!("{:?}", e);
        None
    });

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

    let insert_dynamodb_result = db_cognito::insert_user_to_dynamodb(Option::from(&user_response)).await;
    println!("Insert dynamodb result: {}", insert_dynamodb_result);
    return if insert_dynamodb_result {
        Ok(response)
    } else {
        Err(Box::new(("failed to insert to dynamodb")))
    }

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
