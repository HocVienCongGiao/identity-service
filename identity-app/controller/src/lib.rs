use db_postgres::test1_gateway::Test1SimpleRepository;
use domain::boundaries::{
    Test1DbGateway, Test1SimpleQueryInputBoundary, Test1SimpleQueryRequest,
    Test1SimpleQueryResponse,
};

pub mod openapi;

pub async fn get_test1() -> Test1SimpleQueryResponse {
    let client = db_postgres::connect().await;
    let client = db_postgres::migrate(client).await;

    let test1_repository = Test1SimpleRepository { client };

    domain::interactors::test1_query::Test1SimpleQueryInteractor::new(test1_repository)
        .get_test1(Test1SimpleQueryRequest {
            name: "Ngo Dinh Nhu".to_string(),
        })
        .await
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
