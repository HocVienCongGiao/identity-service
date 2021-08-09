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

pub async fn func(request: Request, context: Context) -> Result<impl IntoResponse, Error> {
    println!("Request {:?}", request);
    println!("Request Method {:?}", request.method());

    if request.method() != method::Method::POST {
        println!("Request method is not in post method");
        return Ok(empty_response(request));
    }

    let lambda_user_request: Option<User> = request.payload().unwrap_or(None);
    let status_code: u16;

    let mut user = &lambda_user_request.unwrap();
    let user_response: Option<controller::openapi::identity_user::User>;
    let result: Result<User, UserMutationError>;

    let invoked_function_arn = context.invoked_function_arn;
    println!("invoked_function_arn: {:?}", invoked_function_arn);
    let user_table_name = get_user_table_name(invoked_function_arn);
    let function_name = get_function_name(request.uri().to_string());

    match function_name {
        FunctionName::Activation => {
            println!("Start activate user");
            result = controller::activate_user(user.id.unwrap()).await;
        }
        FunctionName::Deactivation => {
            println!("Start deactivate user");
            result = controller::deactivate_user(user.id.unwrap()).await;
        }
        FunctionName::CreateUser => {
            if user.username.is_empty() {
                println!("lambda_user_request is None");
                return Ok(empty_response(request));
            }
            println!("Start create user");
            result = controller::create_user(user).await;
        }
        FunctionName::Unknown => result = Result::Err(UserMutationError::UnknownError),
    }

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

    match function_name {
        FunctionName::Activation => {
            let dynamodb_result = db_cognito::activate_user_to_dynamodb(
                Option::from(&user_response),
                user_table_name.parse().unwrap(),
            )
            .await;
            println!("Activate dynamodb result: {}", dynamodb_result);

            if !dynamodb_result {
                println!("Error while active to dynamodb")
            }
        }
        FunctionName::Deactivation => {
            let dynamodb_result = db_cognito::deactivate_user_to_dynamodb(
                Option::from(&user_response),
                user_table_name.parse().unwrap(),
            )
            .await;
            println!("Deactivate dynamodb result: {}", dynamodb_result);

            if !dynamodb_result {
                println!("Error while deactive to dynamodb")
            }
        }
        FunctionName::CreateUser => {
            let insert_dynamodb_result = db_cognito::insert_user_to_dynamodb(
                Option::from(&user_response),
                user_table_name.parse().unwrap(),
            )
            .await;
            println!("Insert dynamodb result: {}", insert_dynamodb_result);

            if !insert_dynamodb_result {
                println!("Error while insert to dynamodb")
            }
        }
        FunctionName::Unknown => {
            println!("Unknown function")
        }
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

fn get_user_table_name(function_name: String) -> String {
    if function_name.contains("prod") {
        "prod-sg_UserTable"
    } else {
        "dev-sg_UserTable"
    }
    .to_string()
}

fn get_function_name(uri: String) -> FunctionName {
    if uri.contains("deactivation") {
        FunctionName::Deactivation
    } else if uri.contains("activation") {
        FunctionName::Activation
    } else {
        FunctionName::CreateUser
    }
}

#[derive(Debug)]
pub enum FunctionName {
    Activation,
    Deactivation,
    CreateUser,
    Unknown,
}
