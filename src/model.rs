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

    use super::{TokenType, UnixTimestamp, WebhookEvent};

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
}
