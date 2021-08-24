use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub enabled: bool,
    pub group: Option<Vec<String>>,
}
