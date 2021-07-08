use async_trait::async_trait;
use futures::executor::block_on;

use crate::boundaries;
use crate::boundaries::{
    UserDbGateway,
    UserDbRequest,
    UserDbResponse,
};
use uuid::Uuid;

pub struct UserSimpleMutationInteractor<A: UserDbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> boundaries::UserSimpleMutationInputBoundary for UserSimpleMutationInteractor<A>
    where
        A: UserDbGateway + Sync + Send, {
    async fn create_user(&self, request: UserDbRequest) -> UserDbResponse {
        let empty_user_response =  UserDbResponse {
            id: Default::default(),
            username: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            enabled: false,
        };

        println!("user simple mutation input boundary {}", request.username);

        if (*self).db_gateway.exists_by_username(request.username.clone()).await {
            println!("user with this {} already exists", request.username);
            return empty_user_response
        }

        println!("new user, all is good");
        let user = crate::entity::user::User {
            id: Uuid::new_v4(),
            username: request.username.clone(),
            email: request.email,
            phone: request.phone,
            enabled: true,
        };

        let user_result_wait = (*self).db_gateway.insert(&user).await;
        println!("user_result_wait {}", user_result_wait);

        return if user_result_wait {
            UserDbResponse {
                id: user.id.clone(),
                username: user.username,
                email: user.email.unwrap(),
                phone: user.phone.unwrap(),
                enabled: false
            }
        } else {
            empty_user_response
        }
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
