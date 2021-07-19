//! Contains error types.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Describes an error on checkins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckinError {
    /// Too long parameters
    TooLong,

    /// Some tag have whitespaces
    HasWhitespaces,
}

impl Display for CheckinError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            CheckinError::TooLong => write!(f, "The parameter was too long"),
            CheckinError::HasWhitespaces => write!(f, "The parameter had whitespaces"),
        }
    }
}

impl Error for CheckinError {}
