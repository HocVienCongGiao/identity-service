extern crate rusoto_core;
extern crate rusoto_dynamodb;

use std::default::Default;
use std::time::SystemTime;

use hvcg_iam_openapi_identity::models::User;
use lambda_http::{Body, Context, IntoResponse, Request, RequestExt, Response};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput, PutItemInput, AttributeValue};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn insert_user_to_dynamodb(response_body: &Body) -> bool {
    // Create custom Region
    let region = Region::Custom {
        name: "us-east-1".to_owned(),
        endpoint: "http://localhost:8000".to_owned(),
    };

    let table_name = "UserTable".to_string();
    let client = DynamoDbClient::new(Region::ApSoutheast1);

    let deserialized_user: User = serde_json::from_slice(&*response_body).unwrap();
    // print the key for this book
    // requires bringing `dynomite::Item` into scope
    println!("user.id {:#?}", deserialized_user.id);

    let mut query = HashMap::new();
    query.insert(String::from("id"), AttributeValue {
        s: Some(Uuid::from(deserialized_user.id.unwrap()).to_string()),
        ..Default::default()
    });
    query.insert(String::from("username"), AttributeValue {
        s: Some(String::from(deserialized_user.username)),
        ..Default::default()
    });
    query.insert(String::from("email"), AttributeValue {
        s: Some(String::from(deserialized_user.email.unwrap()).to_string()),
        ..Default::default()
    });
    query.insert(String::from("phone"), AttributeValue {
        s: Some(String::from(deserialized_user.phone.unwrap()).to_string()),
        ..Default::default()
    });

    let result = client
        .put_item(PutItemInput {
            table_name: table_name.clone(),
            item: query,
            ..PutItemInput::default()
        }).sync();

    println!("put_item() result {:#?}", result.is_ok());
    return result.is_ok()
}

#[cfg(test)]
mod tests {
    use hvcg_iam_openapi_identity::models::User;
    use lambda_http::{Body, Response};
    use std::collections::HashMap;
    use rusoto_dynamodb::AttributeValue;

    #[tokio::test]
    async fn crud_users() {
        crate::insert_user_to_dynamodb(&Body::Empty).await;
        let result = 4;
        assert_eq!(result, 4);
    }
}
