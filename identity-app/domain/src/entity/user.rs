pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) username: String,
    pub(crate) email: String,
    pub(crate) phone: String,
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
