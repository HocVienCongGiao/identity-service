use crate::boundaries;
use crate::boundaries::{Test1DbGateway, Test1SimpleMutationRequest, Test1SimpleMutationResponse};
use async_trait::async_trait;
use futures::executor::block_on;

pub struct Test1SimpleMutationInteractor {
    db_gateway: Box<dyn Test1DbGateway>,
}

impl boundaries::Test1SimpleMutationInputBoundary for Test1SimpleMutationInteractor {
    fn create_test1(&self, request: Test1SimpleMutationRequest) -> Test1SimpleMutationResponse {
        println!("test1 simple mutation input boundary {}", request.name);
        if block_on((*self).db_gateway.exists_by_name(request.name.clone())) {
            println!("user with this name already exists");
        } else {
            println!("new user, all is good");
            let test1 = crate::entity::test1::Test1 {
                id: 0,
                name: request.name,
            };
            if test1.is_valid() {
                println!("This user is valid");
            }
        }
        Test1SimpleMutationResponse {}
    }
}

impl Test1SimpleMutationInteractor {
    pub fn new(db_gateway: Box<dyn Test1DbGateway>) -> Self {
        Test1SimpleMutationInteractor { db_gateway }
    }
}
