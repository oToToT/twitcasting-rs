use std::{fmt, str::FromStr};

use serde::Serialize;

macro_rules! string_id {
    ($name:ident, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates an identifier.
            #[must_use]
            pub fn new(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            /// Returns the wire value.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl FromStr for $name {
            type Err = std::convert::Infallible;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self::new(value))
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self::new(value)
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self::new(value)
            }
        }
    };
}

string_id!(UserId, "An immutable TwitCasting user identifier.");
string_id!(ScreenId, "A user-changeable TwitCasting screen identifier.");
string_id!(MovieId, "A live or recorded movie identifier.");
string_id!(CommentId, "A comment identifier.");
string_id!(LiveScheduleId, "A live schedule identifier.");

/// A user accepted by endpoints that allow either `id` or `screen_id`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UserRef {
    /// Immutable user ID.
    Id(UserId),
    /// Changeable screen ID.
    ScreenId(ScreenId),
}

impl UserRef {
    /// Returns the value used as a URL segment or parameter.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Id(value) => value.as_str(),
            Self::ScreenId(value) => value.as_str(),
        }
    }
}

impl From<UserId> for UserRef {
    fn from(value: UserId) -> Self {
        Self::Id(value)
    }
}

impl From<ScreenId> for UserRef {
    fn from(value: ScreenId) -> Self {
        Self::ScreenId(value)
    }
}

impl Serialize for UserRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::{MovieId, ScreenId, UserRef};

    #[test]
    fn identifiers_preserve_wire_values() {
        assert_eq!(MovieId::new("42").as_str(), "42");
    }

    #[test]
    fn user_ref_preserves_kind() {
        let value = UserRef::from(ScreenId::new("caster"));
        assert!(matches!(value, UserRef::ScreenId(_)));
    }
}
