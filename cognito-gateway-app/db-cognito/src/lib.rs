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
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;

    use crate::{deactivate_user_to_dynamodb, hash, insert_user_to_dynamodb, update_user_password};
    use hvcg_iam_openapi_identity::models::User;
    use rusoto_core::credential::EnvironmentProvider;
    use rusoto_core::{HttpClient, Region};
    use rusoto_dynamodb::{
        AttributeValue, DynamoDb, DynamoDbClient, ListTablesInput, PutItemInput,
    };
    use std::ops::Add;
    use std::path::PathBuf;
    use std::sync::Once;
    use uuid::Uuid;
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
    async fn crud_users() {
        initialise();
        println!("is it working?");
        env::set_var(
            "AWS_ACCESS_KEY_ID",
            std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
        );
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        );

        let table_name = "dev-sg_UserTable".to_string();

        let user_dynamodb = &User {
            id: Option::from(Uuid::new_v4()),
            username: "nhut_donot_delete".to_string(),
            email: Option::from("donotdelete@gmail.com".to_string()),
            phone: Option::from("+84123456789".to_string()),
        };

        let result = insert_user_to_dynamodb(Option::from(user_dynamodb), table_name).await;

        println!("insert to dynamo db result {}", result);
    }

    #[tokio::test]
    async fn deactivate_user() {
        initialise();
        println!("is it working?");
        env::set_var(
            "AWS_ACCESS_KEY_ID",
            std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
        );
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        );

        let table_name = "dev-sg_UserTable".to_string();

        let uuid = Uuid::parse_str("6296fd76-07f6-40c0-9c71-db0412cd0562").unwrap();
        let user_dynamodb = &User {
            id: Option::from(uuid),
            username: "".to_string(),
            email: None,
            phone: None,
        };

        let result = deactivate_user_to_dynamodb(Option::from(user_dynamodb), table_name).await;

        println!("deactivate user result {}", result);
    }

    #[tokio::test]
    async fn update_password() {
        initialise();
        println!("is it working?");
        env::set_var(
            "AWS_ACCESS_KEY_ID",
            std::env::var("AWS_ACCESS_KEY_ID").unwrap(),
        );
        env::set_var(
            "AWS_SECRET_ACCESS_KEY",
            std::env::var("AWS_SECRET_ACCESS_KEY").unwrap(),
        );
        let user = &User {
            id: None,
            username: "nhut_donot_delete".to_string(),
            email: None,
            phone: None,
        };

        let result = update_user_password(user, "Hvcg@123456".to_string()).await;
        println!("Update password result: {}", result)
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
