//! チェックインパラメーター関連の型を集約する。

use crate::error::CheckinError;
use std::fmt::Display;

use chrono::prelude::*;
use serde::Serialize;

/// 有効なチェックインのパラメーターを表す。
#[derive(Debug, PartialEq, Eq, Hash, Serialize)]
pub struct Checkin {
    checked_in_at: String,
    note: Option<String>,
    link: Option<String>,
    tags: Box<[String]>,
    is_private: Option<bool>,
    is_too_sensitive: Option<bool>,
}

impl Checkin {
    /// チェックイン時刻
    pub fn checked_in_at(&self) -> &str {
        &self.checked_in_at
    }

    /// チェックインノート
    pub fn note(&self) -> Option<&String> {
        self.note.as_ref()
    }

    /// オカズリンク
    pub fn link(&self) -> Option<&String> {
        self.link.as_ref()
    }

    /// タグ
    pub fn tags(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }

    /// 非公開フラグ
    pub fn is_private(&self) -> Option<bool> {
        self.is_private
    }

    /// 過激フラグ
    pub fn is_too_sensitive(&self) -> Option<bool> {
        self.is_too_sensitive
    }
}

/// チェックインのパラメーターを構築する。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CheckinBuilder<Tz: TimeZone>
where
    <Tz as TimeZone>::Offset: Display,
{
    checked_in_at: DateTime<Tz>,
    note: Option<String>,
    link: Option<String>,
    tags: Vec<String>,
    is_private: Option<bool>,
    is_too_sensitive: Option<bool>,
}

impl<Tz: TimeZone> CheckinBuilder<Tz>
where
    <Tz as TimeZone>::Offset: Display,
{
    /// ローカルタイムゾーンでチェックインを作成する。
    pub fn new_local() -> CheckinBuilder<Local> {
        CheckinBuilder {
            checked_in_at: Local::now(),
            note: None,
            link: None,
            tags: vec![],
            is_private: None,
            is_too_sensitive: None,
        }
    }

    /// UTC でチェックインを作成する。
    pub fn new_utc() -> CheckinBuilder<Utc> {
        CheckinBuilder {
            checked_in_at: Utc::now(),
            note: None,
            link: None,
            tags: vec![],
            is_private: None,
            is_too_sensitive: None,
        }
    }

    /// `DateTime` を指定してチェックインを作成する。
    pub fn with_datetime(checked_in_at: DateTime<Tz>) -> CheckinBuilder<Tz> {
        CheckinBuilder {
            checked_in_at,
            note: None,
            link: None,
            tags: vec![],
            is_private: None,
            is_too_sensitive: None,
        }
    }

    /// チェックインノートを設定する。
    /// 2000 bytes 以上の場合は `Err(CheckinError::TooLong)` が返る。
    pub fn note(&mut self, text: &str) -> Result<(), CheckinError> {
        if text.chars().count() <= 500 {
            self.note = Some(text.into());
            Ok(())
        } else {
            Err(CheckinError::TooLong)
        }
    }

    /// オカズリンクを設定する。
    /// 500 bytes 以上の場合は `Err(CheckinError::TooLong)` が返る。
    pub fn link(&mut self, link: &str) -> Result<(), CheckinError> {
        if link.chars().count() <= 2000 {
            self.link = Some(link.into());
            Ok(())
        } else {
            Err(CheckinError::TooLong)
        }
    }

    /// タグを設定する。先頭と末尾の空白は削除される。
    /// 途中に空白が含まれている場合、 `Err(CheckinError::HasWhitespaces)` が返る。
    pub fn tags<T: AsRef<str>, I: IntoIterator<Item = T>>(
        &mut self,
        tags: I,
    ) -> Result<(), CheckinError> {
        let mut validated = vec![];
        for tag in tags {
            let tag_str = tag.as_ref().trim();
            if tag_str == "" {
                continue;
            }

            if tag_str.chars().any(|c| c.is_whitespace()) {
                return Err(CheckinError::HasWhitespaces);
            } else {
                validated.push(tag_str.to_owned());
            }
        }

        Ok(())
    }

    /// 非公開フラグを設定する。
    pub fn is_private(&mut self, is_private: bool) {
        self.is_private = Some(is_private);
    }

    /// 過激フラグを設定する。
    pub fn is_too_sensitive(&mut self, is_too_sensitive: bool) {
        self.is_too_sensitive = Some(is_too_sensitive);
    }

    /// チェックインパラメーターを生成する。
    pub fn build(self) -> Checkin {
        Checkin {
            checked_in_at: self
                .checked_in_at
                .to_rfc3339_opts(SecondsFormat::Secs, true),
            note: self.note,
            link: self.link,
            tags: self.tags.into_boxed_slice(),
            is_private: self.is_private,
            is_too_sensitive: self.is_too_sensitive,
        }
    }
}
