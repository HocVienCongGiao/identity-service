#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Add;
    use std::path::PathBuf;

    use hvcg_iam_openapi_identity::models::User;
    use lambda_http::http::Method;
    use lambda_http::{Body, Context, IntoResponse, Request, RequestExt};
    use rand::Rng;
    use serde_json::json;
    use uuid::Uuid;

    use crate::create_user;

    #[tokio::test]
    async fn create_user_success() {
        // Given
        let test_suffix = Uuid::new_v4().to_string();

        let user_request = User {
            id: None,
            username: "test_user".to_string() + &*test_suffix,
            email: Option::from("nhut_cargo@gmail.com".to_string() + &*test_suffix),
            phone: Option::from("+84 909686868".to_string() + &*test_suffix),
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        let request_default = Request::default();
        let (mut parts, _) = request_default.into_parts();
        parts.method = Method::POST;

        parts.headers.append("X-TFE-Notification-Signature",
                             "c7cf4bbba3ff2117c2b235e8c3d77d5023311736072c7af4b72b418361bc05249bc86addc4633382ac8191cfa995a272e578a08c49b508bf2c7bccbf5670ba04".parse().unwrap());
        parts
            .headers
            .append("Content-Type", "application/json".parse().unwrap());

        let request = Request::from_parts(parts, Body::from(serialized_user));

        // When
        let response = create_user(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        assert_eq!(response.status(), 200);

        let deserialized_user: User = serde_json::from_slice(response.body()).unwrap();

        assert_eq!(!deserialized_user.id.is_none(), true);
        assert_eq!(
            deserialized_user.username,
            "test_user".to_string() + &*test_suffix
        );
        assert_eq!(
            deserialized_user.email,
            Option::from("nhut_cargo@gmail.com".to_string() + &*test_suffix)
        );
        assert_eq!(
            deserialized_user.phone,
            Option::from("+84 909686868".to_string() + &*test_suffix)
        );
    }

    #[tokio::test]
    #[should_panic]
    async fn create_user_failed() {
        // Given
        let test_suffix = Uuid::new_v4().to_string();
        let user_request = User {
            id: None,
            username: "test".to_string().add(&*test_suffix),
            email: None,
            phone: None,
        };

        let serialized_user = serde_json::to_string(&user_request).unwrap();

        let request_default = Request::default();
        let (mut parts, _) = request_default.into_parts();
        parts.method = Method::POST;

        parts.headers.append("X-TFE-Notification-Signature",
                             "c7cf4bbba3ff2117c2b235e8c3d77d5023311736072c7af4b72b418361bc05249bc86addc4633382ac8191cfa995a272e578a08c49b508bf2c7bccbf5670ba04".parse().unwrap());
        parts
            .headers
            .append("Content-Type", "application/json".parse().unwrap());

        let request = Request::from_parts(parts, Body::from(serialized_user));

        let expected_response = json!("Failed to insert user").into_response();

        // When
        let response = create_user(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();

        // Then
        // Checking error log
    }
}
