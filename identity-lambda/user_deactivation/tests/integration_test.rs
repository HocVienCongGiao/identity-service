use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
    use std::ops::Add;
    use std::path::PathBuf;
    use std::sync::Once;

    use hvcg_iam_openapi_identity::models::User;
    use jsonwebtoken::TokenData;
    use lambda_http::http::HeaderValue;
    use lambda_http::{http, Context};
    use lambda_http::{Body, IntoResponse};
    use regex::Regex;
    use std::{env, thread};
    use uuid::Uuid;

    use crate::TokenPayload;
    use db_postgres::connect;
    use rusoto_core::credential::EnvironmentProvider;
    use rusoto_core::{HttpClient, Region};
    use rusoto_dynamodb::{
        AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, ListTablesInput,
        PutItemInput,
    };
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use std::thread::sleep;
    use std::time::Duration;
    use tokio_postgres::types::ToSql;
    use user_deactivation::func;

    static INIT: Once = Once::new();

    fn initialise() {
        INIT.call_once(|| {
            let my_path = PathBuf::new().join(".env.test");
            dotenv::from_path(my_path.as_path()).ok();
            println!(
                "testing env {}",
                std::env::var("HELLO").unwrap_or_else(|_| "".to_string())
            );
        });
    }

    #[tokio::test]
    async fn deactivate_user_success() {
        initialise();
        println!("is it working?");
        env::set_var(
            "AWS_ACCESS_KEY_ID",
            std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
        );
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        );

        let client = DynamoDbClient::new_with(
            HttpClient::new().unwrap(),
            EnvironmentProvider::default(),
            Region::ApSoutheast1,
        );

        // Given
        let deactivate_request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/users/deactivation")
            .method("POST")
            .header("Content-Type", "application/json")
            .header("authorization", "Bearer 123445")
            .body(Body::Text("{\n\"id\": \"a020aa98-4b40-4546-91cd-71503dfc14f0\"\n}".parse().unwrap()))
            .unwrap();

        let mut context: Context = Context::default();
        context.invoked_function_arn = "dev-sg_identity-service_users".to_string();

        let response = func(deactivate_request, context)
            .await
            .expect("expected Ok(_) value")
            .into_response();
        let deserialized_user: User = serde_json::from_slice(response.body()).unwrap();
        // Then
        assert_eq!(response.status(), 200);
    }
    fn hash<T>(obj: T) -> u64
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        obj.hash(&mut hasher);
        hasher.finish()
    }
}
