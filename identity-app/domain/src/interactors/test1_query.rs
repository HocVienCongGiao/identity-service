use crate::boundaries;
use crate::boundaries::{Test1DbGateway, Test1SimpleQueryRequest, Test1SimpleQueryResponse};
use async_trait::async_trait;

pub struct Test1SimpleQueryInteractor<A: Test1DbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> boundaries::Test1SimpleQueryInputBoundary for Test1SimpleQueryInteractor<A>
where
    A: Test1DbGateway + Sync + Send,
{
    async fn get_test1(&self, request: Test1SimpleQueryRequest) -> Test1SimpleQueryResponse {
        println!("test1 simple mutation input boundary {}", request.name);
        let status: u16;
        if ((*self).db_gateway.exists_by_name(request.name.clone())).await {
            println!("user found");
            status = 200;
        } else {
            println!("user not found");
            status = 404;
        }
        Test1SimpleQueryResponse { status }
    }
}

impl<A> Test1SimpleQueryInteractor<A>
where
    A: Test1DbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        Test1SimpleQueryInteractor { db_gateway }
    }
}
