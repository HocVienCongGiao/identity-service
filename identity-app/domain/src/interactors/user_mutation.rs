use crate::boundaries;
use crate::boundaries::{
    UserDbGateway,
    UserSimpleMutationRequest,
    UserSimpleMutationResponse,
    UserDbResponse,
    UserDbRequest,
};
use async_trait::async_trait;
use futures::executor::block_on;

pub struct UserSimpleMutationInteractor {
    user_db_gateway: Box<dyn UserDbGateway>,
}


impl boundaries::UserSimpleMutationInputBoundary for UserSimpleMutationInteractor {
    fn create_user(&self, request: UserDbRequest) -> UserDbResponse {
        println!("user simple mutation input boundary {}", request.username.unwrap());
        if block_on((*self).user_db_gateway.exists_by_name(request.username.clone())) {
            println!("user with this {} already exists", request.username.unwrap());
        } else {
            println!("new user, all is good");
            let user = crate::entity::user::User {
                id: request.id.unwrap(),
                username: request.username.unwrap(),
                email: request.email.unwrap(),
                phone: request.phone.unwrap(),
                enabled: true,
            };

            let user_result = (*self).user_db_gateway.insert(user);
            if user_result {
                return UserDbResponse {
                    id: request.id,
                    username: request.username,
                    email: request.email,
                    phone: request.phone,
                    enabled: Option::from(true)
                };
            }
        }
        return UserDbResponse {
            id: None,
            username: None,
            email: None,
            phone: None,
            enabled: None,
        };
    }
}

impl UserSimpleMutationInteractor {
    pub fn new(user_db_gateway: Box<dyn UserDbGateway>) -> Self {
        UserSimpleMutationInteractor { user_db_gateway }
    }
}
