mod checkin;
mod error;

pub use crate::{
    checkin::{Checkin, CheckinBuilder},
    error::CheckinError,
};

pub struct Endpoint(String);

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
