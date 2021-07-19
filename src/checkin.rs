//! Contains checkin types.

use crate::error::CheckinError;
use std::fmt::Display;

use chrono::prelude::*;
use serde::Serialize;

/// Describes a valid checkin.
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
    /// Timestamp of checkin.
    pub fn checked_in_at(&self) -> &str {
        &self.checked_in_at
    }

    /// Notes.
    pub fn note(&self) -> Option<&String> {
        self.note.as_ref()
    }

    /// Link.
    pub fn link(&self) -> Option<&String> {
        self.link.as_ref()
    }

    /// Tag(s).
    pub fn tags(&self) -> impl Iterator<Item = &String> {
        self.tags.iter()
    }

    /// Whether it is private or not.
    pub fn is_private(&self) -> Option<bool> {
        self.is_private
    }

    /// Whether it is too sensitive or not.
    pub fn is_too_sensitive(&self) -> Option<bool> {
        self.is_too_sensitive
    }
}

/// Builder for `Checkin`.
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
    /// Creates a new builder with local timezone.
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

    /// Creates a new builder with UTC.
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

    /// Creates a new builder with specified `DateTime`.
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

    /// Sets checkin note.
    /// Returns `Err(CheckinError::TooLong)` if `text` >= 2000 bytes.
    pub fn note(&mut self, text: &str) -> Result<(), CheckinError> {
        if text.chars().count() <= 500 {
            self.note = Some(text.into());
            Ok(())
        } else {
            Err(CheckinError::TooLong)
        }
    }

    /// Sets checkin link.
    /// Returns `Err(CheckinError::TooLong)` if `text` >= 500 bytes.
    pub fn link(&mut self, link: &str) -> Result<(), CheckinError> {
        if link.chars().count() <= 2000 {
            self.link = Some(link.into());
            Ok(())
        } else {
            Err(CheckinError::TooLong)
        }
    }

    /// Sets tags. For each tag, leading/trailing whitespaces will be removed.
    /// Returns `Err(CheckinError::HasWhitespaces)` if whitespaces found in the middle.
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

    /// Sets private flag.
    pub fn is_private(&mut self, is_private: bool) {
        self.is_private = Some(is_private);
    }

    /// Sets too-sensitive flag.
    pub fn is_too_sensitive(&mut self, is_too_sensitive: bool) {
        self.is_too_sensitive = Some(is_too_sensitive);
    }

    /// Builds `Checkin`.
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
