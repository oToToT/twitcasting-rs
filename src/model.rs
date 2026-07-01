use std::{
    fmt,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use url::Url;

use crate::{CommentId, LiveScheduleId, MovieId, ScreenId, SecretString, UserId};

/// A lossless signed Unix timestamp in seconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UnixTimestamp(pub i64);

impl UnixTimestamp {
    /// Returns the exact seconds since the Unix epoch.
    #[must_use]
    pub const fn seconds(self) -> i64 {
        self.0
    }

    /// Converts to `SystemTime`.
    #[must_use]
    pub fn to_system_time(self) -> SystemTime {
        if self.0 >= 0 {
            UNIX_EPOCH + Duration::from_secs(self.0.unsigned_abs())
        } else {
            UNIX_EPOCH - Duration::from_secs(self.0.unsigned_abs())
        }
    }

    /// Converts from `SystemTime`, returning `None` when whole seconds do not fit in `i64`.
    #[must_use]
    pub fn from_system_time(value: SystemTime) -> Option<Self> {
        match value.duration_since(UNIX_EPOCH) {
            Ok(duration) => i64::try_from(duration.as_secs()).ok().map(Self),
            Err(error) => {
                let seconds = error.duration().as_secs();
                i64::try_from(seconds)
                    .ok()
                    .and_then(i64::checked_neg)
                    .map(Self)
            }
        }
    }
}

/// Basic user information.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    /// Immutable user ID.
    pub id: UserId,
    /// Changeable screen ID.
    pub screen_id: ScreenId,
    /// Display name.
    pub name: String,
    /// User icon URL.
    pub image: Url,
    /// Profile text.
    pub profile: String,
    /// TwitCasting level.
    pub level: u32,
    /// Last broadcast, if any.
    pub last_movie_id: Option<MovieId>,
    /// Whether the user is currently live.
    pub is_live: bool,
    /// Deprecated wire field, currently fixed at zero.
    #[deprecated(note = "TwitCasting returns a fixed zero; use response-level counts")]
    #[serde(default)]
    pub supporter_count: u64,
    /// Deprecated wire field, currently fixed at zero.
    #[deprecated(note = "TwitCasting returns a fixed zero; use response-level counts")]
    #[serde(default)]
    pub supporting_count: u64,
    /// Deprecated wire field, currently fixed at zero.
    #[deprecated(note = "TwitCasting returns a fixed zero")]
    #[serde(default)]
    pub created: i64,
}

/// User lookup response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserInfo {
    /// User.
    pub user: User,
    /// Number of supporters.
    pub supporter_count: u64,
    /// Number of supported users.
    pub supporting_count: u64,
}

/// Registered application information.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Application {
    /// OAuth client ID.
    pub client_id: String,
    /// Application display name.
    pub name: String,
    /// Owner user ID.
    pub owner_user_id: UserId,
}

/// Credential verification response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifiedCredentials {
    /// Application bound to the token.
    pub app: Application,
    /// User bound to the token.
    pub user: User,
    /// Number of supporters.
    pub supporter_count: u64,
    /// Number of supported users.
    pub supporting_count: u64,
}

/// A planned live broadcast.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveSchedule {
    /// Schedule ID.
    pub id: LiveScheduleId,
    /// Owner user ID.
    pub user_id: UserId,
    /// Owner screen ID.
    pub user_screen_id: ScreenId,
    /// Planned start time.
    pub start_at: UnixTimestamp,
    /// Schedule title.
    pub title: String,
    /// Optional thumbnail.
    pub thumbnail: Option<Url>,
}

/// Upcoming schedules response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveSchedules {
    /// Upcoming schedules.
    pub live_schedules: Vec<LiveSchedule>,
}

