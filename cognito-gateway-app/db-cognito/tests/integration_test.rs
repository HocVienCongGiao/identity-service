#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;

    use db_cognito::{
        activate_user_to_dynamodb, deactivate_user_to_dynamodb, insert_user_to_dynamodb,
        update_user_password, update_user_to_dynamodb,
    };
    use hvcg_iam_openapi_identity::models::{User, Group};
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
    async fn activate_user() {
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

        let uuid = Uuid::parse_str("22ea2825-621f-4260-bc13-f2c8432066ae").unwrap();
        let user_dynamodb = &User {
            id: Option::from(uuid),
            username: "".to_string(),
            email: None,
            phone: None,
            enabled: None,
            group: None
        };

        let result = activate_user_to_dynamodb(Option::from(user_dynamodb), table_name).await;

        println!("activate user result {}", result);
    }

    #[tokio::test]
    async fn create_users() {
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
            username: "test_user_group".to_string(),
            email: Option::from("test_user_group1@gmail.com".to_string()),
            phone: Option::from("+84123456789".to_string()),
            enabled: Option::from(true),
            group: Option::from(vec![
                Group::ADMIN_GROUP,
                Group::STUDENT_GROUP,
            ])
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
            enabled: None,
            group: None
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
            enabled: None,
            group: None
        };

        let result = update_user_password(user, "Hvcg@123456".to_string()).await;
        println!("Update password result: {}", result)
    }

    #[tokio::test]
    async fn update_user() {
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

        let uuid = Uuid::parse_str("45cd758d-a99b-4d26-b851-c75779674ebb").unwrap();
        let user_dynamodb = &User {
            id: Option::from(uuid),
            username: "test_user_group1_updated".to_string(),
            email: Option::from("test_user_group1_updated_updated@gmail.com".to_string()),
            phone: Option::from("+84 987654321".to_string()),
            enabled: None,
            group: Option::from(vec![
                Group::OPERATOR_GROUP,
                Group::PROFESSOR_GROUP
            ])
        };

        let result = update_user_to_dynamodb(Option::from(user_dynamodb), table_name).await;

        println!("update_user_to_dynamodb {}", result);
    }
}
