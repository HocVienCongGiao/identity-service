use lambda_runtime::{Context, Error};
use serde_json::{json, Value};
// use hvcg_iam_openapi_identity::models::User;
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::{HttpClient, Region};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, ListTablesInput, PutItemInput, GetItemInput};
use std::collections::HashMap;

pub async fn func(event: Value, _: Context) -> Result<Value, Error> {
    // let first_name = event["firstName"].as_str().unwrap_or("world");
    println!("welcome to dynamodb processor!!!!");
    println!("Event payload is {:?}", event);

    let hash_key = event["Records"].get(0)
        .and_then(|value| value.get("Keys"))
        .and_then(|value| value.get("HashKey"))
        .and_then(|value| value.get("S"))
        .unwrap().to_string();
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
    let user = client.get_item(
        GetItemInput {
            attributes_to_get: None,
            consistent_read: None,
            expression_attribute_names: None,
            key: query_condition,
            projection_expression: Option::from("id, username, email, phone".to_string()),
            return_consumed_capacity: None,
            table_name:user_table_name
        }
    ).sync();
    // upsert to cognito
    Ok(json!({ "message": format!("Hello, {:?}!", event) }))
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{json, Map, Value};
    use rusoto_core::credential::EnvironmentProvider;
    use rusoto_core::{HttpClient, Region};
    use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, ListTablesInput, PutItemInput, GetItemInput};
    use crate::func;
    use std::env;

    #[tokio::test]
    async fn create_user_success() {
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
                s: Option::from("6790795613568784684".to_string()),
                ..Default::default()
            },
        );

        let user_table_name = "dev-sg_UserTable".to_string();
        let user = client.get_item(
            GetItemInput {
                attributes_to_get: None,
                consistent_read: None,
                expression_attribute_names: None,
                key: query_condition,
                projection_expression: Option::from("hash_key, HashKey".to_string()),
                return_consumed_capacity: None,
                table_name: user_table_name
            }
        ).sync();

        println!("user_dynamodb {:?}", user.unwrap());

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