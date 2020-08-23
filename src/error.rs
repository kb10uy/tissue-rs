//! エラー関連の型や操作を集約している。

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// チェックインデータのエラーを表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckinError {
    /// パラメーターが長すぎる
    TooLong,

    /// タグに空白が含まれている
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