/// A live or recorded movie.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Movie {
    /// Movie ID.
    pub id: MovieId,
    /// Broadcaster ID.
    pub user_id: UserId,
    /// Title.
    pub title: String,
    /// Current subtitle.
    pub subtitle: Option<String>,
    /// Latest owner comment.
    pub last_owner_comment: Option<String>,
    /// Subcategory ID.
    pub category: Option<String>,
    /// Public movie URL.
    pub link: Url,
    /// Whether currently live.
    pub is_live: bool,
    /// Whether the recording is public.
    pub is_recorded: bool,
    /// Total comments.
    pub comment_count: u64,
    /// Large thumbnail URL.
    pub large_thumbnail: Url,
    /// Small thumbnail URL.
    pub small_thumbnail: Url,
    /// Country code.
    pub country: String,
    /// Broadcast duration in seconds.
    pub duration: u64,
    /// Broadcast start time.
    pub created: UnixTimestamp,
    /// Whether collaboration mode was used.
    pub is_collabo: bool,
    /// Whether password protected.
    pub is_protected: bool,
    /// Whether this is a membership broadcast.
    #[serde(default)]
    pub is_membership: bool,
    /// Whether this is a premier broadcast.
    #[serde(default)]
    pub is_premier: bool,
    /// Maximum concurrent viewers.
    pub max_view_count: u64,
    /// Current concurrent viewers.
    pub current_view_count: u64,
    /// Total viewers.
    pub total_view_count: u64,
    /// HLS playback URL, when available.
    pub hls_url: Option<Url>,
}

/// Expanded movie response used by movie info, current live, and live search.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieInfo {
    /// Movie.
    pub movie: Movie,
    /// Broadcaster.
    pub broadcaster: User,
    /// Applied tags.
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Movie history response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MovieList {
    /// Total matching records.
    pub total_count: u64,
    /// Returned movies.
    pub movies: Vec<Movie>,
}

/// Subtitle mutation response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubtitleUpdate {
    /// Current movie.
    pub movie_id: MovieId,
    /// New subtitle, or `None` after unsetting.
    pub subtitle: Option<String>,
}

/// Hashtag mutation response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HashtagUpdate {
    /// Current movie.
    pub movie_id: MovieId,
    /// New hashtag, or `None` after unsetting.
    pub hashtag: Option<String>,
}

/// A movie comment.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Comment {
    /// Comment ID.
    pub id: CommentId,
    /// Comment body.
    pub message: String,
    /// Author.
    pub from_user: User,
    /// Creation time.
    pub created: UnixTimestamp,
}

/// Comment list response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommentList {
    /// Movie ID.
    pub movie_id: MovieId,
    /// Total comments.
    pub all_count: u64,
    /// Returned comments.
    pub comments: Vec<Comment>,
}

/// Comment creation response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostedComment {
    /// Movie ID.
    pub movie_id: MovieId,
    /// Total comments after creation.
    pub all_count: u64,
    /// Created comment.
    pub comment: Comment,
}

/// Comment deletion response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeletedComment {
    /// Deleted comment ID.
    pub comment_id: CommentId,
}

fn deserialize_string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Value {
        String(String),
        Signed(i64),
        Unsigned(u64),
    }
    Ok(match Value::deserialize(deserializer)? {
        Value::String(value) => value,
        Value::Signed(value) => value.to_string(),
        Value::Unsigned(value) => value.to_string(),
    })
}

/// A recently sent gift.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Gift {
    /// Gift send ID. The API has returned both JSON strings and numbers.
    #[serde(deserialize_with = "deserialize_string_or_number")]
    pub id: String,
    /// Sender message.
    pub message: String,
    /// Main item image.
    pub item_image: Url,
    /// Selected sub-image, if any.
    #[serde(default)]
    pub item_sub_image: Option<Url>,
    /// Item ID.
    pub item_id: String,
    /// Item MP value as returned by the API.
    pub item_mp: String,
    /// Item display name.
    pub item_name: String,
    /// Sender icon URL.
    pub user_image: Url,
    /// Screen ID at send time.
    pub user_screen_id: ScreenId,
    /// Human-readable screen name.
    pub user_screen_name: String,
    /// Sender display name.
    pub user_name: String,
}

/// Gift polling response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GiftList {
    /// Cursor for the next request.
    pub slice_id: i64,
    /// Recent gifts.
    pub gifts: Vec<Gift>,
}

/// Support relationship status.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SupportingStatus {
    /// Whether the source supports the target.
    pub is_supporting: bool,
    /// Relationship creation time, when supplied.
    #[serde(default)]
    pub supported: Option<UnixTimestamp>,
    /// Target user.
    pub target_user: User,
}

