use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Deserialize, Serialize)]
struct TokenPayload {
    // Despite the struct field being named `username`, it is going to come
    // from a JSON field called `cognito:username`.
    #[serde(rename(deserialize = "cognito:username"))]
    username: String,
    #[serde(rename(deserialize = "cognito:groups"))]
    groups: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::ops::Add;
    use std::path::PathBuf;
    use std::sync::Once;

    use hvcg_iam_openapi_identity::models::{Group, User};
    use jsonwebtoken::TokenData;
    use lambda_http::http::HeaderValue;
    use lambda_http::{http, Context, RequestExt};
    use lambda_http::{Body, IntoResponse};
    use regex::Regex;
    use std::env;
    use uuid::Uuid;

    use crate::TokenPayload;
    use db_postgres::connect;
    use rusoto_core::credential::EnvironmentProvider;
    use rusoto_core::{HttpClient, Region};
    use rusoto_dynamodb::{
        AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, ListTablesInput,
        PutItemInput,
    };
    use std::collections::hash_map::DefaultHasher;
    use std::collections::HashMap;
    use std::hash::{Hash, Hasher};
    use tokio_postgres::types::ToSql;
    use user::{func, UserUpdate};

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

    async fn truncate_data() {
        let connect = connect().await;

        let stmt = (connect)
            .prepare(
                "truncate identity__user_username,\
             identity__user_phone, \
             identity__user_email, \
             identity__user_enabled, \
             identity__user, identity__user_group",
            )
            .await
            .unwrap();

        connect.query_one(&stmt, &[]).await;
    }

