use domain::test_func;
use postgres::{Client, Error, NoTls};

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