/// Batch support result.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SupportResult {
    /// Newly added relationships.
    pub added_count: u64,
}

/// Batch unsupport result.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UnsupportResult {
    /// Removed relationships.
    pub removed_count: u64,
}

/// User enriched with support contribution information.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SupporterUser {
    /// Immutable user ID.
    pub id: UserId,
    /// Changeable screen ID.
    pub screen_id: ScreenId,
    /// Display name.
    pub name: String,
    /// Icon URL.
    pub image: Url,
    /// Profile.
    pub profile: String,
    /// Level.
    pub level: u32,
    /// Last movie.
    pub last_movie_id: Option<MovieId>,
    /// Whether live.
    pub is_live: bool,
    /// Relationship creation time.
    pub supported: UnixTimestamp,
    /// Supporter count.
    #[serde(default)]
    pub supporter_count: u64,
    /// Supporting count.
    #[serde(default)]
    pub supporting_count: u64,
    /// Current item score.
    pub point: i64,
    /// Lifetime item score.
    pub total_point: i64,
    /// Deprecated fixed-zero field.
    #[deprecated(note = "TwitCasting returns a fixed zero")]
    #[serde(default)]
    pub created: i64,
}

/// Supporting-list response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SupportingList {
    /// Total records.
    pub total: u64,
    /// Supported users.
    pub supporting: Vec<SupporterUser>,
}

/// Supporter-list response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SupporterList {
    /// Total records.
    pub total: u64,
    /// Supporters.
    pub supporters: Vec<SupporterUser>,
}

/// Top-level live category.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Category {
    /// Category ID.
    pub id: String,
    /// Localized name.
    pub name: String,
    /// Live subcategories.
    pub sub_categories: Vec<SubCategory>,
}

/// Live subcategory.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubCategory {
    /// Subcategory ID.
    pub id: String,
    /// Localized name.
    pub name: String,
    /// Current live count.
    pub count: u64,
}

/// Category-list response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CategoryList {
    /// Live categories.
    pub categories: Vec<Category>,
}

/// User-search response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserSearchResults {
    /// Matching users.
    pub users: Vec<User>,
}

/// Live-search response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveSearchResults {
    /// Matching expanded movies.
    pub movies: Vec<MovieInfo>,
}

/// Webhook event, preserving unknown future values.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum WebhookEvent {
    /// A broadcast started.
    LiveStart,
    /// A broadcast ended.
    LiveEnd,
    /// A schedule was created.
    LiveScheduleCreate,
    /// A schedule was updated.
    LiveScheduleUpdate,
    /// A schedule was deleted.
    LiveScheduleDelete,
    /// A value introduced after this crate version.
    Unknown(String),
}

impl WebhookEvent {
    /// Returns the wire event key.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::LiveStart => "livestart",
            Self::LiveEnd => "liveend",
            Self::LiveScheduleCreate => "liveschedulecreate",
            Self::LiveScheduleUpdate => "livescheduleupdate",
            Self::LiveScheduleDelete => "livescheduledelete",
            Self::Unknown(value) => value,
        }
    }
}

impl Serialize for WebhookEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for WebhookEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match String::deserialize(deserializer)?.as_str() {
            "livestart" => Self::LiveStart,
            "liveend" => Self::LiveEnd,
            "liveschedulecreate" => Self::LiveScheduleCreate,
            "livescheduleupdate" => Self::LiveScheduleUpdate,
            "livescheduledelete" => Self::LiveScheduleDelete,
            value => Self::Unknown(value.to_owned()),
        })
    }
}

/// Registered webhook row.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Webhook {
    /// Target user.
    pub user_id: UserId,
    /// Event.
    pub event: WebhookEvent,
}

/// Webhook-list response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct WebhookList {
    /// Total registered hooks.
    pub all_count: u64,
    /// Returned hooks.
    pub webhooks: Vec<Webhook>,
}

/// Webhook registration response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddedWebhooks {
    /// Target user.
    pub user_id: UserId,
    /// Events newly registered by this call.
    pub added_events: Vec<WebhookEvent>,
}