    #[tokio::test]
    async fn cleanup_data() {
        truncate_data().await;
    }
    #[tokio::test]
    async fn crud_user_success() {
        truncate_data().await;
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

        let client = DynamoDbClient::new_with(
            HttpClient::new().unwrap(),
            EnvironmentProvider::default(),
            Region::ApSoutheast1,
        );

        // Given
        let test_suffix = Uuid::new_v4().to_string();

        let user_request = User {
            id: None,
            username: "create_test_user".to_string(),
            email: Option::from("create_test_user@gmail.com".to_string()),
            phone: Option::from("+84 939686970".to_string()),
            enabled: None,
            group: Option::from(vec![
                hvcg_iam_openapi_identity::models::Group::ADMIN_GROUP,
                hvcg_iam_openapi_identity::models::Group::STUDENT_GROUP,
            ]),
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        let request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/user")
            .method("POST")
            .header("Content-Type", "application/json")
            .header("authorization", "Bearer 123445")
            .body(Body::from(serialized_user))
            .unwrap();
        println!("request : {}", request.uri().to_string());

        let mut context: Context = Context::default();
        context.invoked_function_arn = "dev-sg_identity-service_users".to_string();

        let response = user::func(request, context)
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        assert_eq!(response.status(), 200);

        let deserialized_user: User = serde_json::from_slice(response.body()).unwrap();

        assert!(!deserialized_user.id.is_none(), true);
        assert_eq!(deserialized_user.username, "create_test_user".to_string());
        assert_eq!(
            deserialized_user.email,
            Option::from("create_test_user@gmail.com".to_string())
        );
        assert_eq!(
            deserialized_user.phone,
            Option::from("+84 939686970".to_string())
        );
        assert_eq!(deserialized_user.enabled, Option::from(true));
        assert_eq!(
            deserialized_user.group,
            Option::from(vec![
                hvcg_iam_openapi_identity::models::Group::ADMIN_GROUP,
                hvcg_iam_openapi_identity::models::Group::STUDENT_GROUP
            ])
        );
        println!("Create user successfully!");
        // Deactivate user
        let deactivate_request = User {
            id: Option::from(deserialized_user.id.unwrap()),
            username: "".to_string(),
            email: None,
            phone: None,
            enabled: None,
            group: None,
        };
        let mut serialized_request = serde_json::to_string(&deactivate_request).unwrap();

        let deactivate_request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/users/deactivation")
            .method("POST")
            .header("Content-Type", "application/json")
            .header("authorization", "Bearer 123445")
            .body(Body::from(serialized_request.clone()))
            .unwrap();

        let mut deactivate_context: Context = Context::default();
        deactivate_context.invoked_function_arn = "dev-sg_identity-service_users".to_string();

        let deactivate_user_response = func(deactivate_request, deactivate_context)
            .await
            .expect("expected Ok(_) value")
            .into_response();
        let deserialized__deactivate_user: User =
            serde_json::from_slice(deactivate_user_response.body()).unwrap();
        // Then
        println!(
            "deactivate deserialized_user: {:?}",
            deserialized__deactivate_user
        );
        assert_eq!(deactivate_user_response.status(), 200);

        // Activate user
        let activate_request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/users/activation")
            .method("POST")
            .header("Content-Type", "application/json")
            .header("authorization", "Bearer 123445")
            .body(Body::from(serialized_request.clone()))
            .unwrap();
        let mut activate_context: Context = Context::default();
        activate_context.invoked_function_arn = "dev-sg_identity-service_users".to_string();
        let activate_user_response = func(activate_request, activate_context)
            .await
            .expect("expected Ok(_) value")
            .into_response();
        let deserialized_activate_user: User =
            serde_json::from_slice(activate_user_response.body()).unwrap();
        println!(
            "deserialized_activate_user: {:?}",
            deserialized_activate_user
        );
        assert_eq!(activate_user_response.status(), 200);

        // Update user
        let update_group_request = vec![
            hvcg_iam_openapi_identity::models::Group::PROFESSOR_GROUP,
            hvcg_iam_openapi_identity::models::Group::OPERATOR_GROUP,
        ];
        let update_user_request = User {
            id: None,
            username: "test_update_user_updated".to_string(),
            email: Option::from("test_update_user_updated@gmail.com".to_string()),
            phone: Option::from("+84 939969699".to_string()),
            enabled: None,
            group: Option::from(update_group_request.clone()),
        };
        let serialized_user = serde_json::to_string(&update_user_request).unwrap();
        let uri = format!(
            "https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/users/{}",
            deserialized_user.id.unwrap().to_hyphenated()
        );
        let mut path_param = HashMap::new();
        path_param.insert(
            "id".to_string(),
            vec![deserialized_user.id.unwrap().to_hyphenated().to_string()],
        );
        let request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/users")
            .method("PUT")
            .header("Content-Type", "application/json")
            .header("authorization", "Bearer 123445")
            .body(Body::from(serialized_user))
            .unwrap()
            .with_path_parameters(path_param);
        println!("request : {}", request.uri().to_string());
        let mut context: Context = Context::default();
        context.invoked_function_arn = "dev-sg_identity-service_users".to_string();

        let response = user::func(request, context)
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        assert_eq!(response.status(), 200);

        let deserialized_user: User = serde_json::from_slice(response.body()).unwrap();
        println!("updated deserialized_user: {:?}", deserialized_user);
        assert!(!deserialized_user.id.is_none(), true);
        assert_eq!(
            deserialized_user.username,
            "test_update_user_updated".to_string()
        );
        assert_eq!(
            deserialized_user.email,
            Option::from("test_update_user_updated@gmail.com".to_string())
        );
        assert_eq!(
            deserialized_user.phone,
            Option::from("+84 939969699".to_string())
        );
        assert_eq!(
            deserialized_user.group.unwrap(),
            update_group_request.clone()
        );
        println!("Update user successfully!");
    }

    #[tokio::test]
    async fn update_password_user_success() {
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

        // Given
        let user_test = User {
            id: None,
            username: "nhut_donot_delete".to_string(),
            email: Option::from("nhut_donot_delete@gmail.com".to_string()),
            phone: Option::from("+84 123456002".to_string()),
            enabled: None,
            group: None,
        };

        let user = controller::create_user(&user_test).await;
        let user_request = UserUpdate {
            id: user.unwrap().id,
            plain_password: "Test@12345678".to_string(),
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        let request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/users/update-password")
            .method("PUT")
            .header("Content-Type", "application/json")
            .header("authorization", "Bearer 123445")
            .body(Body::from(serialized_user))
            .unwrap();
        println!("request: {:?}", request);

        let mut context: Context = Context::default();
        context.invoked_function_arn = "dev-sg_identity-service_users".to_string();

        let response = user::func(request, context)
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn get_user_by_id_success() {
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
        truncate_data().await;
        // Given
        let user_test = User {
            id: None,
            username: "test003".to_string(),
            email: Option::from("test003@gmail.com".to_string()),
            phone: Option::from("+84 123456003".to_string()),
            enabled: None,
            group: Option::from(vec![Group::ADMIN_GROUP, Group::PROFESSOR_GROUP]),
        };

        let user = controller::create_user(&user_test).await;
        let user_id = user.unwrap().id.unwrap();
        let uri = format!(
            "https://dev-sg.portal.hocvienconggiao.com/mutation-api/identity-service/users/{}",
            user_id
        );
        let mut path_param = HashMap::new();
        path_param.insert("id".to_string(), vec![user_id.to_string()]);
        let request = http::Request::builder()
            .uri(uri)
            .method("GET")
            .header("Content-Type", "application/json")
            .header("authorization", "Bearer 123445")
            .body(Body::default())
            .unwrap()
            .with_path_parameters(path_param);
        println!("request: {:?}", request);

        let mut context: Context = Context::default();
        context.invoked_function_arn = "dev-sg_identity-service_users".to_string();

        let response = user::func(request, context)
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        println!("response: {:?}", response);
        assert_eq!(response.status(), 200);
        // truncate_data().await;
    }

    #[tokio::test]
    async fn get_users() {
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
        truncate_data().await;
        // Given
        let first_user_test = User {
            id: None,
            username: "test004".to_string(),
            email: Option::from("test004@gmail.com".to_string()),
            phone: Option::from("+84 123456004".to_string()),
            enabled: None,
            group: Option::from(vec![Group::ADMIN_GROUP, Group::PROFESSOR_GROUP]),
        };

        let second_user_test = User {
            id: None,
            username: "test005".to_string(),
            email: Option::from("test005@gmail.com".to_string()),
            phone: Option::from("+84 123456005".to_string()),
            enabled: None,
            group: Option::from(vec![Group::ADMIN_GROUP, Group::PROFESSOR_GROUP]),
        };

        let first_user_test = controller::create_user(&first_user_test).await;
        println!("first_user_test : {:?}", first_user_test);
        let second_user_test = controller::create_user(&second_user_test).await;
        println!("second_user_test : {:?}", second_user_test);
        let mut query_param = HashMap::new();
        query_param.insert("count".to_string(), vec!["5".to_string()]);
        query_param.insert("offset".to_string(), vec!["1".to_string()]);
        query_param.insert("username".to_string(), vec!["test".to_string()]);
        let request = http::Request::builder()
            .uri("https://dev-sg.portal.hocvienconggiao.com/query-api/identity-service/users?offset=1&count=5")
            .method("GET")
            .header("Content-Type", "application/json")
            .body(Body::Empty)
            .unwrap()
            .with_query_string_parameters(query_param);

        // When
        let response = user::func(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        // Then
        println!("response: {:?}", response);
        assert_eq!(response.status(), 200);
        println!("Trigger build");
    }

    fn hash<T>(obj: T) -> u64
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        obj.hash(&mut hasher);
        hasher.finish()
    }
}
