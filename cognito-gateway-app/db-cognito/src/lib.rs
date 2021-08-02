mod main;

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::time::SystemTime;

use chrono::prelude::*;
use hvcg_iam_openapi_identity::models::User;
use lambda_http::{Body, Context, IntoResponse, Request, RequestExt, Response};
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::{HttpClient, Region};
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, ListTablesInput, PutItemInput};
use uuid::Uuid;

pub async fn insert_user_to_dynamodb(user: Option<&User>, user_table_name: String) -> bool {
    let client = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        EnvironmentProvider::default(),
        Region::ApSoutheast1,
    );

    let user_dynamodb = user.unwrap();

    let mut user_attributes = HashMap::new();
    let random_uuid = Uuid::new_v4();
    println!("random uuid {}", random_uuid);

    user_attributes.insert(
        String::from("HashKey"),
        AttributeValue {
            s: Some(hash(user_dynamodb.id).to_string()),
            ..Default::default()
        },
    );

    user_attributes.insert(
        String::from("id"),
        AttributeValue {
            s: Some(user_dynamodb.id.unwrap().to_string()),
            ..Default::default()
        },
    );

    user_attributes.insert(
        String::from("username"),
        AttributeValue {
            s: Some(user_dynamodb.username.clone()),
            ..Default::default()
        },
    );

    user_attributes.insert(
        String::from("email"),
        AttributeValue {
            s: Some(user_dynamodb.email.clone().unwrap()),
            ..Default::default()
        },
    );

    user_attributes.insert(
        String::from("phone"),
        AttributeValue {
            s: Some(user_dynamodb.phone.clone().unwrap()),
            ..Default::default()
        },
    );

    let result = client
        .put_item(PutItemInput {
            table_name: user_table_name,
            item: user_attributes,
            ..PutItemInput::default()
        })
        .sync();

    if result.is_err() {
        println!("put_item() result {:#?}", result.as_ref().err());
        return false;
    }

    println!("put_item() result {:#?}", result.as_ref().unwrap());

    result.is_ok()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;

    use hvcg_iam_openapi_identity::models::User;
    use rusoto_core::credential::EnvironmentProvider;
    use rusoto_core::{HttpClient, Region};
    use rusoto_dynamodb::{
        AttributeValue, DynamoDb, DynamoDbClient, ListTablesInput, PutItemInput,
    };
    use uuid::Uuid;

    use crate::{hash, insert_user_to_dynamodb};

    #[tokio::test]
    async fn crud_users() {
        let table_name = "dev-sg_UserTable".to_string();

        let user_dynamodb = &User {
            id: Option::from(Uuid::new_v4()),
            username: "123".to_string(),
            email: Option::from(Uuid::new_v4().to_string()),
            phone: Option::from(Uuid::new_v4().to_string()),
        };

        let result = insert_user_to_dynamodb(Option::from(user_dynamodb), table_name).await;

        println!("insert to dynamo db result {}", result);
    }
}

fn hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
