use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub phone: String,
    pub(crate) enabled: bool,
}

impl User {
    pub(crate) fn is_valid(&self) -> bool {
        println!(
            "checking if id {},{},{},{},{} is valid",
            (*self).id,
            (*self).username,
            (*self).email,
            (*self).phone,
            (*self).enabled,
        );
        true
    }
}