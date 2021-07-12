use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
struct TokenPayload {
    // Despite the struct field being named `username`, it is going to come
    // from a JSON field called `cognito:username`.
    #[serde(rename(deserialize = "cognito:username"))]
    username: String,
    #[serde(rename(deserialize = "cognito:groups"))]
    groups: Vec<String>,
}

#[cfg(test)]
mod tests {
    use crate::TokenPayload;
    use hvcg_iam_openapi_identity::models::User;
    use jsonwebtoken::TokenData;
    use lambda_http::http::HeaderValue;
    use lambda_http::{http, Context};
    use lambda_http::{Body, IntoResponse};
    use std::ops::Add;
    use std::path::PathBuf;
    use std::sync::Once;
    use uuid::Uuid;

    static INIT: Once = Once::new();

    fn initialise() {
        INIT.call_once(|| {
            let my_path = PathBuf::new().join(".env.test");
            dotenv::from_path(my_path.as_path()).ok();
            println!("testing env {}", std::env::var("HELLO").unwrap());
        });
    }

    #[tokio::test]
    async fn create_user_success() {
        initialise();
        println!("is it working?");

        // Given
        let test_suffix = Uuid::new_v4().to_string();

        let user_request = User {
            id: None,
            username: "test_user".to_string() + &*test_suffix,
            email: Option::from("nhut_cargo@gmail.com".to_string() + &*test_suffix),
            phone: Option::from("+84 909686868".to_string() + &*test_suffix),
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        // test authorization
        let token = std::env::var("TOKEN_PREFIX")
            .unwrap()
            .add(" ")
            .add(&*std::env::var("TOKEN").unwrap());
        println!("full token: {}", token);

        let request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/user")
            .method("POST")
            .header("Content-Type", "application/json")
            .header("authorization", token)
            .body(Body::from(serialized_user))
            .unwrap();

        let auth_header_value = request.headers().get("authorization").unwrap();

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

        let response = user::create_user(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        assert_eq!(response.status(), 200);

        let deserialized_user: User = serde_json::from_slice(response.body()).unwrap();

        assert_eq!(!deserialized_user.id.is_none(), true);
        assert_eq!(
            deserialized_user.username,
            "test_user".to_string() + &*test_suffix
        );
        assert_eq!(
            deserialized_user.email,
            Option::from("nhut_cargo@gmail.com".to_string() + &*test_suffix)
        );
        assert_eq!(
            deserialized_user.phone,
            Option::from("+84 909686868".to_string() + &*test_suffix)
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn create_user_failed() {
        // Given
        let test_suffix = Uuid::new_v4().to_string();
        let user_request = User {
            id: None,
            username: "test".to_string().add(&*test_suffix),
            email: None,
            phone: None,
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        let request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/user")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(serialized_user))
            .unwrap();

        // When
        user::create_user(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        // Checking error log
        println!("create_user_failed completed")
    }
}
