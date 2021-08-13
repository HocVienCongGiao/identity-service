use async_trait::async_trait;
use regex::Regex;
use uuid::Uuid;

use crate::boundaries;
use crate::boundaries::{
    DbError, UserCollectionQueryResponse, UserDbGateway, UserDbResponse, UserMutationError,
    UserMutationRequest, UserMutationResponse, UserQueryResponse,
};

pub struct UserQueryInteractor<A: UserDbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> boundaries::UserQueryInputBoundary for UserQueryInteractor<A>
where
    A: UserDbGateway + Sync + Send,
{
    async fn get_user_by_id(&self, id: Uuid) -> Option<UserQueryResponse> {
        println!(
            "user query input boundary {}",
            id.to_hyphenated()
        );

        return if let Some(db_response) = ((*self).db_gateway.get_user_by_id(id)).await {
            println!("user found");
            Some(db_response.to_user_query_response())
        } else {
            println!("user not found");
            None
        };
    }

    async fn get_users(
        &self,
        username: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        enabled: Option<bool>,
        offset: Option<u16>,
        count: Option<u16>,
    ) -> UserCollectionQueryResponse {
        let result = ((*self)
            .db_gateway
            .get_users(username, phone, email, enabled, offset, count))
        .await;
        let collection = result
            .collection
            .into_iter()
            .map(|user_db_response| user_db_response.to_user_query_response())
            .collect();
        UserCollectionQueryResponse {
            collection,
            has_more: result.has_more,
        }
    }
}

impl<A> UserQueryInteractor<A>
where
    A: UserDbGateway + Sync + Send,
{
    pub fn new(db_gateway: A) -> Self {
        UserQueryInteractor { db_gateway }
    }
}

impl UserDbResponse {
    fn to_user_query_response(&self) -> UserQueryResponse {
        UserQueryResponse {
            id: self.id.clone(),
            username: self.username.to_string(),
            email: self.email.to_string(),
            phone: self.phone.to_string(),
            enabled: self.enabled.clone(),
        }
    }
}
