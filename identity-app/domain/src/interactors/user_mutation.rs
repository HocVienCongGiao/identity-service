use async_trait::async_trait;
use futures::executor::block_on;

use crate::boundaries;
use crate::boundaries::{
    UserDbGateway,
    UserDbRequest,
    UserDbResponse,
};

pub struct UserSimpleMutationInteractor<A: UserDbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> boundaries::UserSimpleMutationInputBoundary for UserSimpleMutationInteractor<A>
    where
        A: UserDbGateway + Sync + Send, {
    async fn create_user(&self, request: UserDbRequest) -> UserDbResponse {
        println!("user simple mutation input boundary {}", request.username);
            let user = crate::entity::user::User {
                id: request.id.clone(),
                username: request.username.clone(),
                email: request.email.unwrap(),
                phone: request.phone.unwrap(),
                enabled: true,
            };
        (*self).db_gateway.insert(&user).await;
        // if (*self).db_gateway.exists_by_username(request.username.clone()) {
        //     println!("user with this {} already exists", request.username);
        // } else {
        //     println!("new user, all is good");
        //     let user = crate::entity::user::User {
        //         id: request.id.clone(),
        //         username: request.username.clone(),
        //         email: request.email.unwrap(),
        //         phone: request.phone.unwrap(),
        //         enabled: true,
        //     };
        //
        //     (*self).db_gateway.insert(&user);

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
        // }

        return UserDbResponse {
            id: Default::default(),
            username: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            enabled: false,
        };
    }
}

impl<A> UserSimpleMutationInteractor<A>
    where
        A: UserDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        UserSimpleMutationInteractor { db_gateway }
    }
}
