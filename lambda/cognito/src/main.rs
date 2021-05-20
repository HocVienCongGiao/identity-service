extern crate rusoto_core;
extern crate rusoto_dynamodb;

use std::default::Default;

use rusoto_cognito_idp::{
    CognitoIdentityProvider, CognitoIdentityProviderClient, ListUsersRequest,
};
use rusoto_core::{Client, Region};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};
use rusoto_s3::{S3Client, S3};
// use rusoto_credential::{ProfileProvider};
// https://docs.rs/rusoto_cognito_idp/0.46.0/rusoto_cognito_idp/
use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(cognito)).await?;
    Ok(())
}

async fn cognito(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    // let dispatcher = HttpClient::new().expect("failed to create request dispatcher");
    // let default_provider_result = ProfileProvider::new();
    // let mut default_provider = default_provider_result.unwrap();
    // default_provider.set_profile("hvcg");
    // let aws_client = Client::new_with(default_provider, dispatcher);
    let aws_client = Client::shared();

    let rusoto_cognito_idp_client =
        CognitoIdentityProviderClient::new_with_client(aws_client.clone(), Region::ApSoutheast1);
    // let create_user_request = AdminCreateUserRequest {
    //     desired_delivery_mediums: None,
    //     force_alias_creation: None,
    //     message_action: None,
    //     temporary_password: None,
    //     user_attributes: None,
    //     user_pool_id: "".to_string(),
    //     username: "".to_string(),
    //     validation_data: None
    // };
    let list_user_request = ListUsersRequest {
        attributes_to_get: None,
        filter: None,
        limit: None,
        pagination_token: None,
        user_pool_id: "ap-southeast-1_vmFHg7JIC".to_string(),
    };
    match rusoto_cognito_idp_client
        .list_users(list_user_request)
        .sync()
    {
        Ok(response) => match response.users {
            Some(user_types) => {
                println!("User Type here");
                for user_type in user_types {
                    println!("{}", user_type.username.unwrap())
                }
            }
            None => println!("No buckets in region!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    let s3client = S3Client::new_with_client(aws_client, Region::ApSoutheast1);
    match s3client.list_buckets().sync() {
        Ok(output) => match output.buckets {
            Some(bucket_list) => {
                println!("Bucket to be printed here");
                for bucket in bucket_list {
                    println!("{}", bucket.name.unwrap());
                }
            }
            None => println!("No buckets in region!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    let client = DynamoDbClient::new(Region::ApSoutheast1);
    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(list_tables_input).sync() {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                println!("Tables in database:");

                for table_name in table_name_list {
                    println!("{}", table_name);
                }
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }

    // creating an application/json response
    Ok(json!({
    "message": "Hey cognito!"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cognito_handles() {
        let request = Request::default();
        let expected = json!({
        "message": "Hey cognito!"
        })
        .into_response();
        let response = cognito(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}
