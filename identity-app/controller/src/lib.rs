use db_postgres::test1_gateway::Test1SimpleRepository;
use db_postgres::user_gateway::UserRepository;
use domain::boundaries::{Test1DbGateway, Test1SimpleQueryInputBoundary, Test1SimpleQueryRequest, Test1SimpleQueryResponse, UserDbResponse, UserDbRequest, UserDbGateway, UserSimpleMutationInputBoundary};

pub mod openapi;

pub async fn create_user(user_request: UserDbRequest) -> UserDbResponse{
    let client = db_postgres::connect().await;
    let user_repository = UserRepository { client };
    let user_db_gateway = Box::new(user_repository);
    let result = domain::interactors::user_mutation::UserSimpleMutationInteractor::new(user_db_gateway)
        .create_user(user_request);
    hvcg_iam_openapi_identity::Api::activate_user(&(), &());
    // toopen API
    return result
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
