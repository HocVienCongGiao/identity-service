use lambda_runtime::{Context, Error};
use rusoto_cognito_idp::{
    AdminCreateUserRequest, AdminDisableProviderForUserRequest, AdminDisableUserRequest,
    AttributeType, CognitoIdentityProvider, CognitoIdentityProviderClient,
    ProviderUserIdentifierType,
};
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::{Client, HttpClient, Region};
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, ListTablesInput, PutItemInput,
};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn func(event: Value, context: Context) -> Result<Value, Error> {
    println!("welcome to dynamodb processor!!!!");
    println!("Event payload is {:?}", event);

    let hash_key = event["Records"]
        .get(0)
        .and_then(|value| value.get("dynamodb"))
        .and_then(|value| value.get("Keys"))
        .and_then(|value| value.get("HashKey"))
        .and_then(|value| value.get("S"))
        .unwrap()
        .to_string();
    println!("hash_key: {}", hash_key);

    let function_name = context.invoked_function_arn;
    println!("function_name: {}", function_name);

    let user_table_name = if function_name.contains("prod") {
        "prod-sg_UserTable"
    } else {
        "dev-sg_UserTable"
    }
    .to_string();

    println!("Table name: {}", user_table_name);

    // Get item by hash key
    let client = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        EnvironmentProvider::default(),
        Region::ApSoutheast1,
    );

    // Filter condition
    let mut query_condition: HashMap<String, AttributeValue> = HashMap::new();
    query_condition.insert(
        String::from("HashKey"),
        AttributeValue {
            s: Option::from(hash_key.replace("\"", "")),
            ..Default::default()
        },
    );

    let user = client
        .get_item(GetItemInput {
            attributes_to_get: None,
            consistent_read: None,
            expression_attribute_names: None,
            key: query_condition,
            projection_expression: Option::from("id, username, email, phone, enabled".to_string()),
            return_consumed_capacity: None,
            table_name: user_table_name,
        })
        .sync();

    println!("dynamodb user: {:?}", user);

    let username_dynamodb = user
        .as_ref()
        .unwrap()
        .item
        .as_ref()
        .unwrap()
        .get("username")
        .and_then(|value| value.s.clone());

    let email_dynamodb = user
        .as_ref()
        .unwrap()
        .item
        .as_ref()
        .unwrap()
        .get("email")
        .and_then(|value| value.s.clone());

    let enabled = user
        .as_ref()
        .unwrap()
        .item
        .as_ref()
        .unwrap()
        .get("enabled")
        .and_then(|value| value.s.clone());

    // Insert user to cognito

    let aws_client = Client::shared();
    let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
    let rusoto_cognito_idp_client =
        CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

    let mut user_attributes: Vec<AttributeType> = vec![AttributeType {
        name: "email".to_string(),
        value: email_dynamodb,
    }];

    let mut event_name = event["Records"]
        .get(0)
        .and_then(|value| value.get("eventName"))
        .unwrap();
    println!("event_name: {}", event_name);
    if event_name.eq("INSERT") {
        let admin_create_user_request = AdminCreateUserRequest {
            desired_delivery_mediums: None,
            force_alias_creation: None,
            message_action: None,
            temporary_password: Option::from("Hvcg@123456789".to_string()),
            user_attributes: Option::from(user_attributes),
            user_pool_id,
            username: username_dynamodb.unwrap(),
            validation_data: None,
        };

        let result_cognito = rusoto_cognito_idp_client
            .admin_create_user(admin_create_user_request)
            .sync();

        Ok(json!({
            "message": format!("Cognito insert result, {:?}!", result_cognito.is_ok())
        }))
    } else if event_name.eq("MODIFY") {
        if enabled.unwrap() == *"false" {
            let admin_disable_user_request = AdminDisableUserRequest {
                user_pool_id,
                username: username_dynamodb.unwrap(),
            };

            let result_cognito = rusoto_cognito_idp_client
                .admin_disable_user(admin_disable_user_request)
                .sync();

            Ok(json!({
                "message":
                    format!(
                        "Cognito disable user result, {:?}!",
                        result_cognito.unwrap()
                    )
            }))
        } else {
            Ok(json!({ "message": "This user did not disable." }))
        }
    } else {
        Ok(json!({
            "message": "Delete function will be implemented later!"
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::func;
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
    async fn get_user_success() {
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
        let result = func(event, Default::default()).await;
        println!("Result: {:?}", result.is_err());

        assert!(!result.is_err());

        let aws_client = Client::shared();
        let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
        let rusoto_cognito_idp_client =
            CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

        let test_username = "nhutcargo001".to_string();
        let admin_delete_user_request = AdminDeleteUserRequest {
            user_pool_id,
            username: test_username,
        };

        let delete_result = rusoto_cognito_idp_client
            .admin_delete_user(admin_delete_user_request)
            .sync();
        println!("delete result: {:?}", delete_result);

        // Get item by hash key
        let client = DynamoDbClient::new_with(
            HttpClient::new().unwrap(),
            EnvironmentProvider::default(),
            Region::ApSoutheast1,
        );

        // Filter condition
        let mut query_condition: HashMap<String, AttributeValue> = HashMap::new();
        query_condition.insert(
            String::from("HashKey"),
            AttributeValue {
                s: Option::from(hash_key.replace("\"", "")),
                ..Default::default()
            },
        );
        let user_table_name = "dev-sg_UserTable".to_string();
        let user = client
            .get_item(GetItemInput {
                attributes_to_get: None,
                consistent_read: None,
                expression_attribute_names: None,
                key: query_condition,
                projection_expression: None,
                return_consumed_capacity: None,
                table_name: user_table_name,
            })
            .sync();
        println!("user : {:?}", user);
        let username = user
            .as_ref()
            .unwrap()
            .item
            .as_ref()
            .unwrap()
            .get("username")
            .and_then(|value| value.s.clone());
        println!("{}", username.as_ref().unwrap());
        let email = user
            .as_ref()
            .unwrap()
            .item
            .as_ref()
            .unwrap()
            .get("email")
            .and_then(|value| value.s.clone());
        println!("{}", email.as_ref().unwrap());

        let phone = user
            .unwrap()
            .item
            .unwrap()
            .get("phone")
            .and_then(|value| value.s.clone());
        println!("{}", phone.as_ref().unwrap());
        // Insert user to cognito
        let aws_client = Client::shared();
        let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
        let rusoto_cognito_idp_client =
            CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

        let mut user_attributes: Vec<AttributeType> = vec![AttributeType {
            name: "email".to_string(),
            value: email,
        }];

        let admin_create_user_request = AdminCreateUserRequest {
            desired_delivery_mediums: None,
            force_alias_creation: None,
            message_action: None,
            temporary_password: Option::from("Hvcg@123456789".to_string()),
            user_attributes: Option::from(user_attributes),
            user_pool_id,
            username: username.unwrap(),
            validation_data: None,
        };

        let result_cognito = rusoto_cognito_idp_client
            .admin_create_user(admin_create_user_request)
            .sync();

        println!("Result: {:?}", result_cognito)
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
}
