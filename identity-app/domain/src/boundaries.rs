use async_trait::async_trait;
use uuid::Uuid;

use crate::entity::user::User;

#[async_trait]
pub trait Test1SimpleQueryInputBoundary {
    async fn get_test1(&self, request: Test1SimpleQueryRequest) -> Test1SimpleQueryResponse;
}

pub trait Test1SimpleMutationInputBoundary {
    fn create_test1(&self, request: Test1SimpleMutationRequest) -> Test1SimpleMutationResponse;
}

#[async_trait]
pub trait UserSimpleMutationInputBoundary {
    async fn activate_user(&self, id: Uuid) -> Result<UserMutationResponse, UserMutationError>;
    async fn create_user(
        &self,
        request: UserMutationRequest,
    ) -> Result<UserMutationResponse, UserMutationError>;
    async fn deactivate_user(&self, id: Uuid) -> Result<UserMutationResponse, UserMutationError>;
    async fn update_user(
        &self,
        id: Uuid,
        request: UserMutationRequest,
    ) -> Result<UserMutationResponse, UserMutationError>;
}

#[async_trait]
pub trait UserQueryInputBoundary {
    async fn get_user_by_id(&self, id: Uuid) -> Option<UserQueryResponse>;
    async fn get_users(
        &self,
        username: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        enabled: Option<bool>,
        offset: Option<u16>,
        count: Option<u16>,
    ) -> UserCollectionQueryResponse;
}
pub struct Test1SimpleMutationRequest {
    pub name: String,
}

pub struct Test1SimpleQueryRequest {
    pub name: String,
}

pub struct UserMutationRequest {
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub struct Test1SimpleMutationResponse {}

pub struct Test1SimpleQueryResponse {
    pub status: u16,
}

pub struct UserDbResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub enabled: bool,
}

pub struct UserMutationResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub enabled: bool,
}

pub struct UserQueryResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub enabled: bool,
}

pub trait MutationOutputBoundary {}

#[async_trait]
pub trait Test1DbGateway {
    async fn exists_by_name(&self, name: String) -> bool;
    async fn insert(&self, name: String, country: String) -> bool;
}

#[async_trait]
pub trait UserDbGateway {
    async fn activate_user(&self, id: Uuid) -> Result<User, DbError>;
    async fn deactivate_user(&self, id: Uuid) -> Result<User, DbError>;
    async fn exists_by_username(&self, username: String) -> bool;
    async fn insert(&self, user: &User) -> Result<(), DbError>;
    async fn get_user_by_id(&self, id: Uuid) -> Option<UserDbResponse>;
    async fn get_users(
        &self,
        username: Option<String>,
        phone: Option<String>,
        email: Option<String>,
        enabled: Option<bool>,
        offset: Option<u16>,
        count: Option<u16>,
    ) -> UserCollectionDbResponse;
    async fn update(&self, user: &User) -> Result<(), DbError>;
}

#[derive(Debug)]
pub enum UserMutationError {
    UniqueConstraintViolationError(String),
    IdCollisionError,
    InvalidUser,
    InvalidEmail,
    InvalidPhone,
    ExistedUser,
    UnknownError,
}

#[derive(Debug)]
pub enum DbError {
    UniqueConstraintViolationError(String),
    UnknownError,
}

pub struct UserCollectionQueryResponse {
    pub collection: Vec<UserQueryResponse>,
    pub has_more: Option<bool>,
}

pub struct UserCollectionDbResponse {
    pub collection: Vec<UserDbResponse>,
    pub has_more: Option<bool>,
}
// CommonUser
// CommonUserFactory
// JpaUser
// JpaUserRepository
// User
// UserDataMapper
// UserDsRequestModel
// UserFactory
// UserInputBoundary
// UserPresenter
// UserRegisterController
// UserRegisterDsGateway
// UserRegisterInteractor
// UserRequestModel
// UserResponseFormatter
// UserResponseModel
