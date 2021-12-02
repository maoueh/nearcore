use std::fmt;

/// An error occurred when parsing an invalid Account ID with [`AccountId::validate`](crate::AccountId::validate).
#[derive(Eq, Clone, Debug, PartialEq)]
pub struct ParseAccountError(pub(crate) ParseErrorKind, pub(crate) String);

impl ParseAccountError {
    /// Returns the corresponding [`ParseErrorKind`] for this error.
    pub fn kind(&self) -> &ParseErrorKind {
        &self.0
    }

    /// Returns the corresponding [`AccountId`](crate::AccountId) for this error.
    pub fn get_account_id(self) -> String {
        self.1
    }
}

impl std::error::Error for ParseAccountError {}
impl fmt::Display for ParseAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]: {}", self.1, self.0)
    }
}

/// A list of errors that occur when parsing an invalid Account ID.
///
/// Also see [Error kind precedence](crate::AccountId#error-kind-precedence).
#[non_exhaustive]
#[derive(Eq, Clone, Debug, PartialEq)]
pub enum ParseErrorKind {
    /// The Account ID was too long.
    ///
    /// Thrown if the `AccountId` was longer than [`MAX_ACCOUNT_ID_LEN`](crate::MAX_ACCOUNT_ID_LEN).
    TooLong,
    /// The Account ID was too short.
    ///
    /// Thrown if the `AccountId` was longer than [`MIN_ACCOUNT_ID_LEN`](crate::MIN_ACCOUNT_ID_LEN).
    TooShort,
    /// The Account ID has upper-case characters.
    ///
    /// Example: `Emily.near`, `jemma.Dover.near`.
    HasCapsChars,
    /// The Account ID has separators immediately following each other.
    ///
    /// Example: `tyrell__wellick.near`.
    HasConsecutiveSeparators,
    /// The Account ID begins or ends with separators.
    ///
    /// Example: `_angela_moss_`.
    HasUnterminatedSeparators,
    /// The Account ID has invalid characters (non-separating symbol or space).
    ///
    /// Example: `ƒelicia.near`, `user@app.com`.
    HasInvalidChars,
}

impl ParseErrorKind {
    /// Returns `true` if the Account ID was too long.
    ///
    /// Thrown if the `AccountId` was longer than [`MAX_ACCOUNT_ID_LEN`](crate::MAX_ACCOUNT_ID_LEN).
    pub fn is_too_long(&self) -> bool {
        matches!(self, ParseErrorKind::TooLong)
    }

    /// Returns `true` if the Account ID was too short.
    ///
    /// Thrown if the `AccountId` was longer than [`MIN_ACCOUNT_ID_LEN`](crate::MIN_ACCOUNT_ID_LEN).
    pub fn is_too_short(&self) -> bool {
        matches!(self, ParseErrorKind::TooShort)
    }

    /// Returns `true` if the Account ID has upper-case characters.
    ///
    /// Example: `Emily.near`, `jemma.Dover.near`.
    pub fn has_caps_chars(&self) -> bool {
        matches!(self, ParseErrorKind::HasCapsChars)
    }

    /// Returns `true` if the Account ID has separators immediately following each other.
    ///
    /// Example: `tyrell__wellick.near`.
    pub fn has_consecutive_separators(&self) -> bool {
        matches!(self, ParseErrorKind::HasConsecutiveSeparators)
    }

    /// Returns `true` if the Account ID begins or ends with separators.
    ///
    /// Example: `_angela_moss_`.
    pub fn has_unterminated_separators(&self) -> bool {
        matches!(self, ParseErrorKind::HasUnterminatedSeparators)
    }

    /// Returns `true` if the Account ID has invalid characters (non-separating symbol or space).
    ///
    /// Example: `ƒelicia.near`, `user@app.com`.
    pub fn has_invalid_chars(&self) -> bool {
        matches!(self, ParseErrorKind::HasInvalidChars)
    }
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseErrorKind::TooLong => write!(f, "the account ID is too long"),
            ParseErrorKind::TooShort => write!(f, "the account ID is too short"),
            ParseErrorKind::HasCapsChars => write!(f, "the account ID has upper-case characters"),
            ParseErrorKind::HasConsecutiveSeparators => {
                write!(f, "the account ID has separators immediately following each other")
            }
            ParseErrorKind::HasUnterminatedSeparators => {
                write!(f, "the account ID begins or ends with a separator")
            }
            _ => write!(f, "the account ID has an invalid format"),
        }
    }
}