/// Webhook removal response.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeletedWebhooks {
    /// Target user.
    pub user_id: UserId,
    /// Events removed by this call.
    pub deleted_events: Vec<WebhookEvent>,
}

/// A secret-bearing URL with redacted formatting.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretUrl(Url);

impl SecretUrl {
    /// Exposes the URL to the broadcaster configuration code that needs it.
    #[must_use]
    pub fn expose_secret(&self) -> &Url {
        &self.0
    }
}

impl fmt::Debug for SecretUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecretUrl([REDACTED])")
    }
}

impl fmt::Display for SecretUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

impl<'de> Deserialize<'de> for SecretUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Url::deserialize(deserializer).map(Self)
    }
}

/// RTMP publishing configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct RtmpCredentials {
    /// Whether RTMP broadcasting is enabled.
    pub enabled: bool,
    /// Publishing URL, including secret query data.
    pub url: Option<SecretUrl>,
    /// Stream key.
    pub stream_key: Option<SecretString>,
}

/// OAuth access-token response.
#[derive(Clone, Debug, Deserialize)]
pub struct AccessToken {
    /// Token type, preserving unknown values.
    pub token_type: TokenType,
    /// Lifetime in seconds.
    pub expires_in: u64,
    /// Bearer token.
    pub access_token: SecretString,
}

/// OAuth token type, preserving unknown future values.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    /// Bearer token.
    Bearer,
    /// Unknown token type.
    Unknown(String),
}

