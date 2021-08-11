use domain::boundaries::UserMutationError;
use hvcg_iam_openapi_identity::models::User;
use jsonwebtoken::TokenData;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::{method, HeaderValue, Method, StatusCode};
use lambda_http::{Body, Context, IntoResponse, Request, RequestExt, Response};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    let status_code: u16;
    let user_response: Option<controller::openapi::identity_user::User>;
    let invoked_function_arn = context.invoked_function_arn;
    println!("invoked_function_arn: {:?}", invoked_function_arn);
    let user_table_name = get_user_table_name(invoked_function_arn);
    let function_name = get_post_function_name(request.uri().to_string());

    match *request.method() {
        method::Method::GET => {
            println!("Handle get method.");
            user_response = None;
            status_code = 404;
        }
        method::Method::POST => match function_name {
            PostFunctionName::Activation => {
                let lambda_user_request: Option<User> = request.payload().unwrap_or(None);
                let mut user = &lambda_user_request.unwrap();
                println!("Start activate user");
                let result = controller::activate_user(user.id.unwrap()).await;
                status_code = set_status_code(&result);
                user_response = result.map(Some).unwrap_or_else(|e| {
                    println!("{:?}", e);
                    None
                });
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
            PostFunctionName::Deactivation => {
                println!("Start deactivate user");
                let lambda_user_request: Option<User> = request.payload().unwrap_or(None);
                let mut user = &lambda_user_request.unwrap();
                let result = controller::deactivate_user(user.id.unwrap()).await;
                status_code = set_status_code(&result);
                user_response = result.map(Some).unwrap_or_else(|e| {
                    println!("{:?}", e);
                    None
                });
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
            PostFunctionName::CreateUser => {
                let lambda_user_request: Option<User> = request.payload().unwrap_or(None);
                let mut user = &lambda_user_request.unwrap();
                if user.username.is_empty() {
                    println!("lambda_user_request is None");
                    return Ok(empty_response(request));
                }
                println!("Start create user");
                let result = controller::create_user(user).await;
                status_code = set_status_code(&result);
                user_response = result.map(Some).unwrap_or_else(|e| {
                    println!("{:?}", e);
                    None
                });
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
            _ => {
                user_response = None;
                status_code = 404
            }
        },
        method::Method::PUT => {
            if request.uri().to_string().contains("update-password") {
                println!("Update user password");
                let user_update_request = request.payload().unwrap_or(None);
                if user_update_request.is_some() {
                    let lambda_user_request: UserUpdate = user_update_request.unwrap();
                    let user_result =
                        controller::get_user_by_id(lambda_user_request.id.unwrap()).await;
                    let user = user_result.unwrap();
                    let update_password_result =
                        db_cognito::update_user_password(&user, lambda_user_request.plain_password)
                            .await;
                    println!("update_password_result: {}", update_password_result);
                    let result = controller::get_user_by_id(lambda_user_request.id.unwrap()).await;
                    status_code = set_status_code(&result);
                    user_response = result.map(Some).unwrap_or_else(|e| {
                        println!("{:?}", e);
                        None
                    });
                } else {
                    user_response = None;
                    status_code = 404
                }
            } else {
                user_response = None;
                status_code = 404
            }
        }
        _ => {
            user_response = None;
            status_code = 404
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

fn set_status_code(result: &Result<User, UserMutationError>) -> u16 {
    match result {
        Ok(_) => 200,
        Err(UserMutationError::UniqueConstraintViolationError(..)) => 503,
        Err(UserMutationError::InvalidUser) => 405,
        Err(UserMutationError::InvalidEmail) => 405,
        Err(UserMutationError::InvalidPhone) => 405,
        Err(UserMutationError::ExistedUser) => 400,
        Err(UserMutationError::UnknownError) | Err(UserMutationError::IdCollisionError) => 500,
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

fn get_user_table_name(function_name: String) -> String {
    if function_name.contains("prod") {
        "prod-sg_UserTable"
    } else {
        "dev-sg_UserTable"
    }
    .to_string()
}

fn get_post_function_name(uri: String) -> PostFunctionName {
    if uri.contains("deactivation") {
        PostFunctionName::Deactivation
    } else if uri.contains("activation") {
        PostFunctionName::Activation
    } else {
        PostFunctionName::CreateUser
    }
}

pub fn get_id_from_request(req: &Request) -> Option<uuid::Uuid> {
    let path_parameters = req.path_parameters();
    let id_param = path_parameters.get("id");
    if let Some(id) = id_param {
        println!("id found");
        Some(uuid::Uuid::parse_str(id).unwrap())
    } else {
        println!("id not found");
        None
    }
}

#[derive(Debug)]
pub enum PostFunctionName {
    Activation,
    Deactivation,
    CreateUser,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct UserUpdate {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "plainPassword")]
    pub plain_password: String,
}
