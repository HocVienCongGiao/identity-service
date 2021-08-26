pub use domain::boundaries::UserMutationResponse;
use domain::boundaries::{UserCollectionQueryResponse, UserMutationRequest, UserQueryResponse};
pub use hvcg_iam_openapi_identity::models::User;
use hvcg_iam_openapi_identity::models::{Group, UserCollection};

impl ToOpenApi<User> for UserMutationResponse {
    fn user_openapi(self) -> User {
        let mut user_group: Vec<String> = self.groups.clone();
        let mut groups: Vec<Group> = vec![];
        for i in &mut user_group {
            groups.push(get_group_name_open_api(i));
        }

        User {
            id: Option::from(self.id),
            username: self.username.to_string(),
            email: Option::from(self.email),
            phone: Option::from(self.phone),
            enabled: Option::from(self.enabled),
            groups: Option::from(groups),
        }
    }
}

impl ToOpenApi<User> for UserQueryResponse {
    fn user_openapi(self) -> User {
        let mut user_group: Vec<String> = self.groups.clone();
        let mut groups: Vec<Group> = vec![];
        for i in &mut user_group {
            groups.push(get_group_name_open_api(i));
        }
        User {
            id: Option::from(self.id),
            username: self.username.to_string(),
            email: Option::from(self.email),
            phone: Option::from(self.phone),
            enabled: Option::from(self.enabled),
            groups: Option::from(groups),
        }
    }
}

impl ToModel<UserMutationRequest> for &User {
    fn to_model(&self) -> UserMutationRequest {
        // let user_group: String = self.group.clone().unwrap().to_string();
        let mut user_group: Vec<Group> = self.groups.clone().unwrap();
        let mut groups: Vec<String> = vec![];
        for i in &mut user_group {
            groups.push(get_group_name(i))
        }
        UserMutationRequest {
            username: self.username.to_string(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            groups: Option::from(groups),
        }
    }
}
fn get_group_name(mut group: &Group) -> String {
    match group {
        Group::ADMIN_GROUP => "AdminGroup".to_string(),
        Group::OPERATOR_GROUP => "OperatorGroup".to_string(),
        Group::PROFESSOR_GROUP => "ProfessorGroup".to_string(),
        Group::STUDENT_GROUP => "StudentGroup".to_string(),
        Group::UNKNOWN => "Unknown".to_string(),
    }
}

fn get_group_name_open_api(mut group_name: &str) -> Group {
    let admin_group: &String = &"AdminGroup".to_string();
    let operator_group: &String = &"OperatorGroup".to_string();
    let professor_group: &String = &"ProfessorGroup".to_string();
    let student_group: &String = &"StudentGroup".to_string();
    if group_name.eq(admin_group) {
        Group::ADMIN_GROUP
    } else if group_name.eq(operator_group) {
        Group::OPERATOR_GROUP
    } else if group_name.eq(professor_group) {
        Group::PROFESSOR_GROUP
    } else if group_name.eq(student_group) {
        Group::STUDENT_GROUP
    } else {
        Group::UNKNOWN
    }
}
pub trait ToOpenApi<T> {
    fn user_openapi(self) -> T;
}

pub trait ToModel<T> {
    fn to_model(&self) -> T;
}

impl ToOpenApi<UserCollection> for UserCollectionQueryResponse {
    fn user_openapi(self) -> UserCollection {
        let collection = (self
            .collection
            .into_iter()
            .map(|user_query_response| user_query_response.user_openapi())
            .collect::<Vec<User>>())
        .to_vec();
        UserCollection {
            users: Some(collection),
            has_more: self.has_more,
        }
    }
}
