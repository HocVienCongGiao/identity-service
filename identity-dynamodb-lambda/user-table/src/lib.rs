use lambda_runtime::{Context, Error};
use rusoto_cognito_idp::{
    AdminCreateUserRequest, AttributeType, CognitoIdentityProvider, CognitoIdentityProviderClient,
};
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::{Client, HttpClient, Region};
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, ListTablesInput, PutItemInput,
};
use serde_json::{json, Value};
use std::collections::HashMap;

pub async fn func(event: Value, _: Context) -> Result<Value, Error> {
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

    // Get item by hash key
    // let client = DynamoDbClient::new_with(
    //     HttpClient::new().unwrap(),
    //     EnvironmentProvider::default(),
    //     Region::ApSoutheast1,
    // );

    let client = DynamoDbClient::new(Region::ApSoutheast1);

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
            projection_expression: Option::from("id, username, email, phone".to_string()),
            return_consumed_capacity: None,
            table_name: user_table_name,
        })
        .sync();
    let username = user
        .as_ref()
        .unwrap()
        .item
        .as_ref()
        .unwrap()
        .get("username")
        .and_then(|value| value.s.clone());
    let email = user
        .as_ref()
        .unwrap()
        .item
        .as_ref()
        .unwrap()
        .get("email")
        .and_then(|value| value.s.clone());
    let phone = user
        .unwrap()
        .item
        .unwrap()
        .get("phone")
        .and_then(|value| value.s.clone());
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
    if result_cognito.is_err() {
        println!("Error: {:?}", result_cognito.as_ref().err());
    }

    Ok(json!({
        "message": format!("Cognito insert result, {:?}!", result_cognito.unwrap())
    }))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::func;
    use rusoto_cognito_idp::{
        AdminCreateUserRequest, AdminSetUserPasswordRequest, AttributeType,
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
            println!("testing env {}", std::env::var("HELLO").unwrap());
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
            Value::String("11905088586532604268".to_string()),
        );
        hash_key_object.insert(
            "HashKey".to_string(),
            Value::Object(hash_key_object_details),
        );

        key_object.insert("Keys".to_string(), Value::Object(hash_key_object));

        // "Keys": Object({"HashKey": Object({"S": String("11905088586532604268")})})
        aws_object.insert("dynamodb".to_string(), Value::Object(key_object));

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

        // // Get item by hash key
        // let client = DynamoDbClient::new_with(
        //     HttpClient::new().unwrap(),
        //     EnvironmentProvider::default(),
        //     Region::ApSoutheast1,
        // );
        //
        // // Filter condition
        // let mut query_condition: HashMap<String, AttributeValue> = HashMap::new();
        // query_condition.insert(
        //     String::from("HashKey"),
        //     AttributeValue {
        //         s: Option::from(hash_key.replace("\"", "")),
        //         ..Default::default()
        //     },
        // );
        // let user_table_name = "dev-sg_UserTable".to_string();
        // let user = client
        //     .get_item(GetItemInput {
        //         attributes_to_get: None,
        //         consistent_read: None,
        //         expression_attribute_names: None,
        //         key: query_condition,
        //         projection_expression: None,
        //         return_consumed_capacity: None,
        //         table_name: user_table_name,
        //     })
        //     .sync();
        // println!("user : {:?}", user);
        // let username = user
        //     .as_ref()
        //     .unwrap()
        //     .item
        //     .as_ref()
        //     .unwrap()
        //     .get("username")
        //     .and_then(|value| value.s.clone());
        // println!("{}", username.as_ref().unwrap());
        // let email = user
        //     .as_ref()
        //     .unwrap()
        //     .item
        //     .as_ref()
        //     .unwrap()
        //     .get("email")
        //     .and_then(|value| value.s.clone());
        // println!("{}", email.as_ref().unwrap());
        // let phone = user
        //     .unwrap()
        //     .item
        //     .unwrap()
        //     .get("phone")
        //     .and_then(|value| value.s.clone());
        // println!("{}", phone.as_ref().unwrap());
        // // Insert user to cognito
        // let aws_client = Client::shared();
        // let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
        // let rusoto_cognito_idp_client =
        //     CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);
        //
        // let mut user_attributes: Vec<AttributeType> = vec![AttributeType {
        //     name: "email".to_string(),
        //     value: email,
        // }];
        //
        // let admin_create_user_request = AdminCreateUserRequest {
        //     desired_delivery_mediums: None,
        //     force_alias_creation: None,
        //     message_action: None,
        //     temporary_password: Option::from("Hvcg@123456789".to_string()),
        //     user_attributes: Option::from(user_attributes),
        //     user_pool_id,
        //     username: username.unwrap(),
        //     validation_data: None,
        // };
        //
        // let result_cognito = rusoto_cognito_idp_client
        //     .admin_create_user(admin_create_user_request)
        //     .sync();
        // if result_cognito.is_err() {
        //     println!("Error: {:?}", result_cognito.as_ref().err());
        // }
        //
        // println!("Result: {:?}", result_cognito.unwrap())
    }

    // #[tokio::test]
    async fn create_user_success_single() {
        initialise();
        env::set_var(
            "AWS_ACCESS_KEY_ID",
            std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
        );
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        );

        let username = "nhuthm005".to_string();

        let email = "nhuthm005@gmail.com".to_string();

        let password = "Hvcg@123456789".to_string();

        // Insert user to cognito
        let aws_client = Client::shared();
        let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
        let rusoto_cognito_idp_client =
            CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

        let mut user_attributes: Vec<AttributeType> = vec![AttributeType {
            name: "email".to_string(),
            value: Option::from(email),
        }];

        let admin_create_user_request = AdminCreateUserRequest {
            desired_delivery_mediums: None,
            force_alias_creation: None,
            message_action: None,
            temporary_password: Option::from(password),
            user_attributes: Option::from(user_attributes),
            user_pool_id,
            username,
            validation_data: None,
        };

        let result_cognito = rusoto_cognito_idp_client
            .admin_create_user(admin_create_user_request)
            .sync();
        if result_cognito.is_err() {
            println!("Error: {:?}", result_cognito.as_ref().err());
        }

        println!("Result: {:?}", result_cognito.unwrap())
    }
}
