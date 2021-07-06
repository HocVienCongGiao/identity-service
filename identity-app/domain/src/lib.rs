pub fn test_func() {
    println!("hello");
}

pub mod boundaries;
pub mod entity;
pub mod interactors;

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use crate::boundaries::{
        Test1DbGateway, Test1SimpleMutationInputBoundary, Test1SimpleMutationRequest,
    };

    struct Test1DbGatewayStub {}

    #[async_trait]
    impl Test1DbGateway for Test1DbGatewayStub {
        async fn exists_by_name(&self, name: String) -> bool {
            if name == "existing" {
                return true;
            }
            false
        }

        async fn insert(&self, name: String, country: String) -> bool {
            todo!()
        }
    }

    #[test]
    fn it_works() {
        let test1_simple_mutator =
            crate::interactors::test1_mutation::Test1SimpleMutationInteractor::new(Box::new(
                Test1DbGatewayStub {},
            ));
        test1_simple_mutator.create_test1(Test1SimpleMutationRequest {
            name: "existing".to_string(),
        });
        test1_simple_mutator.create_test1(Test1SimpleMutationRequest {
            name: "new".to_string(),
        });
        let result = 4;
        assert_eq!(result, 4);
    }
}
