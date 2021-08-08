use lambda_runtime::{Context, Error};
use rusoto_cognito_idp::{AdminCreateUserRequest, AdminDisableUserRequest, AttributeType, CognitoIdentityProvider, CognitoIdentityProviderClient, AdminEnableUserRequest};
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::{Client, HttpClient, Region};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
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

    let invoked_function_arn = context.invoked_function_arn;
    println!("invoked_function_arn: {}", invoked_function_arn);

    let user_table_name = if invoked_function_arn.contains("prod") {
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

    let item = user.unwrap().item;
    if item.is_none() {
        println!("User not found");
        return Ok(json!({ "message": "User not found." }));
    }

    let username_dynamodb = item
        .as_ref()
        .unwrap()
        .get("username")
        .and_then(|value| value.s.clone());

    let email_dynamodb = item
        .as_ref()
        .unwrap()
        .get("email")
        .and_then(|value| value.s.clone());

    let enabled = item
        .as_ref()
        .unwrap()
        .get("enabled")
        .and_then(|value| value.s.clone());

    // Insert user to cognito

    let aws_client = Client::shared();
    let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
    let rusoto_cognito_idp_client =
        CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

    let user_attributes: Vec<AttributeType> = vec![AttributeType {
        name: "email".to_string(),
        value: email_dynamodb,
    }];

    let event_name = event["Records"]
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
            let admin_enabled_user_request = AdminEnableUserRequest {
                user_pool_id,
                username: username_dynamodb.unwrap(),
            };

            let result_cognito = rusoto_cognito_idp_client
                .admin_enable_user(admin_enabled_user_request)
                .sync();

            Ok(json!({
                "message":
                    format!(
                        "Cognito enable user result, {:?}!",
                        result_cognito.unwrap()
                    )
            }))
        }
    } else {
        Ok(json!({
            "message": "Delete function will be implemented later!"
        }))
    }
}
