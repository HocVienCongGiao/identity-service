use hvcg_iam_openapi_identity::models::User;

use db_postgres::user_gateway::UserRepository;
use domain::boundaries::{UserDbGateway, UserDbRequest, UserDbResponse, UserSimpleMutationInputBoundary};

pub mod openapi;

pub async fn create_user(user: &User) -> User{
    let client = db_postgres::connect().await;
    let user_repository = UserRepository { client };
    let user_db_gateway = Box::new(user_repository);
    println!("nhuthuynh");
    // TODO debug at here
    // let result = domain::interactors::user_mutation::UserSimpleMutationInteractor::new(user_db_gateway)
    //     .create_user(user.to_model());
    // return result.to_openapi();
    return User {
        id: None,
        username: "".to_string(),
        email: None,
        phone: None
    }
}

impl ToOpenApi<User> for UserDbResponse {
    fn to_openapi(&self) -> hvcg_iam_openapi_identity::models::User {
        User {
            id: Option::from(self.id),
            username: self.username.to_string(),
            email: Option::from(self.email.to_string()),
            phone: Option::from(self.phone.to_string())
        }
    }
}

impl ToModel<UserDbRequest> for &User {
    fn to_model(&self) -> UserDbRequest {
        UserDbRequest {
            id: self.id.unwrap(),
            username: self.username.to_string(),
            email: self.email.clone(),
            phone: self.phone.clone()
        }
    }
}


pub trait ToOpenApi<T> {
    fn to_openapi(&self) -> T;
}

pub trait ToModel<T> {
    fn to_model(&self) -> T;
}

#[cfg(test)]
mod tests {
    use crate::create_user;
    use db_postgres::user_gateway::UserRepository;
    use domain::boundaries::{UserDbGateway, UserDbRequest, UserDbResponse, UserSimpleMutationInputBoundary};
    use hvcg_iam_openapi_identity::models::User;
    use uuid::Uuid;

    #[tokio::test]
    async fn user_controller_test() {
        create_user(&User {
            id: Option::from(Uuid::new_v4()),
            username: "test".to_string(),
            email: Option::from("test".to_string()),
            phone: Option::from("test".to_string())
        }).await;
    }
}
