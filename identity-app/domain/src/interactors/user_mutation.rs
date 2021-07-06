use async_trait::async_trait;
use futures::executor::block_on;

use crate::boundaries;
use crate::boundaries::{
    UserDbGateway,
    UserDbRequest,
    UserDbResponse,
};

pub struct UserSimpleMutationInteractor {
    user_db_gateway: Box<dyn UserDbGateway>,
}


impl boundaries::UserSimpleMutationInputBoundary for UserSimpleMutationInteractor {
    fn create_user(&self, request: UserDbRequest) -> UserDbResponse {
        println!("user simple mutation input boundary {}", request.username);
        if block_on((*self).user_db_gateway.exists_by_username(request.username.clone())) {
            println!("user with this {} already exists", request.username);
        } else {
            println!("new user, all is good");
            let user = crate::entity::user::User {
                id: request.id.clone(),
                username: request.username.clone(),
                email: request.email.unwrap(),
                phone: request.phone.unwrap(),
                enabled: true,
            };

            (*self).user_db_gateway.insert(&user);

            // let user_result_wait = futures::executor::block_on((*self).user_db_gateway.insert(&user));

            // if user_result_wait {
            //     return UserDbResponse {
            //
            //         // id: user.id.clone(),
            //         // username: user.username.clone(),
            //         // email: user.email.clone(),
            //         // phone: user.phone.clone(),
            //         // enabled: true
            //         id: Default::default(),
            //         username: "".to_string(),
            //         email: "".to_string(),
            //         phone: "".to_string(),
            //         enabled: false
            //     };
            // }
        }

        return UserDbResponse {
            id: Default::default(),
            username: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            enabled: false
        };
    }
}

impl UserSimpleMutationInteractor {
    pub fn new(user_db_gateway: Box<dyn UserDbGateway>) -> Self {
        UserSimpleMutationInteractor { user_db_gateway }
    }
}
