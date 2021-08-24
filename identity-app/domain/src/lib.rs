pub fn test_func() {
    println!("hello");
}

pub mod boundaries;
pub mod entity;
pub mod interactors;

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