impl<'de> Deserialize<'de> for TokenType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.eq_ignore_ascii_case("bearer") {
            Ok(Self::Bearer)
        } else {
            Ok(Self::Unknown(value))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, UNIX_EPOCH};
    use url::Url;

    use super::{
        Application, Category, CategoryList, Comment, CommentList, DeletedComment, Gift, GiftList,
        HashtagUpdate, LiveSchedule, LiveSchedules, Movie, MovieInfo, MovieList, PostedComment,
        SubCategory, SubtitleUpdate, SupporterList, SupporterUser, SupportingList,
        SupportingStatus, TokenType, UnixTimestamp, User, UserInfo, UserSearchResults,
        VerifiedCredentials, Webhook, WebhookEvent, WebhookList,
    };

    #[test]
    fn timestamps_round_trip_before_and_after_epoch() {
        for timestamp in [UnixTimestamp(-42), UnixTimestamp(0), UnixTimestamp(42)] {
            assert_eq!(
                UnixTimestamp::from_system_time(timestamp.to_system_time()),
                Some(timestamp)
            );
        }
        assert_eq!(
            UnixTimestamp::from_system_time(UNIX_EPOCH + Duration::from_secs(7)),
            Some(UnixTimestamp(7))
        );
    }

    #[test]
    fn unknown_enums_are_preserved() {
        let event: WebhookEvent = serde_json::from_str("\"futureevent\"").unwrap();
        assert_eq!(event, WebhookEvent::Unknown("futureevent".into()));
        let token: TokenType = serde_json::from_str("\"DPoP\"").unwrap();
        assert_eq!(token, TokenType::Unknown("DPoP".into()));
    }

    // ── WebhookEvent round-trip ──────────────────────────────────────────────

    fn webhook_event_round_trip(event: WebhookEvent, expected_wire: &str) {
        let serialized = serde_json::to_string(&event).unwrap();
        assert_eq!(serialized, format!("\"{expected_wire}\""));
        let deserialized: WebhookEvent = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, event);
    }

    #[test]
    fn webhook_event_live_start_round_trip() {
        webhook_event_round_trip(WebhookEvent::LiveStart, "livestart");
    }

    #[test]
    fn webhook_event_live_end_round_trip() {
        webhook_event_round_trip(WebhookEvent::LiveEnd, "liveend");
    }

    #[test]
    fn webhook_event_schedule_create_round_trip() {
        webhook_event_round_trip(WebhookEvent::LiveScheduleCreate, "liveschedulecreate");
    }

    #[test]
    fn webhook_event_schedule_update_round_trip() {
        webhook_event_round_trip(WebhookEvent::LiveScheduleUpdate, "livescheduleupdate");
    }

    #[test]
    fn webhook_event_schedule_delete_round_trip() {
        webhook_event_round_trip(WebhookEvent::LiveScheduleDelete, "livescheduledelete");
    }

    #[test]
    fn webhook_event_unknown_round_trip() {
        webhook_event_round_trip(WebhookEvent::Unknown("custom".into()), "custom");
    }

    // ── TokenType round-trip ─────────────────────────────────────────────────

    #[test]
    fn token_type_bearer_case_insensitive() {
        for wire in [r#""Bearer""#, r#""bearer""#, r#""BEARER""#] {
            let token: TokenType = serde_json::from_str(wire).unwrap();
            assert_eq!(token, TokenType::Bearer);
        }
    }

    // ── User object ──────────────────────────────────────────────────────────

    #[allow(deprecated)]
    fn sample_user() -> User {
        User {
            id: crate::UserId::new("1"),
            screen_id: crate::ScreenId::new("caster"),
            name: "Caster".into(),
            image: Url::parse("https://x.com/i.png").unwrap(),
            profile: "Hi".into(),
            level: 5,
            last_movie_id: Some(crate::MovieId::new("10")),
            is_live: true,
            supporter_count: 0,
            supporting_count: 0,
            created: 0,
        }
    }

    #[test]
    fn user_round_trip() {
        let user = sample_user();
        let json = serde_json::to_value(&user).unwrap();
        let deserialized: User = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.id.as_str(), "1");
        assert!(deserialized.is_live);
    }

    #[allow(deprecated)]
    #[test]
    fn user_accepts_missing_deprecated_fields() {
        let json = r#"{
            "id":"1","screen_id":"s","name":"N","image":"https://x.com/i.png",
            "profile":"","level":1,"last_movie_id":null,"is_live":false
        }"#;
        // Without supporter_count, supporting_count, created — all default to 0
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.supporter_count, 0);
        assert_eq!(user.supporting_count, 0);
        assert_eq!(user.created, 0);
    }

    // ── UserInfo ─────────────────────────────────────────────────────────────

    #[test]
    fn user_info_round_trip() {
        let info = UserInfo {
            user: sample_user(),
            supporter_count: 10,
            supporting_count: 24,
        };
        let json = serde_json::to_value(&info).unwrap();
        let deserialized: UserInfo = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.supporter_count, 10);
        assert_eq!(deserialized.supporting_count, 24);
    }

    // ── Movie ────────────────────────────────────────────────────────────────

    fn sample_movie() -> Movie {
        Movie {
            id: crate::MovieId::new("100"),
            user_id: crate::UserId::new("1"),
            title: "Live".into(),
            subtitle: Some("Hello".into()),
            last_owner_comment: None,
            category: Some("music".into()),
            link: Url::parse("https://x.com/m/100").unwrap(),
            is_live: true,
            is_recorded: false,
            comment_count: 42,
            large_thumbnail: Url::parse("https://x.com/l.jpg").unwrap(),
            small_thumbnail: Url::parse("https://x.com/s.jpg").unwrap(),
            country: "jp".into(),
            duration: 3600,
            created: UnixTimestamp(1000000000),
            is_collabo: false,
            is_protected: false,
            is_membership: false,
            is_premier: false,
            max_view_count: 0,
            current_view_count: 5,
            total_view_count: 100,
            hls_url: Some(Url::parse("https://x.com/stream.m3u8").unwrap()),
        }
    }

    #[test]
    fn movie_round_trip() {
        let movie = sample_movie();
        let json = serde_json::to_value(&movie).unwrap();
        let deserialized: Movie = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.id.as_str(), "100");
        assert_eq!(deserialized.comment_count, 42);
    }

    #[test]
    fn movie_with_null_subtitle_and_hls() {
        let json = r#"{
            "id":"1","user_id":"1","title":"T",
            "subtitle":null,"last_owner_comment":null,"category":null,
            "link":"https://x.com/m/1","is_live":false,"is_recorded":true,
            "comment_count":0,
            "large_thumbnail":"https://x.com/l.jpg","small_thumbnail":"https://x.com/s.jpg",
            "country":"jp","duration":0,"created":1000000000,
            "is_collabo":false,"is_protected":false,
            "is_membership":true,"is_premier":false,
            "max_view_count":0,"current_view_count":0,"total_view_count":0,
            "hls_url":null
        }"#;
        let movie: Movie = serde_json::from_str(json).unwrap();
        assert!(movie.subtitle.is_none());
        assert!(movie.hls_url.is_none());
        assert!(movie.is_membership);
        assert!(!movie.is_premier);
    }

    #[test]
    fn movie_accepts_missing_membership_and_premier_fields() {
        let json = r#"{
            "id":"1","user_id":"1","title":"T",
            "subtitle":null,"last_owner_comment":null,"category":null,
            "link":"https://x.com/m/1","is_live":false,"is_recorded":true,
            "comment_count":0,
            "large_thumbnail":"https://x.com/l.jpg","small_thumbnail":"https://x.com/s.jpg",
            "country":"jp","duration":0,"created":1000000000,
            "is_collabo":false,"is_protected":false,
            "max_view_count":0,"current_view_count":0,"total_view_count":0,
            "hls_url":null
        }"#;
        let movie: Movie = serde_json::from_str(json).unwrap();
        assert!(!movie.is_membership);
        assert!(!movie.is_premier);
    }

    // ── MovieInfo ────────────────────────────────────────────────────────────

    #[test]
    fn movie_info_round_trip() {
        let info = MovieInfo {
            movie: sample_movie(),
            broadcaster: sample_user(),
            tags: vec!["tag1".into(), "tag2".into()],
        };
        let json = serde_json::to_value(&info).unwrap();
        let deserialized: MovieInfo = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.tags, vec!["tag1", "tag2"]);
    }

    // ── MovieList ────────────────────────────────────────────────────────────

    #[test]
    fn movie_list_round_trip() {
        let list = MovieList {
            total_count: 1,
            movies: vec![sample_movie()],
        };
        let json = serde_json::to_value(&list).unwrap();
        let deserialized: MovieList = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.total_count, 1);
        assert_eq!(deserialized.movies.len(), 1);
    }

    // ── SubtitleUpdate ───────────────────────────────────────────────────────

    #[test]
    fn subtitle_update_round_trip() {
        let update = SubtitleUpdate {
            movie_id: crate::MovieId::new("1"),
            subtitle: Some("hello".into()),
        };
        let json = serde_json::to_value(&update).unwrap();
        let deserialized: SubtitleUpdate = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.subtitle.as_deref(), Some("hello"));
    }

    #[test]
    fn subtitle_update_null_round_trip() {
        let update = SubtitleUpdate {
            movie_id: crate::MovieId::new("1"),
            subtitle: None,
        };
        let json = serde_json::to_value(&update).unwrap();
        assert_eq!(json["subtitle"], serde_json::Value::Null);
    }

    // ── HashtagUpdate ────────────────────────────────────────────────────────

    #[test]
    fn hashtag_update_round_trip() {
        let update = HashtagUpdate {
            movie_id: crate::MovieId::new("1"),
            hashtag: Some("#tag".into()),
        };
        let json = serde_json::to_value(&update).unwrap();
        let deserialized: HashtagUpdate = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.hashtag.as_deref(), Some("#tag"));
    }

    // ── Comment ──────────────────────────────────────────────────────────────

    #[test]
    fn comment_round_trip() {
        let comment = Comment {
            id: crate::CommentId::new("500"),
            message: "Moi".into(),
            from_user: sample_user(),
            created: UnixTimestamp(1500000000),
        };
        let json = serde_json::to_value(&comment).unwrap();
        let deserialized: Comment = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.message, "Moi");
    }

    // ── CommentList ──────────────────────────────────────────────────────────

    #[test]
    fn comment_list_round_trip() {
        let list = CommentList {
            movie_id: crate::MovieId::new("100"),
            all_count: 5,
            comments: vec![Comment {
                id: crate::CommentId::new("1"),
                message: "hi".into(),
                from_user: sample_user(),
                created: UnixTimestamp(1500000000),
            }],
        };
        let json = serde_json::to_value(&list).unwrap();
        let deserialized: CommentList = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.all_count, 5);
    }

    // ── PostedComment ────────────────────────────────────────────────────────

    #[test]
    fn posted_comment_round_trip() {
        let posted = PostedComment {
            movie_id: crate::MovieId::new("100"),
            all_count: 6,
            comment: Comment {
                id: crate::CommentId::new("2"),
                message: "hello".into(),
                from_user: sample_user(),
                created: UnixTimestamp(1500000001),
            },
        };
        let json = serde_json::to_value(&posted).unwrap();
        let deserialized: PostedComment = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.all_count, 6);
    }

    // ── DeletedComment ───────────────────────────────────────────────────────

    #[test]
    fn deleted_comment_round_trip() {
        let deleted = DeletedComment {
            comment_id: crate::CommentId::new("99"),
        };
        let json = serde_json::to_value(&deleted).unwrap();
        let deserialized: DeletedComment = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.comment_id.as_str(), "99");
    }

    // ── Gift ─────────────────────────────────────────────────────────────────

    #[test]
    fn gift_round_trip() {
        let gift = Gift {
            id: "100".into(),
            message: "Moi".into(),
            item_image: Url::parse("https://x.com/item.png").unwrap(),
            item_sub_image: None,
            item_id: "tea".into(),
            item_mp: "10".into(),
            item_name: "Tea".into(),
            user_image: Url::parse("https://x.com/user.png").unwrap(),
            user_screen_id: crate::ScreenId::new("caster"),
            user_screen_name: "caster".into(),
            user_name: "Caster".into(),
        };
        let json = serde_json::to_value(&gift).unwrap();
        let deserialized: Gift = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.id, "100");
    }

    // ── GiftList ─────────────────────────────────────────────────────────────

    #[test]
    fn gift_list_round_trip() {
        let list = GiftList {
            slice_id: 42,
            gifts: vec![],
        };
        let json = serde_json::to_value(&list).unwrap();
        let deserialized: GiftList = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.slice_id, 42);
        assert!(deserialized.gifts.is_empty());
    }

    // ── SupportingStatus ─────────────────────────────────────────────────────

    #[test]
    fn supporting_status_round_trip() {
        let status = SupportingStatus {
            is_supporting: true,
            supported: Some(UnixTimestamp(1600000000)),
            target_user: sample_user(),
        };
        let json = serde_json::to_value(&status).unwrap();
        let deserialized: SupportingStatus = serde_json::from_value(json).unwrap();
        assert!(deserialized.is_supporting);
        assert_eq!(deserialized.supported.unwrap().seconds(), 1600000000);
    }

    #[test]
    fn supporting_status_not_supporting_round_trip() {
        let status = SupportingStatus {
            is_supporting: false,
            supported: None,
            target_user: sample_user(),
        };
        let json = serde_json::to_value(&status).unwrap();
        // `supported` should be omitted when None (skip_serializing_if not used,
        // but #[serde(default)] handles deserialization)
        let deserialized: SupportingStatus = serde_json::from_value(json).unwrap();
        assert!(!deserialized.is_supporting);
        assert!(deserialized.supported.is_none());
    }

    // ── SupporterUser / SupportingList / SupporterList ───────────────────────

    #[allow(deprecated)]
    fn sample_supporter_user() -> SupporterUser {
        SupporterUser {
            id: crate::UserId::new("1"),
            screen_id: crate::ScreenId::new("supporter"),
            name: "Supporter".into(),
            image: Url::parse("https://x.com/i.png").unwrap(),
            profile: "".into(),
            level: 10,
            last_movie_id: Some(crate::MovieId::new("5")),
            is_live: false,
            supported: UnixTimestamp(1700000000),
            supporter_count: 0,
            supporting_count: 0,
            point: 100,
            total_point: 500,
            created: 0,
        }
    }

    #[test]
    fn supporter_user_round_trip() {
        let user = sample_supporter_user();
        let json = serde_json::to_value(&user).unwrap();
        let deserialized: SupporterUser = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.point, 100);
        assert_eq!(deserialized.total_point, 500);
    }

    #[test]
    fn supporting_list_round_trip() {
        let list = SupportingList {
            total: 1,
            supporting: vec![sample_supporter_user()],
        };
        let json = serde_json::to_value(&list).unwrap();
        let deserialized: SupportingList = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.total, 1);
    }

    #[test]
    fn supporter_list_round_trip() {
        let list = SupporterList {
            total: 2,
            supporters: vec![sample_supporter_user()],
        };
        let json = serde_json::to_value(&list).unwrap();
        let deserialized: SupporterList = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.total, 2);
    }

    // ── Category │ SubCategory │ CategoryList ────────────────────────────────

    #[test]
    fn category_list_round_trip() {
        let list = CategoryList {
            categories: vec![Category {
                id: "_channel".into(),
                name: "Channel".into(),
                sub_categories: vec![SubCategory {
                    id: "_sub_1".into(),
                    name: "Sub".into(),
                    count: 10,
                }],
            }],
        };
        let json = serde_json::to_value(&list).unwrap();
        let deserialized: CategoryList = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.categories[0].sub_categories[0].count, 10);
    }

    // ── UserSearchResults ────────────────────────────────────────────────────

    #[test]
    fn user_search_results_round_trip() {
        let results = UserSearchResults {
            users: vec![sample_user()],
        };
        let json = serde_json::to_value(&results).unwrap();
        let deserialized: UserSearchResults = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.users.len(), 1);
    }

    // ── Application ──────────────────────────────────────────────────────────

    #[test]
    fn application_round_trip() {
        let app = Application {
            client_id: "client123".into(),
            name: "MyApp".into(),
            owner_user_id: crate::UserId::new("42"),
        };
        let json = serde_json::to_value(&app).unwrap();
        let deserialized: Application = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.client_id, "client123");
    }

    // ── VerifiedCredentials ──────────────────────────────────────────────────

    #[test]
    fn verified_credentials_round_trip() {
        let creds = VerifiedCredentials {
            app: Application {
                client_id: "a1".into(),
                name: "App".into(),
                owner_user_id: crate::UserId::new("99"),
            },
            user: sample_user(),
            supporter_count: 5,
            supporting_count: 10,
        };
        let json = serde_json::to_value(&creds).unwrap();
        let deserialized: VerifiedCredentials = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.app.client_id, "a1");
    }

    // ── LiveSchedule ─────────────────────────────────────────────────────────

    #[test]
    fn live_schedule_round_trip() {
        let schedule = LiveSchedule {
            id: crate::LiveScheduleId::new("ts-1"),
            user_id: crate::UserId::new("1"),
            user_screen_id: crate::ScreenId::new("caster"),
            start_at: UnixTimestamp(1767193200),
            title: "配信予定".into(),
            thumbnail: Some(Url::parse("https://x.com/thumb.jpg").unwrap()),
        };
        let json = serde_json::to_value(&schedule).unwrap();
        let deserialized: LiveSchedule = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.title, "配信予定");
    }

    // ── LiveSchedules ────────────────────────────────────────────────────────

    #[test]
    fn live_schedules_round_trip() {
        let scheds = LiveSchedules {
            live_schedules: vec![],
        };
        let json = serde_json::to_value(&scheds).unwrap();
        let deserialized: LiveSchedules = serde_json::from_value(json).unwrap();
        assert!(deserialized.live_schedules.is_empty());
    }

    // ── Webhook / WebhookList ────────────────────────────────────────────────

    #[test]
    fn webhook_round_trip() {
        let hook = Webhook {
            user_id: crate::UserId::new("1"),
            event: WebhookEvent::LiveStart,
        };
        let json = serde_json::to_value(&hook).unwrap();
        let deserialized: Webhook = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.event, WebhookEvent::LiveStart);
    }

    #[test]
    fn webhook_list_round_trip() {
        let list = WebhookList {
            all_count: 1,
            webhooks: vec![Webhook {
                user_id: crate::UserId::new("1"),
                event: WebhookEvent::LiveEnd,
            }],
        };
        let json = serde_json::to_value(&list).unwrap();
        let deserialized: WebhookList = serde_json::from_value(json).unwrap();
        assert_eq!(deserialized.all_count, 1);
    }

    // ── RtmpCredentials ──────────────────────────────────────────────────────
    // Note: only Deserialize, not Serialize, so no round-trip test.

    // ── AccessToken ──────────────────────────────────────────────────────────
    // Note: only Deserialize, so deserialization is tested in fixtures.
}
