mod main;

use hvcg_iam_openapi_identity::models::User;
use rusoto_cognito_idp::{
    AdminCreateUserRequest, AdminSetUserPasswordRequest, AdminUpdateAuthEventFeedbackRequest,
    AttributeType, CognitoIdentityProvider, CognitoIdentityProviderClient,
};
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::{Client, HttpClient, Region};
use rusoto_dynamodb::{
    AttributeValue, AttributeValueUpdate, DynamoDb, DynamoDbClient, PutItemInput, UpdateItemInput,
};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

pub async fn activate_user_to_dynamodb(user: Option<&User>, user_table_name: String) -> bool {
    let client = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        EnvironmentProvider::default(),
        Region::ApSoutheast1,
    );

    let user_dynamodb = user.unwrap();

    let mut user_key = HashMap::new();
    println!("user_id {}", user_dynamodb.id.unwrap());

    user_key.insert(
        String::from("HashKey"),
        AttributeValue {
            s: Some(hash(user_dynamodb.id).to_string()),
            ..Default::default()
        },
    );

    let mut attribute_updates = HashMap::new();
    attribute_updates.insert(
        String::from("enabled"),
        AttributeValueUpdate {
            action: Option::from("PUT".to_string()),
            value: Option::from(AttributeValue {
                s: Some("true".to_string()),
                ..Default::default()
            }),
        },
    );

    let result = client
        .update_item(UpdateItemInput {
            table_name: user_table_name,
            key: user_key,
            attribute_updates: Option::from(attribute_updates),
            ..UpdateItemInput::default()
        })
        .sync();

    if result.is_err() {
        println!("put_item() result {:#?}", result.as_ref().err());
        return false;
    }

    println!("put_item() result {:#?}", result.as_ref().unwrap());

    result.is_ok()
}

pub async fn insert_user_to_dynamodb(user: Option<&User>, user_table_name: String) -> bool {
    let client = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        EnvironmentProvider::default(),
        Region::ApSoutheast1,
    );

    let user_dynamodb = user.unwrap();

    let mut user_attributes = HashMap::new();
    println!("user_id: {}", user_dynamodb.id.unwrap());

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

    user_attributes.insert(
        String::from("enabled"),
        AttributeValue {
            s: Some("true".parse().unwrap()),
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

pub async fn deactivate_user_to_dynamodb(user: Option<&User>, user_table_name: String) -> bool {
    let client = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        EnvironmentProvider::default(),
        Region::ApSoutheast1,
    );

    let user_dynamodb = user.unwrap();

    let mut user_key = HashMap::new();
    println!("user_id {}", user_dynamodb.id.unwrap());

    user_key.insert(
        String::from("HashKey"),
        AttributeValue {
            s: Some(hash(user_dynamodb.id).to_string()),
            ..Default::default()
        },
    );

    let mut attribute_updates = HashMap::new();
    attribute_updates.insert(
        String::from("enabled"),
        AttributeValueUpdate {
            action: Option::from("PUT".to_string()),
            value: Option::from(AttributeValue {
                s: Some("false".to_string()),
                ..Default::default()
            }),
        },
    );

    let result = client
        .update_item(UpdateItemInput {
            table_name: user_table_name,
            key: user_key,
            attribute_updates: Option::from(attribute_updates),
            ..UpdateItemInput::default()
        })
        .sync();

    if result.is_err() {
        println!("put_item() result {:#?}", result.as_ref().err());
        return false;
    }

    println!("put_item() result {:#?}", result.as_ref().unwrap());

    result.is_ok()
}

pub async fn update_user_to_dynamodb(user: Option<&User>, user_table_name: String) -> bool {
    let client = DynamoDbClient::new_with(
        HttpClient::new().unwrap(),
        EnvironmentProvider::default(),
        Region::ApSoutheast1,
    );

    let user_dynamodb = user.unwrap();

    let mut user_key = HashMap::new();
    println!("user_id {}", user_dynamodb.id.unwrap());

    user_key.insert(
        String::from("HashKey"),
        AttributeValue {
            s: Some(hash(user_dynamodb.id).to_string()),
            ..Default::default()
        },
    );

    let mut attribute_updates = HashMap::new();
    attribute_updates.insert(
        String::from("username"),
        AttributeValueUpdate {
            action: Option::from("PUT".to_string()),
            value: Option::from(AttributeValue {
                s: Some(user_dynamodb.username.clone()),
                ..Default::default()
            }),
        },
    );

    attribute_updates.insert(
        String::from("phone"),
        AttributeValueUpdate {
            action: Option::from("PUT".to_string()),
            value: Option::from(AttributeValue {
                s: user_dynamodb.phone.clone(),
                ..Default::default()
            }),
        },
    );

    attribute_updates.insert(
        String::from("email"),
        AttributeValueUpdate {
            action: Option::from("PUT".to_string()),
            value: Option::from(AttributeValue {
                s: user_dynamodb.email.clone(),
                ..Default::default()
            }),
        },
    );

    let result = client
        .update_item(UpdateItemInput {
            table_name: user_table_name,
            key: user_key,
            attribute_updates: Option::from(attribute_updates),
            ..UpdateItemInput::default()
        })
        .sync();

    if result.is_err() {
        println!("put_item() result {:#?}", result.as_ref().err());
        return false;
    }

    println!("put_item() result {:#?}", result.as_ref().unwrap());

    result.is_ok()
}
pub async fn update_user_password(user: &User, plain_password: String) -> bool {
    let aws_client = Client::shared();
    let user_pool_id = "ap-southeast-1_9QWSYGzXk".to_string();
    let rusoto_cognito_idp_client =
        CognitoIdentityProviderClient::new_with_client(aws_client, Region::ApSoutheast1);

    let admin_set_user_password_request = AdminSetUserPasswordRequest {
        password: plain_password,
        permanent: None,
        user_pool_id,
        username: user.username.clone(),
    };

    let result_cognito = rusoto_cognito_idp_client
        .admin_set_user_password(admin_set_user_password_request)
        .sync();
    let is_ok_result = result_cognito.is_ok();
    println!("Update password result: {}", is_ok_result);
    is_ok_result
}

fn hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
