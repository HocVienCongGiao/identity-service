use domain::boundaries::{UserMutationRequest, UserCollectionQueryResponse, UserQueryResponse};
pub use domain::boundaries::UserMutationResponse;
pub use hvcg_iam_openapi_identity::models::User;
use hvcg_iam_openapi_identity::models::UserCollection;

pub fn create_saint() {
    println!("Creating User in Controller OpenApi saint.rs")
}

impl ToOpenApi<User> for UserMutationResponse {
    fn to_openapi(self) -> User {
        User {
            id: Option::from(self.id),
            username: self.username.to_string(),
            email: Option::from(self.email.to_string()),
            phone: Option::from(self.phone.to_string()),
        }
    }
}

impl ToModel<UserMutationRequest> for &User {
    fn to_model(&self) -> UserMutationRequest {
        UserMutationRequest {
            username: self.username.to_string(),
            email: self.email.clone(),
            phone: self.phone.clone(),
        }
    }
}

pub trait ToOpenApi<T> {
    fn to_openapi(self) -> T;
}

pub trait ToModel<T> {
    fn to_model(&self) -> T;
}

impl ToOpenApi<UserCollection> for UserCollectionQueryResponse {
    fn to_openapi(self) -> UserCollection {
        let collection = (self
            .collection
            .into_iter()
            .map(|user_query_response| user_query_response.to_openapi())
            .collect::<Vec<User>>())
            .to_vec();
        UserCollection {
            users: Some(collection),
            has_more: self.has_more,
        }
    }
}

impl ToOpenApi<User> for UserQueryResponse {
    fn to_openapi(self) -> User {
        User {
            id: Option::from(self.id),
            username: self.username,
            email: Option::from(self.email),
            phone: Option::from(self.phone)
        }
    }
}