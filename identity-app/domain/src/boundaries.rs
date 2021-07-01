use async_trait::async_trait;
use crate::entity::user::User;

#[async_trait]
pub trait Test1SimpleQueryInputBoundary {
    async fn get_test1(&self, request: Test1SimpleQueryRequest) -> Test1SimpleQueryResponse;
}

pub trait Test1SimpleMutationInputBoundary {
    fn create_test1(&self, request: Test1SimpleMutationRequest) -> Test1SimpleMutationResponse;
}

pub trait UserSimpleMutationInputBoundary {
    fn create_user(&self, request: UserDbRequest) -> UserDbResponse;
}

pub struct Test1SimpleMutationRequest {
    pub name: String,
}
pub struct Test1SimpleQueryRequest {
    pub name: String,
}

pub struct UserMutationRequest {
    pub id: Uuid,
}

pub struct UserDbRequest {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>
}

pub struct Test1SimpleMutationResponse {}
pub struct Test1SimpleQueryResponse {
    pub status: u16,
}

pub struct UserSimpleMutationResponse {
    pub id: Option<uuid>,
}

pub struct UserDbResponse {
    pub id: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub enabled: Option<bool>
}

pub trait MutationOutputBoundary {}

#[async_trait]
pub trait Test1DbGateway {
    async fn exists_by_name(&self, name: String) -> bool;
    async fn insert(&self, name: String, country: String) -> bool;
}

#[async_trait]
pub trait UserDbGateway {
    async fn exists_by_username(&self, username: String) -> bool;
    async fn insert(&self, user: User) -> bool;
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
