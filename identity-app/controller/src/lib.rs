use db_postgres::test1_gateway::Test1SimpleRepository;
use db_postgres::user_gateway::UserRepository;
use domain::boundaries::{Test1DbGateway, Test1SimpleQueryInputBoundary, Test1SimpleQueryRequest, Test1SimpleQueryResponse, UserDbResponse, UserDbRequest, UserDbGateway, UserSimpleMutationInputBoundary};
use hvcg_iam_openapi_identity::models::User;

pub mod openapi;

pub async fn create_user(user: &User) -> User{
    let client = db_postgres::connect().await;
    let user_repository = UserRepository { client };
    let user_db_gateway = Box::new(user_repository);


    let result = domain::interactors::user_mutation::UserSimpleMutationInteractor::new(user_db_gateway)
        .create_user(user.to_model());
    return result.to_openapi();
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

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
