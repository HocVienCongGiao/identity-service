use hvcg_iam_openapi_identity::models::{User, UserCollection};

use crate::openapi::identity_user::{ToModel, ToOpenApi};
use db_postgres::user_gateway::UserRepository;
use domain::boundaries::{
    UserDbResponse, UserMutationError, UserMutationRequest, UserQueryInputBoundary,
    UserSimpleMutationInputBoundary,
};
use uuid::Uuid;

pub mod openapi;

pub async fn create_user(user: &User) -> Result<openapi::identity_user::User, UserMutationError> {
    let client = db_postgres::connect().await;
    let user_repository = UserRepository { client };
    let user_request = user.to_model();

    let user_interactor =
        domain::interactors::user_mutation::UserSimpleMutationInteractor::new(user_repository);

    let response = user_interactor.create_user(user_request).await;

    response.map(|res| res.to_openapi())
}

pub async fn deactivate_user(id: Uuid) -> Result<openapi::identity_user::User, UserMutationError> {
    let client = db_postgres::connect().await;
    let user_repository = UserRepository { client };

    let user_interactor =
        domain::interactors::user_mutation::UserSimpleMutationInteractor::new(user_repository);
    let response = user_interactor.deactivate_user(id).await;
    response.map(|res| res.to_openapi())
}

pub async fn activate_user(id: Uuid) -> Result<openapi::identity_user::User, UserMutationError> {
    let client = db_postgres::connect().await;
    let user_repository = UserRepository { client };

    let user_interactor =
        domain::interactors::user_mutation::UserSimpleMutationInteractor::new(user_repository);
    let response = user_interactor.activate_user(id).await;
    response.map(|res| res.to_openapi())
}

pub async fn get_user_by_id(id: Uuid) -> Option<openapi::identity_user::User> {
    let client = db_postgres::connect().await;
    let user_repository = UserRepository { client };

    let user_interactor =
        domain::interactors::user_query::UserQueryInteractor::new(user_repository);
    let response = user_interactor.get_user_by_id(id).await;
    if response.is_none() {
        return None;
    }
    Some(response.unwrap().to_openapi())
}

pub async fn get_users(
    username: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    enabled: Option<bool>,
    offset: Option<u16>,
    count: Option<u16>,
) -> UserCollection {
    let client = db_postgres::connect().await;
    let user_repository = UserRepository { client };

    let user_interactor =
        domain::interactors::user_query::UserQueryInteractor::new(user_repository);
    let response = user_interactor
        .get_users(username, phone, email, enabled, offset, count)
        .await;
    response.to_openapi()
}

#[cfg(test)]
mod tests {
    use hvcg_iam_openapi_identity::models::User;
    use uuid::Uuid;

    use db_postgres::user_gateway::UserRepository;
    use domain::boundaries::{
        UserDbGateway, UserDbResponse, UserMutationRequest, UserSimpleMutationInputBoundary,
    };

    use crate::create_user;

    #[tokio::test]
    async fn user_controller_test() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
