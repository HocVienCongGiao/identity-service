use async_trait::async_trait;
use regex::Regex;
use uuid::Uuid;

use crate::boundaries;
use crate::boundaries::{
    DbError, UserDbGateway, UserDbResponse, UserMutationError, UserMutationRequest,
    UserMutationResponse,
};

pub struct UserSimpleMutationInteractor<A: UserDbGateway> {
    db_gateway: A,
}

#[async_trait]
impl<A> boundaries::UserSimpleMutationInputBoundary for UserSimpleMutationInteractor<A>
where
    A: UserDbGateway + Sync + Send,
{
    async fn create_user(
        &self,
        request: UserMutationRequest,
    ) -> Result<UserMutationResponse, UserMutationError> {
        println!("user mutation input boundary {}", request.username);
        let is_not_valid_username = request.username.is_empty();
        if is_not_valid_username {
            println!("Cannot process with empty username");
            return Err(UserMutationError::InvalidUser);
        }

        if request.is_not_valid_email_format() {
            println!("Email is not in valid format");
            return Err(UserMutationError::InvalidEmail);
        }

        if request.is_not_valid_phone_format() {
            println!("Phone is not in valid format");
            return Err(UserMutationError::InvalidPhone);
        }

        if (*self)
            .db_gateway
            .exists_by_username(request.username.clone())
            .await
        {
            println!("user with this {} already exists", request.username);
            return Err(UserMutationError::ExistedUser);
        }

        println!("new user, all is good");
        let user = crate::entity::user::User {
            id: Uuid::new_v4(),
            username: request.username.clone(),
            email: request.email,
            phone: request.phone,
            enabled: true,
        };

        if !is_not_valid_username {
            println!("This user is valid");
            (*self)
                .db_gateway
                .insert(&user)
                .await
                .map(|_| user.to_user_mutation_response())
                .map_err(|err| err.to_user_mutation_error())
        } else {
            Err(UserMutationError::UnknownError)
        }
    }

    async fn deactivate_user(&self, id: Uuid) -> Result<UserMutationResponse, UserMutationError> {
        let result = (*self)
            .db_gateway
            .deactivate_user(id)
            .await
            .map(|user| user.to_user_mutation_response())
            .map_err(|err| err.to_user_mutation_error());

        if result.is_err() {
            Err(UserMutationError::UnknownError)
        } else {
            result
        }
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<UserMutationResponse, UserMutationError> {
        (*self)
            .db_gateway
            .get_user_by_id(id)
            .await
            .map(|user| user.to_user_mutation_response())
            .map_err(|err| err.to_user_mutation_error())
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

impl DbError {
    fn to_user_mutation_error(&self) -> UserMutationError {
        match self {
            DbError::UniqueConstraintViolationError(field) => {
                UserMutationError::UniqueConstraintViolationError(field.to_string())
            }
            DbError::UnknownError => UserMutationError::UnknownError,
        }
    }
}

impl crate::entity::user::User {
    fn to_user_db_request(&self) -> UserMutationRequest {
        UserMutationRequest {
            username: self.username.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
        }
    }

    fn to_user_mutation_response(&self) -> UserMutationResponse {
        UserMutationResponse {
            id: self.id,
            username: self.username.clone(),
            email: self.email.clone().unwrap(),
            phone: self.phone.clone().unwrap(),
            enabled: self.enabled,
        }
    }
}

impl crate::interactors::user_mutation::UserMutationRequest {
    fn is_not_valid_email_format(&self) -> bool {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();

        if self.email.is_none() {
            println!("Email is none");
            return true;
        }

        !email_regex.is_match(&*self.email.clone().unwrap())
    }

    fn is_not_valid_phone_format(&self) -> bool {
        let phone_regex = Regex::new(r"^(\+84 [0-9]{9}$)").unwrap();
        if self.phone.is_none() {
            println!("Phone is none");
            return true;
        }
        !phone_regex.is_match(&*self.phone.clone().unwrap())
    }
}
