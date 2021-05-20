pub(crate) struct Test1 {
    pub(crate) id: i32,
    pub(crate) name: String,
}

impl Test1 {
    pub(crate) fn is_valid(&self) -> bool {
        println!(
            "checking if id {} and {} is valid",
            (*self).id,
            (*self).name
        );
        true
    }
}
