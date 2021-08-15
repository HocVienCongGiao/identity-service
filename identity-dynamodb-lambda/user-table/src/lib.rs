use lambda_runtime::{Context, Error};
use rusoto_cognito_idp::{
    AdminCreateUserRequest, AdminDisableUserRequest, AdminEnableUserRequest,
    AdminUpdateUserAttributesRequest, AttributeType, CognitoIdentityProvider,
    CognitoIdentityProviderClient,
};
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::{Client, HttpClient, Region};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::str::FromStr;
use strum_macros::EnumString;

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

    let cognito_user_pool_id = get_user_pool_id(invoked_function_arn);
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
            projection_expression: None,
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
    let rusoto_cognito_idp_client =
        CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

    let email_user_attribute = AttributeType {
        name: "email".to_string(),
        value: email_dynamodb,
    };

    let event_name = event["Records"]
        .get(0)
        .and_then(|value| value.get("eventName"))
        .unwrap()
        .as_str();

    if event_name.unwrap() == "INSERT" {
        println!("Start insert user to cognito");
        let user_attributes: Vec<AttributeType> = vec![email_user_attribute];
        let admin_create_user_request = AdminCreateUserRequest {
            desired_delivery_mediums: None,
            force_alias_creation: None,
            message_action: None,
            temporary_password: Option::from("Hvcg@123456789".to_string()),
            user_attributes: Option::from(user_attributes),
            user_pool_id: cognito_user_pool_id,
            username: username_dynamodb.unwrap(),
            validation_data: None,
        };

        let result_cognito = rusoto_cognito_idp_client
            .admin_create_user(admin_create_user_request)
            .sync();

        Ok(json!({
            "message": format!("Cognito insert result, {:?}!", result_cognito.is_ok())
        }))
    } else if event_name.unwrap() == "MODIFY" {
        println!("Start update user to cognito");
        let user_name = username_dynamodb.unwrap();
        let update_user_attributes: Vec<AttributeType> = vec![email_user_attribute];

        let admin_user_attribute_update_request = AdminUpdateUserAttributesRequest {
            user_attributes: update_user_attributes,
            user_pool_id: cognito_user_pool_id.clone(),
            username: user_name.clone(),
        };

        let result_cognito = rusoto_cognito_idp_client
            .admin_update_user_attributes(admin_user_attribute_update_request)
            .sync();

        if enabled.unwrap() == *"false" {
            let admin_disable_user_request = AdminDisableUserRequest {
                user_pool_id: cognito_user_pool_id,
                username: user_name,
            };

            let result_cognito = rusoto_cognito_idp_client
                .admin_disable_user(admin_disable_user_request)
                .sync();

            println!(
                "Cognito disable user result, {:?}!",
                result_cognito.unwrap()
            );
        } else {
            let admin_enabled_user_request = AdminEnableUserRequest {
                user_pool_id: cognito_user_pool_id,
                username: user_name,
            };

            let result_cognito = rusoto_cognito_idp_client
                .admin_enable_user(admin_enabled_user_request)
                .sync();

            println!(
                "Cognito enabled user result, {:?}!",
                result_cognito.unwrap()
            );
        }

        Ok(json!({
            "message": format!("Cognito update user result, {:?}!", result_cognito.unwrap())
        }))
    } else {
        println!("Start delete user");
        Ok(json!({
            "message": "Delete function will be implemented later!"
        }))
    }
}

fn get_user_pool_id(invoked_function_arn: String) -> String {
    if invoked_function_arn.contains("prod") {
        "ap-southeast-1_03CBODhXO"
    } else {
        "ap-southeast-1_9QWSYGzXk"
    }
    .to_string()
}

#[derive(Debug, PartialEq)]
enum EventName {
    Insert,
    Modify,
    Delete,
    Unknown,
}

fn get_event_name(event_name: String) -> EventName {
    if event_name == "INSERT" {
        EventName::Insert
    } else if event_name == "MODIFY" {
        EventName::Modify
    } else if event_name == "DELETE" {
        EventName::Delete
    } else {
        EventName::Unknown
    }
}
