use lambda_runtime::{Context, Error};
use rusoto_cognito_idp::{
    AdminAddUserToGroupRequest, AdminCreateUserRequest, AdminDisableUserRequest,
    AdminEnableUserRequest, AdminListGroupsForUserRequest, AdminRemoveUserFromGroupRequest,
    AdminUpdateUserAttributesRequest, AttributeType, CognitoIdentityProvider,
    CognitoIdentityProviderClient,
};
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::{Client, HttpClient, Region};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use serde_json::{json, Value};
use std::collections::HashMap;
pub async fn func(event: Value, context: Context) -> Result<Value, Error> {
    let temp_password: String = "Hvcg@123456789".to_string();
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
        .await;

    println!("dynamodb user: {:?}", user);

    let item = user.unwrap().item;
    if item.is_none() {
        println!("User not found");
        return Ok(json!({ "message": "User not found." }));
    }

    if item.as_ref().unwrap().get("username").is_none()
        || item.as_ref().unwrap().get("email").is_none()
        || item.as_ref().unwrap().get("groups").is_none()
    {
        println!("Username or email or group not found.");
        return Ok(json!({ "message": "Username or email or enabled not found." }));
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

    let groups = item
        .as_ref()
        .unwrap()
        .get("groups")
        .and_then(|value| value.ss.clone());
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
            client_metadata: None,
            desired_delivery_mediums: None,
            force_alias_creation: None,
            message_action: None,
            temporary_password: Option::from(temp_password),
            user_attributes: Option::from(user_attributes),
            user_pool_id: cognito_user_pool_id.clone(),
            username: username_dynamodb.clone().unwrap(),
            validation_data: None,
        };

        let result_cognito = rusoto_cognito_idp_client
            .admin_create_user(admin_create_user_request)
            .await;

        let user_groups = groups.unwrap();

        if user_groups.is_empty() {
            println!("Groups not found.");
            return Ok(json!({ "message": "Groups is empty".to_string()}));
        }
        for group_name in user_groups {
            println!("group_name: {}", group_name.clone());
            if group_name.is_empty() {
                println!("group name is empty");
                return Ok(json!({
                    "message": "Group name is empty"
                }));
            }
            let admin_add_user_to_group = AdminAddUserToGroupRequest {
                group_name: group_name.clone(),
                user_pool_id: cognito_user_pool_id.clone(),
                username: username_dynamodb.clone().unwrap(),
            };
            let result = rusoto_cognito_idp_client
                .admin_add_user_to_group(admin_add_user_to_group)
                .await;
            if result.is_err() {
                println!("error while add user to group: {:?}", result);
                return Ok(json!({
                    "message": format!("Error while add user to group {}", group_name)
                }));
            }
        }

        Ok(json!({
            "message": format!("Cognito insert result, {:?}!", result_cognito.is_ok())
        }))
    } else if event_name.unwrap() == "MODIFY" {
        println!("Start update user to cognito");
        let user_name = username_dynamodb.unwrap();
        let update_user_attributes: Vec<AttributeType> = vec![email_user_attribute];

        let admin_user_attribute_update_request = AdminUpdateUserAttributesRequest {
            client_metadata: None,
            user_attributes: update_user_attributes,
            user_pool_id: cognito_user_pool_id.clone(),
            username: user_name.clone(),
        };

        let result_cognito = rusoto_cognito_idp_client
            .admin_update_user_attributes(admin_user_attribute_update_request)
            .await;
        if result_cognito.is_err() {
            return Ok(json!({
                "message":
                    format!(
                        "Error when update user result, {:?}!",
                        result_cognito.unwrap()
                    )
            }));
        }

        let user_groups = groups.unwrap();

        if user_groups.is_empty() {
            println!("Groups not found.");
            return Ok(json!({
                "message": "Groups is empty.".to_string()
            }));
        }
        update_cognito_user_group(
            user_name.clone(),
            cognito_user_pool_id.clone(),
            user_groups,
            rusoto_cognito_idp_client.clone(),
        )
        .await;

        if enabled.unwrap() == *"false" {
            let admin_disable_user_request = AdminDisableUserRequest {
                user_pool_id: cognito_user_pool_id,
                username: user_name,
            };

            let result_cognito = rusoto_cognito_idp_client
                .admin_disable_user(admin_disable_user_request)
                .await;

            if result_cognito.is_err() {
                return Ok(json!({
                    "message":
                        format!(
                            "Error when deactivate user result, {:?}!",
                            result_cognito.unwrap()
                        )
                }));
            }
        } else {
            let admin_enabled_user_request = AdminEnableUserRequest {
                user_pool_id: cognito_user_pool_id,
                username: user_name,
            };

            let result_cognito = rusoto_cognito_idp_client
                .admin_enable_user(admin_enabled_user_request)
                .await;

            if result_cognito.is_err() {
                return Ok(json!({
                    "message":
                        format!(
                            "Error when activate user result, {:?}!",
                            result_cognito.unwrap()
                        )
                }));
            }
        }

        Ok(json!({
            "message": "Cognito update user result success".to_string()
        }))
    } else {
        println!("Start delete user");
        Ok(json!({
            "message": "Delete function will be implemented later!"
        }))
    }
}

async fn update_cognito_user_group(
    user_name: String,
    cognito_user_pool_id: String,
    user_groups: Vec<String>,
    rusoto_cognito_idp_client: CognitoIdentityProviderClient,
) -> bool {
    let mut result = true;
    // get user
    let admin_list_group_for_user = AdminListGroupsForUserRequest {
        limit: None,
        next_token: None,
        user_pool_id: cognito_user_pool_id.clone(),
        username: user_name.clone(),
    };

    let admin_list_group_for_user_result = rusoto_cognito_idp_client
        .admin_list_groups_for_user(admin_list_group_for_user)
        .await;
    let current_group = admin_list_group_for_user_result.unwrap().groups.unwrap();
    for group in current_group {
        let remove_group_result = rusoto_cognito_idp_client
            .admin_remove_user_from_group(AdminRemoveUserFromGroupRequest {
                group_name: group.group_name.as_ref().unwrap().clone(),
                user_pool_id: cognito_user_pool_id.clone(),
                username: user_name.clone(),
            })
            .await;
        if remove_group_result.is_err() {
            println!("Error while removing group: {:?}", group);
            result = false;
        }
    }

    for group_name in user_groups {
        println!("group_name: {}", group_name.clone());
        if group_name.is_empty() {
            println!("group name is empty");
            result = false
        }
        let admin_add_user_to_group = AdminAddUserToGroupRequest {
            group_name: group_name.clone(),
            user_pool_id: cognito_user_pool_id.clone(),
            username: user_name.clone(),
        };
        let result_add_user_group = rusoto_cognito_idp_client
            .admin_add_user_to_group(admin_add_user_to_group)
            .await;
        if result_add_user_group.is_err() {
            println!("error while add user to group: {:?}", result);
            result = false;
        }
    }
    result
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
