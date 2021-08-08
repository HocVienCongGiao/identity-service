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
    use std::env;
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
    use tokio_postgres::types::ToSql;
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
    async fn create_user_success() {
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
        let test_suffix = Uuid::new_v4().to_string();

        let user_request = User {
            id: None,
            username: "test_user".to_string() + &*test_suffix,
            email: Option::from("nhutcargo@gmail.com".to_string()),
            phone: Option::from("+84 939686970".to_string()),
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        let request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/user")
            .method("POST")
            .header("Content-Type", "application/json")
            .header("authorization", "Bearer 123445")
            .body(Body::from(serialized_user))
            .unwrap();

        let mut context: Context = Context::default();
        context.invoked_function_arn = "dev-sg_identity-service_users".to_string();

        let response = user::func(request, context)
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        assert_eq!(response.status(), 200);

        let deserialized_user: User = serde_json::from_slice(response.body()).unwrap();

        assert!(!deserialized_user.id.is_none(), true);
        assert_eq!(
            deserialized_user.username,
            "test_user".to_string() + &*test_suffix
        );
        assert_eq!(
            deserialized_user.email,
            Option::from("nhutcargo@gmail.com".to_string())
        );
        assert_eq!(
            deserialized_user.phone,
            Option::from("+84 939686970".to_string())
        );
        println!("Create user successfully!");

        // delete user in postgres
        let connect = connect().await;

        let stmt = (connect)
            .prepare(
                "truncate identity__user_username,\
             identity__user_phone, \
             identity__user_email, \
             identity__user_enabled, \
             identity__user",
            )
            .await
            .unwrap();

        let id: &[&(dyn ToSql + Sync)] = &[&deserialized_user.id];
        connect.query_one(&stmt, &[]).await;

        // delete user in dynamodb
        let hash_key = hash(deserialized_user.id);
        println!("hash_key: {}", hash_key);
        // Filter condition
        let mut query_condition: HashMap<String, AttributeValue> = HashMap::new();
        query_condition.insert(
            String::from("HashKey"),
            AttributeValue {
                s: Option::from(hash_key.to_string()),
                ..Default::default()
            },
        );

        let user_table_name = "dev-sg_UserTable".to_string();

        // TODO do not delete user in dynano db to avoid the processor retry
        // let result = client
        //     .delete_item(DeleteItemInput {
        //         condition_expression: None,
        //         conditional_operator: None,
        //         expected: None,
        //         expression_attribute_names: None,
        //         expression_attribute_values: None,
        //         key: query_condition,
        //         return_consumed_capacity: None,
        //         return_item_collection_metrics: None,
        //         return_values: None,
        //         table_name: user_table_name,
        //     })
        //     .sync();
        println!("trigger build!!!");
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
