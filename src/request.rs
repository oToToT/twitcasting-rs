use serde::Serialize;

use crate::{CommentId, MovieId, UserRef, WebhookEvent};

/// Pagination for movie history.
#[derive(Clone, Debug, Default, Serialize)]
pub struct MovieListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slice_id: Option<MovieId>,
}

impl MovieListRequest {
    /// Sets the 0–1000 offset.
    pub fn offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }

    /// Sets the 1–50 result limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    /// Selects movies older than this ID.
    #[must_use]
    pub fn slice_id(mut self, value: MovieId) -> Self {
        self.slice_id = Some(value);
        self
    }
}

/// Pagination for comments.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CommentListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slice_id: Option<CommentId>,
}

impl CommentListRequest {
    /// Sets a nonnegative offset.
    #[must_use]
    pub fn offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }

    /// Sets the 1–50 result limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    /// Selects comments after this ID.
    #[must_use]
    pub fn slice_id(mut self, value: CommentId) -> Self {
        self.slice_id = Some(value);
        self
    }
}

/// Comment text sent to TwitCasting for server-side validation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct CommentText(String);

impl CommentText {
    /// Creates comment text.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the comment text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Current-live subtitle sent to TwitCasting for server-side validation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Subtitle(String);

impl Subtitle {
    /// Creates a subtitle.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the subtitle.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Current-live hashtag sent to TwitCasting for server-side validation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct Hashtag(String);

impl Hashtag {
    /// Creates a hashtag.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the hashtag.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A batch of user references for support mutation.
#[derive(Clone, Debug)]
pub struct SupportBatch(Vec<UserRef>);

impl SupportBatch {
    /// Creates a support batch.
    #[must_use]
    pub fn new(values: impl IntoIterator<Item = UserRef>) -> Self {
        Self(values.into_iter().collect())
    }

    pub(crate) fn wire_values(&self) -> Vec<&str> {
        self.0.iter().map(UserRef::as_str).collect()
    }
}

/// Upcoming schedule query.
#[derive(Clone, Debug, Default, Serialize)]
pub struct UpcomingSchedulesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    in_days: Option<u32>,
}

impl UpcomingSchedulesRequest {
    /// Sets the inclusive 0–90 day range.
    pub fn in_days(mut self, value: u32) -> Self {
        self.in_days = Some(value);
        self
    }
}

/// Thumbnail image size.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ThumbnailSize {
    /// Small image.
    #[default]
    Small,
    /// Large image.
    Large,
}

/// Thumbnail capture position.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ThumbnailPosition {
    /// Most recent frame.
    #[default]
    Latest,
    /// Frame at broadcast start.
    Beginning,
}

/// Live-thumbnail options.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct ThumbnailOptions {
    /// Requested image size.
    pub size: ThumbnailSize,
    /// Requested capture position.
    pub position: ThumbnailPosition,
}

/// Supported request language.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    /// Japanese.
    Ja,
    /// English (supported by categories).
    En,
}

/// Search words sent to TwitCasting for server-side validation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct SearchTerms(String);

impl SearchTerms {
    /// Creates search terms.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

/// User-search request.
#[derive(Clone, Debug, Serialize)]
pub struct UserSearchRequest {
    words: SearchTerms,
    lang: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl UserSearchRequest {
    /// Creates a Japanese user search.
    #[must_use]
    pub fn new(words: SearchTerms) -> Self {
        Self {
            words,
            lang: Language::Ja,
            limit: None,
        }
    }

    /// Sets the 1–50 result limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}

/// Live-search kind and its required context.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiveSearchKind {
    /// Search hashtags.
    Tag(SearchTerms),
    /// Search words.
    Word(SearchTerms),
    /// Search a subcategory ID.
    Category(String),
    /// New broadcasts.
    New,
    /// Recommended broadcasts.
    Recommend,
}

/// Live-search request.
#[derive(Clone, Debug, Serialize)]
pub struct LiveSearchRequest {
    #[serde(rename = "type")]
    kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
    lang: Language,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
}

impl LiveSearchRequest {
    /// Creates a live search.
    #[must_use]
    pub fn new(kind: LiveSearchKind) -> Self {
        let (kind, context) = match kind {
            LiveSearchKind::Tag(value) => ("tag", Some(value.0)),
            LiveSearchKind::Word(value) => ("word", Some(value.0)),
            LiveSearchKind::Category(value) => ("category", Some(value)),
            LiveSearchKind::New => ("new", None),
            LiveSearchKind::Recommend => ("recommend", None),
        };
        Self {
            kind,
            context,
            lang: Language::Ja,
            limit: None,
        }
    }

    /// Sets the 1–100 result limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }
}

/// Supporter-list order.
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SupporterSort {
    /// Newest support relationships first.
    New,
    /// Contribution ranking.
    Ranking,
}

/// Supporter/supporting list pagination.
#[derive(Clone, Debug, Default, Serialize)]
pub struct SupporterListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<SupporterSort>,
}

impl SupporterListRequest {
    /// Sets a nonnegative offset.
    #[must_use]
    pub fn offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }

    /// Sets the 1–20 result limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    /// Sets supporter order.
    #[must_use]
    pub fn sort(mut self, value: SupporterSort) -> Self {
        self.sort = Some(value);
        self
    }
}

/// Gift polling cursor.
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct GiftRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    slice_id: Option<i64>,
}

impl GiftRequest {
    /// Sets a cursor of -1 or greater.
    pub fn slice_id(mut self, value: i64) -> Self {
        self.slice_id = Some(value);
        self
    }
}

/// Webhook-list filtering and pagination.
#[derive(Clone, Debug, Default, Serialize)]
pub struct WebhookListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
}

impl WebhookListRequest {
    /// Sets the 1–50 result limit.
    pub fn limit(mut self, value: u32) -> Self {
        self.limit = Some(value);
        self
    }

    /// Sets a nonnegative offset.
    #[must_use]
    pub fn offset(mut self, value: u32) -> Self {
        self.offset = Some(value);
        self
    }

    /// Filters by immutable user ID.
    #[must_use]
    pub fn user_id(mut self, value: crate::UserId) -> Self {
        self.user_id = Some(value);
        self
    }
}

/// Webhook events sent to TwitCasting for server-side validation.
#[derive(Clone, Debug)]
pub struct WebhookEvents(Vec<WebhookEvent>);

impl WebhookEvents {
    /// Creates a webhook event list.
    #[must_use]
    pub fn new(values: impl IntoIterator<Item = WebhookEvent>) -> Self {
        Self(values.into_iter().collect())
    }

    pub(crate) fn as_slice(&self) -> &[WebhookEvent] {
        &self.0
    }
}
