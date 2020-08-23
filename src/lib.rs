mod checkin;
mod error;
mod tissue;

pub use crate::{
    checkin::{Checkin, CheckinBuilder},
    error::CheckinError,
    tissue::{CheckinResponse, IncomingEndpoint, ReceivedCheckin},
};

/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
