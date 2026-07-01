use serde::Serialize;

use crate::{CommentId, MovieId, UserRef};
#[cfg(feature = "webhooks")]
use crate::WebhookEvent;

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
#[cfg(feature = "webhooks")]
#[derive(Clone, Debug, Default, Serialize)]
pub struct WebhookListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<crate::UserId>,
}

#[cfg(feature = "webhooks")]
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
#[cfg(feature = "webhooks")]
#[derive(Clone, Debug)]
pub struct WebhookEvents(Vec<WebhookEvent>);

#[cfg(feature = "webhooks")]
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

#[cfg(test)]
mod tests {
    use crate::{CommentId, MovieId, ScreenId, UserId, UserRef};

    use super::*;

    // ── MovieListRequest ────────────────────────────────────────────────────

    #[test]
    fn movie_list_request_default_omits_all_params() {
        let value = serde_json::to_value(MovieListRequest::default()).unwrap();
        assert_eq!(value, serde_json::json!({}));
    }

    #[test]
    fn movie_list_request_with_all_params() {
        let req = MovieListRequest::default()
            .offset(10)
            .limit(20)
            .slice_id(MovieId::new("500"));
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["offset"], 10);
        assert_eq!(value["limit"], 20);
        assert_eq!(value["slice_id"], "500");
    }

    #[test]
    fn movie_list_request_partial_params() {
        let req = MovieListRequest::default().limit(5);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["limit"], 5);
        assert!(value.get("offset").is_none());
        assert!(value.get("slice_id").is_none());
    }

    // ── CommentListRequest ───────────────────────────────────────────────────

    #[test]
    fn comment_list_request_default_omits_all_params() {
        let value = serde_json::to_value(CommentListRequest::default()).unwrap();
        assert_eq!(value, serde_json::json!({}));
    }

    #[test]
    fn comment_list_request_with_slice_id() {
        let req = CommentListRequest::default()
            .offset(0)
            .limit(50)
            .slice_id(CommentId::new("100"));
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["offset"], 0);
        assert_eq!(value["limit"], 50);
        assert_eq!(value["slice_id"], "100");
    }

    // ── UpcomingSchedulesRequest ─────────────────────────────────────────────

    #[test]
    fn upcoming_schedules_default_omits_in_days() {
        let value = serde_json::to_value(UpcomingSchedulesRequest::default()).unwrap();
        assert_eq!(value, serde_json::json!({}));
    }

    #[test]
    fn upcoming_schedules_with_in_days() {
        let req = UpcomingSchedulesRequest::default().in_days(30);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["in_days"], 30);
    }

    // ── ThumbnailOptions ─────────────────────────────────────────────────────

    #[test]
    fn thumbnail_options_defaults() {
        let value = serde_json::to_value(ThumbnailOptions::default()).unwrap();
        assert_eq!(value["size"], "small");
        assert_eq!(value["position"], "latest");
    }

    #[test]
    fn thumbnail_options_explicit() {
        let options = ThumbnailOptions {
            size: ThumbnailSize::Large,
            position: ThumbnailPosition::Beginning,
        };
        let value = serde_json::to_value(options).unwrap();
        assert_eq!(value["size"], "large");
        assert_eq!(value["position"], "beginning");
    }

    // ── Language ─────────────────────────────────────────────────────────────

    #[test]
    fn language_serialization() {
        assert_eq!(serde_json::to_value(Language::Ja).unwrap(), "ja");
        assert_eq!(serde_json::to_value(Language::En).unwrap(), "en");
    }

    // ── UserSearchRequest ───────────────────────────────────────────────────

    #[test]
    fn user_search_request_default_lang_is_ja() {
        let req = UserSearchRequest::new(SearchTerms::new("official"));
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["words"], "official");
        assert_eq!(value["lang"], "ja");
        assert!(value.get("limit").is_none());
    }

    #[test]
    fn user_search_request_with_limit() {
        let req = UserSearchRequest::new(SearchTerms::new("test")).limit(25);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["limit"], 25);
    }

    // ── LiveSearchRequest ────────────────────────────────────────────────────

    #[test]
    fn live_search_tag() {
        let req = LiveSearchRequest::new(LiveSearchKind::Tag(SearchTerms::new("music")));
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["type"], "tag");
        assert_eq!(value["context"], "music");
    }

    #[test]
    fn live_search_word() {
        let req = LiveSearchRequest::new(LiveSearchKind::Word(SearchTerms::new("gaming")));
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["type"], "word");
        assert_eq!(value["context"], "gaming");
    }

    #[test]
    fn live_search_category() {
        let req = LiveSearchRequest::new(LiveSearchKind::Category("_system_channel_5".into()));
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["type"], "category");
        assert_eq!(value["context"], "_system_channel_5");
    }

    #[test]
    fn live_search_new() {
        let req = LiveSearchRequest::new(LiveSearchKind::New);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["type"], "new");
        assert!(value.get("context").is_none());
    }

    #[test]
    fn live_search_recommend() {
        let req = LiveSearchRequest::new(LiveSearchKind::Recommend);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["type"], "recommend");
        assert!(value.get("context").is_none());
    }

    #[test]
    fn live_search_default_lang_is_ja() {
        let req = LiveSearchRequest::new(LiveSearchKind::New);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["lang"], "ja");
    }

    #[test]
    fn live_search_with_limit() {
        let req = LiveSearchRequest::new(LiveSearchKind::Recommend).limit(50);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["limit"], 50);
    }

    // ── SupporterSort ────────────────────────────────────────────────────────

    #[test]
    fn supporter_sort_serialization() {
        assert_eq!(serde_json::to_value(SupporterSort::New).unwrap(), "new");
        assert_eq!(
            serde_json::to_value(SupporterSort::Ranking).unwrap(),
            "ranking"
        );
    }

    // ── SupporterListRequest ─────────────────────────────────────────────────

    #[test]
    fn supporter_list_request_default_omits_all_params() {
        let value = serde_json::to_value(SupporterListRequest::default()).unwrap();
        assert_eq!(value, serde_json::json!({}));
    }

    #[test]
    fn supporter_list_request_with_all_params() {
        let req = SupporterListRequest::default()
            .offset(10)
            .limit(20)
            .sort(SupporterSort::Ranking);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["offset"], 10);
        assert_eq!(value["limit"], 20);
        assert_eq!(value["sort"], "ranking");
    }

    // ── GiftRequest ──────────────────────────────────────────────────────────

    #[test]
    fn gift_request_default_omits_slice_id() {
        let value = serde_json::to_value(GiftRequest::default()).unwrap();
        assert_eq!(value, serde_json::json!({}));
    }

    #[test]
    fn gift_request_with_slice_id() {
        let req = GiftRequest::default().slice_id(42);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["slice_id"], 42);
    }

    // ── WebhookListRequest ───────────────────────────────────────────────────

    #[test]
    #[cfg(feature = "webhooks")]
    fn webhook_list_request_default_omits_all() {
        let value = serde_json::to_value(WebhookListRequest::default()).unwrap();
        assert_eq!(value, serde_json::json!({}));
    }

    #[test]
    #[cfg(feature = "webhooks")]
    fn webhook_list_request_with_user_id() {
        let req = WebhookListRequest::default().user_id(UserId::new("42"));
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["user_id"], "42");
        assert!(value.get("limit").is_none());
        assert!(value.get("offset").is_none());
    }

    #[test]
    #[cfg(feature = "webhooks")]
    fn webhook_list_request_with_pagination() {
        let req = WebhookListRequest::default().limit(10).offset(5);
        let value = serde_json::to_value(req).unwrap();
        assert_eq!(value["limit"], 10);
        assert_eq!(value["offset"], 5);
    }

    // ── CommentText / Subtitle / Hashtag ─────────────────────────────────────

    #[test]
    fn comment_text_serializes_transparently() {
        assert_eq!(
            serde_json::to_string(&CommentText::new("hello")).unwrap(),
            r#""hello""#
        );
    }

    #[test]
    fn subtitle_serializes_transparently() {
        assert_eq!(
            serde_json::to_string(&Subtitle::new("初見さん大歓迎！")).unwrap(),
            r#""初見さん大歓迎！""#
        );
    }

    #[test]
    fn hashtag_serializes_transparently() {
        let serialized = serde_json::to_string(&Hashtag::new("#stream")).unwrap();
        assert_eq!(serialized, "\"#stream\"");
    }

    // ── SupportBatch ─────────────────────────────────────────────────────────

    #[test]
    fn support_batch_wire_values() {
        let batch = SupportBatch::new([
            UserRef::Id(UserId::new("42")),
            UserRef::ScreenId(ScreenId::new("caster")),
        ]);
        // SupportBatch itself serializes via the internal Body struct in resources,
        // but wire_values() is the key transformation.
        let values = batch.wire_values();
        assert_eq!(values, vec!["42", "caster"]);
    }

    // ── WebhookEvents ────────────────────────────────────────────────────────

    #[test]
    #[cfg(feature = "webhooks")]
    fn webhook_events_as_slice() {
        let events = WebhookEvents::new([WebhookEvent::LiveStart, WebhookEvent::LiveEnd]);
        assert_eq!(
            events.as_slice(),
            &[WebhookEvent::LiveStart, WebhookEvent::LiveEnd]
        );
    }
}
