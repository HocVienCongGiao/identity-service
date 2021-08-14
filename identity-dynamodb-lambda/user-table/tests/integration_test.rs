use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use lambda_runtime::{Context, Error};
    use rusoto_cognito_idp::{
        AdminCreateUserRequest, AdminDeleteUserRequest, AdminSetUserPasswordRequest, AttributeType,
        CognitoIdentityProvider, CognitoIdentityProviderClient,
    };
    use rusoto_core::credential::EnvironmentProvider;
    use rusoto_core::{Client, HttpClient, Region};
    use rusoto_dynamodb::{
        AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, ListTablesInput, PutItemInput,
    };
    use serde_json::{json, Map, Value};
    use std::env;
    use std::path::PathBuf;
    use std::sync::Once;
    use user_table::func;

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
        env::set_var(
            "AWS_ACCESS_KEY_ID",
            std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
        );
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        );

        let mut records: Map<String, Value> = Default::default();
        let mut aws_object: Map<String, Value> = Default::default();
        let mut hash_key_object: Map<String, Value> = Default::default();
        let mut hash_key_object_details: Map<String, Value> = Default::default();
        let mut key_object: Map<String, Value> = Default::default();

        hash_key_object_details.insert(
            "S".to_string(),
            Value::String("5285243916027018465".to_string()),
        );
        hash_key_object.insert(
            "HashKey".to_string(),
            Value::Object(hash_key_object_details),
        );

        key_object.insert("Keys".to_string(), Value::Object(hash_key_object));

        // "Keys": Object({"HashKey": Object({"S": String("11905088586532604268")})})
        aws_object.insert("dynamodb".to_string(), Value::Object(key_object));
        aws_object.insert("eventName".to_string(), Value::String("INSERT".to_string()));

        records.insert(
            "Records".parse().unwrap(),
            Value::Array(vec![Value::Object(aws_object)]),
        );

        let event = Value::Object(records);

        println!("event_nhut: {:?}", event);

        let mut hash_key = event["Records"]
            .get(0)
            .and_then(|value| value.get("dynamodb"))
            .and_then(|value| value.get("Keys"))
            .and_then(|value| value.get("HashKey"))
            .and_then(|value| value.get("S"))
            .unwrap()
            .to_string();
        println!("hash_key: {}", hash_key.replace("\"", ""));
        let event_name = event["Records"]
            .get(0)
            .and_then(|value| value.get("eventName"))
            .unwrap()
            .to_string();
        println!("event_name: {}", event_name);
        let mut context = Context::default();
        context.invoked_function_arn = "dev".to_string();
        let result = func(event, context).await;
        println!("Result is ok?: {:?}", result.is_ok());

        assert!(!result.is_err());

        let aws_client = Client::shared();
        let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
        let rusoto_cognito_idp_client =
            CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

        let test_username = "user_donot_delete_create".to_string();
        let admin_delete_user_request = AdminDeleteUserRequest {
            user_pool_id,
            username: test_username,
        };

        let delete_result = rusoto_cognito_idp_client
            .admin_delete_user(admin_delete_user_request)
            .sync();
        println!("delete result: {:?}", delete_result);
    }

    #[tokio::test]
    async fn disable_user_success() {
        initialise();
        env::set_var(
            "AWS_ACCESS_KEY_ID",
            std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
        );
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        );

        let mut records: Map<String, Value> = Default::default();
        let mut aws_object: Map<String, Value> = Default::default();
        let mut hash_key_object: Map<String, Value> = Default::default();
        let mut hash_key_object_details: Map<String, Value> = Default::default();
        let mut key_object: Map<String, Value> = Default::default();

        hash_key_object_details.insert(
            "S".to_string(),
            Value::String("11905088586532604268".to_string()),
        );
        hash_key_object.insert(
            "HashKey".to_string(),
            Value::Object(hash_key_object_details),
        );

        key_object.insert("Keys".to_string(), Value::Object(hash_key_object));

        // "Keys": Object({"HashKey": Object({"S": String("11905088586532604268")})})
        aws_object.insert("dynamodb".to_string(), Value::Object(key_object));
        aws_object.insert("eventName".to_string(), Value::String("MODIFY".to_string()));

        records.insert(
            "Records".parse().unwrap(),
            Value::Array(vec![Value::Object(aws_object)]),
        );

        let event = Value::Object(records);

        let mut context: Context = Context::default();
        context.invoked_function_arn = "dev-sg_identity-service_users".to_string();
        let result = func(event, context).await;
        print!("Disabled user result: {:?}", result.unwrap());
    }

    #[tokio::test]
    async fn update_user_success() {
        initialise();
        env::set_var(
            "AWS_ACCESS_KEY_ID",
            std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
        );
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        );

        let mut records: Map<String, Value> = Default::default();
        let mut aws_object: Map<String, Value> = Default::default();
        let mut hash_key_object: Map<String, Value> = Default::default();
        let mut hash_key_object_details: Map<String, Value> = Default::default();
        let mut key_object: Map<String, Value> = Default::default();

        hash_key_object_details.insert(
            "S".to_string(),
            Value::String("12358323330878084114".to_string()),
        );
        hash_key_object.insert(
            "HashKey".to_string(),
            Value::Object(hash_key_object_details),
        );

        key_object.insert("Keys".to_string(), Value::Object(hash_key_object));

        aws_object.insert("dynamodb".to_string(), Value::Object(key_object));
        aws_object.insert("eventName".to_string(), Value::String("MODIFY".to_string()));

        records.insert(
            "Records".parse().unwrap(),
            Value::Array(vec![Value::Object(aws_object)]),
        );

        let event = Value::Object(records);

        let mut context: Context = Context::default();
        context.invoked_function_arn = "dev-sg_identity-service_users".to_string();
        let result = func(event, context).await;
        print!("Update user result: {:?}", result.unwrap());
    }

    #[tokio::test]
    async fn user_not_found() {
        initialise();
        env::set_var(
            "AWS_ACCESS_KEY_ID",
            std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
        );
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        );

        let mut records: Map<String, Value> = Default::default();
        let mut aws_object: Map<String, Value> = Default::default();
        let mut hash_key_object: Map<String, Value> = Default::default();
        let mut hash_key_object_details: Map<String, Value> = Default::default();
        let mut key_object: Map<String, Value> = Default::default();

        hash_key_object_details.insert(
            "S".to_string(),
            Value::String("123456789".to_string()),
        );
        hash_key_object.insert(
            "HashKey".to_string(),
            Value::Object(hash_key_object_details),
        );

        key_object.insert("Keys".to_string(), Value::Object(hash_key_object));

        aws_object.insert("dynamodb".to_string(), Value::Object(key_object));
        aws_object.insert("eventName".to_string(), Value::String("MODIFY".to_string()));

        records.insert(
            "Records".parse().unwrap(),
            Value::Array(vec![Value::Object(aws_object)]),
        );

        let event = Value::Object(records);

        let mut context: Context = Context::default();
        context.invoked_function_arn = "dev-sg_identity-service_users".to_string();
        let result = func(event, context).await;
        print!("Update user result: {:?}", result.unwrap());
    }
}
