use hvcg_iam_openapi_identity::models::User;

use crate::openapi::identity_user::{ToModel, ToOpenApi};
use db_postgres::user_gateway::UserRepository;
use domain::boundaries::{
    UserDbResponse, UserMutationError, UserMutationRequest, UserSimpleMutationInputBoundary,
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
