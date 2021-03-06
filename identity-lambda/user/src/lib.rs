use domain::boundaries::UserMutationError;
use hvcg_iam_openapi_identity::models::{User, UserCollection};
use jsonwebtoken::TokenData;
use lambda_http::http::header::{
    ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS, ACCESS_CONTROL_ALLOW_ORIGIN,
    CONTENT_TYPE,
};
use lambda_http::http::{method, HeaderValue, Method, StatusCode};
use lambda_http::{Body, Context, IntoResponse, Request, RequestExt, Response};
use rusoto_cognito_idp::{
    AdminSetUserPasswordRequest, CognitoIdentityProvider, CognitoIdentityProviderClient,
};
use rusoto_core::{Client, Region};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
    println!("path_parameters {:?}", request.path_parameters());
    println!(
        "query_string_parameters {:?}",
        request.query_string_parameters()
    );
    println!("Request Method {:?}", request.method());
    if request.method() == method::Method::OPTIONS {
        return Ok(Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .header(ACCESS_CONTROL_ALLOW_HEADERS, "*")
            .header(ACCESS_CONTROL_ALLOW_METHODS, "*")
            .status(200)
            .body(Body::Empty)
            .expect("unable to build http::Response"));
    }
    let default_header_value = HeaderValue::from_str("Bearer eyJraWQiOiJaTGpneG41SStaZEpldnJRb0lpMTZEWEZoRHI4eG9UbVZ2b2ZuVm5vb3RFPSIsImFsZyI6IlJTMjU2In0.eyJzdWIiOiJmZDlhN2FmOC1mYTc2LTRiODYtYWYzZC1kOTYzNGVmNTIzNzQiLCJhdWQiOiIxcmF2NDExbmNjbnA3M2h0b3BiaG1sOHM2MSIsImNvZ25pdG86Z3JvdXBzIjpbIk9wZXJhdG9yR3JvdXAiXSwiZXZlbnRfaWQiOiI5NjQ1ZDYyMi0zZjRiLTQyYjctOWI0ZC03MWQzNWRhOTI1NmQiLCJ0b2tlbl91c2UiOiJpZCIsImF1dGhfdGltZSI6MTYyMzkzNDkyNiwiaXNzIjoiaHR0cHM6XC9cL2NvZ25pdG8taWRwLmFwLXNvdXRoZWFzdC0xLmFtYXpvbmF3cy5jb21cL2FwLXNvdXRoZWFzdC0xXzlRV1NZR3pYayIsInBob25lX251bWJlcl92ZXJpZmllZCI6dHJ1ZSwiY29nbml0bzp1c2VybmFtZSI6ImRldi1vcGVyYXRvciIsInBob25lX251bWJlciI6Iis4NDM2OTE0MDkxNiIsImV4cCI6MTYyMzk0ODAwOCwiaWF0IjoxNjIzOTQ0NDA4fQ.ml3N8J7uw4rbQOneEdnmQW6OwsAY6ycmp5PIrKGZKF3yWQn0oQECIhF2Q_jjWOjWPikpUQEy5IKgghiJLukgKo7q-T4tUauPG3GJxoSGQkfVcglkNu8nZTu7ioxXzlQAWsXLakgkH40mGzI6kl2hkEhRQh_lWGrT7TqDP2yVTsDMKEGJBdtcb-kFCnYHfn9FMoCyVGo4K3tSrkeGno7bzwO_XpFtZRhv9Qs4OtfESXARYCP3St69hyf4JuAop6-Zb38FPWcp6rnpRG3BF64YPGqo0J0MAyWVz_Du7Pk3-H5uZqqrr6iHKoPwoabPPlZxJ3JGdifVt_I54SwTbelbzw").unwrap();
    let auth_header_value = request
        .headers()
        .get("authorization")
        .unwrap_or(&default_header_value);
    let auth_header_str = auth_header_value.to_str().unwrap();
    let username: String;
    let groups: Vec<String>;
    if auth_header_str != "anonymous12" {
        let jwt_token = &auth_header_str.to_string()[7..];
        let token_data: TokenData<TokenPayload> =
            jsonwebtoken::dangerous_insecure_decode(jwt_token).unwrap();
        let token_payload = token_data.claims;
        username = token_payload.username;
        groups = token_payload.groups;
        println!("Groups include {:?}", groups);
    } else {
        username = String::from("anonymous");
    }
    println!("token username {}", username);
    println!("auth_header is {}", auth_header_str);
    println!("req.headers() is {:?}", request.headers());
    let status_code: u16;
    let user_response: Option<controller::openapi::identity_user::User>;
    let invoked_function_arn = context.invoked_function_arn;
    println!("invoked_function_arn: {:?}", invoked_function_arn);
    let user_table_name = get_user_table_name(invoked_function_arn);
    let request_post_function_name = get_post_function_name(request.uri().to_string());
    println!("function_name: {:?}", &request_post_function_name);
    let mut is_get_users = false;
    let user_collection: Option<UserCollection>;

    match *request.method() {
        method::Method::GET => {
            println!("Handle get method.");
            if let Some(id) = get_id_from_request(&request) {
                println!("Get user by id: {}", id.clone());
                user_collection = None;
                user_response = controller::get_user_by_id(id).await;
                status_code = 200;
                println!("Get user by id successfully: {}", id);
            } else {
                let query = get_query_from_request(&request);
                let username: Option<String> = query.username;
                let phone: Option<String> = query.phone;
                let email: Option<String> = query.email;
                let enabled: Option<bool> = query.enabled;
                let offset: Option<u16> = query.offset;
                let count: Option<u16> = query.count;
                println!(
                    "Search user with query with username {:?}, phone {:?},\
                email {:?}, enabled {:?}, offset {:?}, count {:?}",
                    username, phone, email, enabled, offset, count
                );
                user_collection = Some(
                    controller::get_users(username, phone, email, enabled, offset, count).await,
                );
                is_get_users = true;
                user_response = None;
                status_code = 200;
            }
        }
        method::Method::POST => match request_post_function_name {
            PostFunctionName::Activation => {
                let lambda_user_request: Option<UserUpdateRequest> =
                    request.payload().unwrap_or(None);
                let mut user = &lambda_user_request.unwrap();
                println!("Start activate user");
                let result = controller::activate_user(user.id.unwrap()).await;
                status_code = set_status_code(&result);
                user_response = result.map(Some).unwrap_or_else(|e| {
                    println!("{:?}", e);
                    None
                });
                user_collection = None;
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
                let lambda_user_request: Option<UserUpdateRequest> =
                    request.payload().unwrap_or(None);
                let mut user = &lambda_user_request.unwrap();
                let result = controller::deactivate_user(user.id.unwrap()).await;
                println!("deactivate user result:{:?}", &result);
                status_code = set_status_code(&result);
                user_response = result.map(Some).unwrap_or_else(|e| {
                    println!("{:?}", e);
                    None
                });
                user_collection = None;
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
                user_collection = None;
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
                user_collection = None;
                user_response = None;
                status_code = 404
            }
        },
        method::Method::PUT => {
            let id = get_id_from_request(&request);
            let value = request.payload().unwrap_or(None);
            if id.is_some() && value.is_some() {
                println!("Start update user info");
                let lambda_user_request: User = value.unwrap();
                let result = controller::update_user(id.unwrap(), &lambda_user_request).await;
                status_code = set_status_code(&result);
                user_response = result.map(Some).unwrap_or_else(|e| {
                    println!("error: {:?}", e);
                    None
                });
                let dynamodb_result = db_cognito::update_user_to_dynamodb(
                    Option::from(&user_response),
                    user_table_name.parse().unwrap(),
                )
                .await;
                println!("update user dynamodb result: {}", dynamodb_result);
                user_collection = None;
            } else if request.uri().to_string().contains("password-update") {
                println!("Update user password");
                let user_update_request = request.payload().unwrap_or(None);
                if user_update_request.is_some() {
                    let lambda_user_request: UserUpdateRequest = user_update_request.unwrap();
                    let user_result =
                        controller::get_user_by_id(lambda_user_request.id.unwrap()).await;
                    if user_result.is_none() {
                        print!("Password update user not found.");
                        user_collection = None;
                        user_response = None;
                        status_code = 404
                    } else {
                        let user = user_result.unwrap();
                        println!("Password update user found: {:?}", user);
                        let aws_client = Client::shared();
                        let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
                        let rusoto_cognito_idp_client =
                            CognitoIdentityProviderClient::new_with_client(
                                aws_client,
                                Region::ApSoutheast1,
                            );

                        let admin_set_user_password_request = AdminSetUserPasswordRequest {
                            password: lambda_user_request.plain_password.unwrap(),
                            permanent: None,
                            user_pool_id,
                            username: user.username,
                        };

                        let result_cognito = rusoto_cognito_idp_client
                            .admin_set_user_password(admin_set_user_password_request)
                            .await;
                        if result_cognito.is_ok() {
                            user_response =
                                controller::get_user_by_id(lambda_user_request.id.unwrap()).await;
                            status_code = 200;
                            user_collection = None;
                        } else {
                            print!("Error when update password: {:?}", result_cognito);
                            user_collection = None;
                            user_response = None;
                            status_code = 404
                        }
                    }
                } else {
                    user_collection = None;
                    user_response = None;
                    status_code = 404
                }
            } else {
                user_collection = None;
                user_response = None;
                status_code = 404
            }
        }
        _ => {
            user_collection = None;
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
        .body(if user_response.is_none() && user_collection.is_none() {
            Body::Empty
        } else {
            if is_get_users {
                serde_json::to_string(&user_collection)
            } else {
                serde_json::to_string(&user_response)
            }
            .expect("unable to serialize serde_json::Value")
            .into()
        })
        .expect("unable to build http::Response");
    println!(
        "final user response{:?}",
        serde_json::to_string(&user_response)
    );

    Ok(response)
}

fn set_status_code(result: &Result<User, UserMutationError>) -> u16 {
    match result {
        Ok(_) => 200,
        Err(UserMutationError::UniqueConstraintViolationError(..)) => 503,
        Err(UserMutationError::InvalidUser) => 405,
        Err(UserMutationError::InvalidEmail) => 405,
        Err(UserMutationError::InvalidPhone) => 405,
        Err(UserMutationError::NotExistedGroup) => 405,
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
                enabled: None,
                groups: None,
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

pub struct UserQuery {
    username: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    enabled: Option<bool>,
    offset: Option<u16>,
    count: Option<u16>,
}

pub fn get_query_from_request(req: &Request) -> UserQuery {
    let query = req.query_string_parameters();
    UserQuery {
        username: query.get("username").map(|str| str.to_string()),
        phone: query.get("phone").map(|str| str.to_string()),
        email: query.get("email").map(|str| str.to_string()),
        enabled: query
            .get("enabled")
            .map(|str| str.to_string().parse().unwrap()),
        offset: query.get("offset").map(|str| str.parse().unwrap()),
        count: query.get("count").map(|str| str.parse().unwrap()),
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
pub struct UserUpdateRequest {
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "plainPassword")]
    pub plain_password: Option<String>,
}
