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
    // let first_name = event["firstName"].as_str().unwrap_or("world");
    println!("welcome to dynamodb processor!!!!");
    println!("Event payload is {:?}", event);

    let hash_key = event["Records"]
        .get(0)
        .and_then(|value| value.get("Keys"))
        .and_then(|value| value.get("HashKey"))
        .and_then(|value| value.get("S"))
        .unwrap()
        .to_string();
    println!("hash_key: {}", hash_key);

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
            s: Option::from(hash_key),
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
    let user_pool_id = "ap-southeast-1_vmFHg7JIC".to_string();
    let rusoto_cognito_idp_client =
        CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

    let mut user_attributes: Vec<AttributeType> = vec![
        AttributeType {
            name: "email".to_string(),
            value: email,
        },
        AttributeType {
            name: "phone".to_string(),
            value: phone,
        },
    ];

    let admin_create_user_request = AdminCreateUserRequest {
        desired_delivery_mediums: None,
        force_alias_creation: None,
        message_action: None,
        temporary_password: None,
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
    use rusoto_core::credential::EnvironmentProvider;
    use rusoto_core::{HttpClient, Region};
    use rusoto_dynamodb::{
        AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, ListTablesInput, PutItemInput,
    };
    use serde_json::{json, Map, Value};
    use std::env;

    #[tokio::test]
    async fn create_user_success() {
        env::set_var("AWS_ACCESS_KEY_ID", "AKIA47GDVJO6TNULSQ73");
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            "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",
        );
        // Get item by hash key
        let client = DynamoDbClient::new_with(
            HttpClient::new().unwrap(),
            EnvironmentProvider::default(),
            Region::ApSoutheast1,
        );

        // Filter condition

        let mut query_condition = HashMap::new();
        query_condition.insert(
            String::from("HashKey"),
            AttributeValue {
                s: Option::from("6790795613568784684".to_string()),
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

        let test = user
            .unwrap()
            .item
            .unwrap()
            .get("username")
            .and_then(|value| value.s.clone())
            .unwrap();

        println!("user_dynamodb {:?}", test);

        // let mut records: Map<String, Value> = Default::default();
        // let mut aws_object: Map<String, Value> = Default::default();
        // let mut hash_key_object: Map<String, Value> = Default::default();
        // let mut hash_key_object_details: Map<String, Value> = Default::default();
        // hash_key_object_details.insert("S".to_string(), Value::String("11905088586532604268".to_string()));
        // hash_key_object.insert("HashKey".to_string(),
        //                        Value::Object(hash_key_object_details));
        // // "Keys": Object({"HashKey": Object({"S": String("11905088586532604268")})})
        // aws_object.insert("Keys".to_string(), Value::Object(hash_key_object));
        // records.insert(
        //     "Records".parse().unwrap(),
        //     Value::Array(
        //         vec![Value::Object(aws_object)]
        //     )
        // );

        // Testing purpose
        // let hash_key = records["Records"].get(0)
        //     .and_then(|value| value.get("Keys"))
        //     .and_then(|value| value.get("HashKey"))
        //     .and_then(|value| value.get("S"))
        //     .unwrap().to_string();
        // let hostname: Option<&str> = records.get("Records")
        //     .and_then(|value| value.get(0))
        //     .and_then(|value| value.get("HashKey"))
        //     .and_then(|value| value.as_str());
        // nhut_hash_key: {"Records": Array([Object({"Keys":
        // Object({"HashKey": Object({"S": String("11905088586532604268")})})})])}
        // println!("nhut_hash_key: {:?}", hash_key);

        // let value = Value::Object(
        //     records
        // );
        // let result = func(value, Default::default()).await;
    }
}
