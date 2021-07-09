use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub(crate) enabled: bool,
}

impl User {
    pub(crate) fn is_valid(&self) -> bool {
        println!("checking if id {} is valid", (*self).username);
        true
    }
}
