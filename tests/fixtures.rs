//! Official-shape JSON fixture tests for every API response type.
//!
//! JSON is transcribed from <https://apiv2-doc.twitcasting.tv/>.

use twitcasting::*;

// ── User ─────────────────────────────────────────────────────────────────────

#[test]
fn user_info_with_all_fields() {
    let json = r#"{
        "user":{
            "id":"182224938",
            "screen_id":"twitcasting_jp",
            "name":"ツイキャス公式",
            "image":"https://example.com/icon.png",
            "profile":"公式アカウントです。",
            "level":24,
            "last_movie_id":"189037369",
            "is_live":false,
            "supporter_count":0,
            "supporting_count":0,
            "created":0
        },
        "supporter_count":10,
        "supporting_count":24
    }"#;
    let info: UserInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.user.id.as_str(), "182224938");
    assert_eq!(info.user.screen_id.as_str(), "twitcasting_jp");
    assert_eq!(info.user.name, "ツイキャス公式");
    assert_eq!(info.user.level, 24);
    assert!(info.user.last_movie_id.is_some());
    assert!(!info.user.is_live);
    assert_eq!(info.supporter_count, 10);
    assert_eq!(info.supporting_count, 24);
}

#[test]
fn user_info_with_null_last_movie() {
    let json = r#"{
        "user":{
            "id":"1","screen_id":"test","name":"T","image":"https://x.com/i.png",
            "profile":"","level":1,"last_movie_id":null,"is_live":true,
            "supporter_count":0,"supporting_count":0,"created":0
        },
        "supporter_count":0,"supporting_count":0
    }"#;
    let info: UserInfo = serde_json::from_str(json).unwrap();
    assert!(info.user.last_movie_id.is_none());
    assert!(info.user.is_live);
}

#[test]
fn user_info_accepts_deprecated_and_unknown_fields() {
    let json = r#"{
        "user":{
            "id":"182224938","screen_id":"twitcasting_jp","name":"TwitCasting",
            "image":"https://example.com/icon.png","profile":"official","level":24,
            "last_movie_id":null,"is_live":false,
            "supporter_count":0,"supporting_count":0,"created":0,
            "future_field":true
        },
        "supporter_count":10,"supporting_count":24
    }"#;
    let info: UserInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.user.id.as_str(), "182224938");
}

// ── Verify Credentials ────────────────────────────────────────────────────────

#[test]
fn verified_credentials() {
    let json = r#"{
        "app":{
            "client_id":"182224938.d37f58350925d568e2db24719fe86f11c4d14e0461429e8b5da732fcb1917b6e",
            "name":"サンプルアプリケーション",
            "owner_user_id":"182224938"
        },
        "user":{
            "id":"182224938","screen_id":"twitcasting_jp","name":"ツイキャス公式",
            "image":"https://example.com/icon.png","profile":"公式","level":24,
            "last_movie_id":"189037369","is_live":false,
            "supporter_count":0,"supporting_count":0,"created":0
        },
        "supporter_count":10,"supporting_count":24
    }"#;
    let creds: VerifiedCredentials = serde_json::from_str(json).unwrap();
    assert_eq!(
        creds.app.client_id,
        "182224938.d37f58350925d568e2db24719fe86f11c4d14e0461429e8b5da732fcb1917b6e"
    );
    assert_eq!(creds.app.owner_user_id.as_str(), "182224938");
    assert_eq!(creds.supporter_count, 10);
}

// ── Live Schedule ────────────────────────────────────────────────────────────

#[test]
fn live_schedules() {
    let json = r#"{
        "live_schedules":[
            {
                "id":"timetable-80959",
                "user_id":"1025221958827311105",
                "user_screen_id":"twitcasting_dev",
                "start_at":1767193200,
                "title":"配信予定のテストです",
                "thumbnail":"https://example.com/thumb.jpg"
            }
        ]
    }"#;
    let schedules: LiveSchedules = serde_json::from_str(json).unwrap();
    assert_eq!(schedules.live_schedules.len(), 1);
    let s = &schedules.live_schedules[0];
    assert_eq!(s.id.as_str(), "timetable-80959");
    assert_eq!(s.start_at.seconds(), 1767193200);
    assert_eq!(s.title, "配信予定のテストです");
    assert!(s.thumbnail.is_some());
}

#[test]
fn live_schedules_null_thumbnail() {
    let json = r#"{
        "live_schedules":[
            {
                "id":"timetable-1","user_id":"1","user_screen_id":"dev",
                "start_at":1000000000,"title":"Test","thumbnail":null
            }
        ]
    }"#;
    let schedules: LiveSchedules = serde_json::from_str(json).unwrap();
    assert!(schedules.live_schedules[0].thumbnail.is_none());
}

#[test]
fn live_schedules_empty() {
    let json = r#"{"live_schedules":[]}"#;
    let schedules: LiveSchedules = serde_json::from_str(json).unwrap();
    assert!(schedules.live_schedules.is_empty());
}

// ── Movie ────────────────────────────────────────────────────────────────────

#[test]
fn movie_info_with_all_fields() {
    let json = r#"{
        "movie":{
            "id":"189037369",
            "user_id":"182224938",
            "title":"ライブ #189037369",
            "subtitle":"ライブ配信中！",
            "last_owner_comment":"もいもい",
            "category":"girls_jcjk_jp",
            "link":"https://twitcasting.tv/twitcasting_jp/movie/189037369",
            "is_live":false,
            "is_recorded":false,
            "comment_count":2124,
            "large_thumbnail":"https://example.com/large.jpg",
            "small_thumbnail":"https://example.com/small.jpg",
            "country":"jp",
            "duration":1186,
            "created":1438500282,
            "is_collabo":false,
            "is_protected":false,
            "max_view_count":1675,
            "current_view_count":20848,
            "total_view_count":20848,
            "hls_url":"https://twitcasting.tv/twitcasting_jp/metastream.m3u8/?video=1"
        },
        "broadcaster":{
            "id":"182224938","screen_id":"twitcasting_jp","name":"ツイキャス公式",
            "image":"https://example.com/icon.png","profile":"公式","level":24,
            "last_movie_id":"189037369","is_live":true,
            "supporter_count":0,"supporting_count":0,"created":0
        },
        "tags":["人気","雑談"]
    }"#;
    let info: MovieInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.movie.id.as_str(), "189037369");
    assert_eq!(info.movie.subtitle.as_deref(), Some("ライブ配信中！"));
    assert_eq!(info.movie.category.as_deref(), Some("girls_jcjk_jp"));
    assert!(info.movie.hls_url.is_some());
    assert_eq!(info.broadcaster.id.as_str(), "182224938");
    assert_eq!(info.tags, vec!["人気", "雑談"]);
}

#[test]
fn movie_info_nullable_fields() {
    let json = r#"{
        "movie":{
            "id":"189037369","user_id":"182224938","title":"Live",
            "subtitle":null,"last_owner_comment":null,"category":null,
            "link":"https://twitcasting.tv/u/movie/189037369",
            "is_live":false,"is_recorded":false,"comment_count":0,
            "large_thumbnail":"https://example.com/large.jpg",
            "small_thumbnail":"https://example.com/small.jpg",
            "country":"jp","duration":1,"created":1438500282,
            "is_collabo":false,"is_protected":false,
            "max_view_count":0,"current_view_count":0,"total_view_count":0,
            "hls_url":null
        },
        "broadcaster":{
            "id":"182224938","screen_id":"twitcasting_jp","name":"TwitCasting",
            "image":"https://example.com/icon.png","profile":"","level":24,
            "last_movie_id":"189037369","is_live":false
        },
        "tags":[]
    }"#;
    let info: MovieInfo = serde_json::from_str(json).unwrap();
    assert!(info.movie.hls_url.is_none());
    assert!(info.movie.subtitle.is_none());
    assert!(info.movie.last_owner_comment.is_none());
    assert!(info.movie.category.is_none());
    assert!(info.tags.is_empty());
}

#[test]
fn movie_info_live_broadcaster_fields() {
    let json = r#"{
        "movie":{
            "id":"1","user_id":"1","title":"Live",
            "subtitle":null,"last_owner_comment":null,"category":null,
            "link":"https://x.com/m/1","is_live":true,"is_recorded":false,
            "comment_count":0,
            "large_thumbnail":"https://x.com/l.jpg","small_thumbnail":"https://x.com/s.jpg",
            "country":"jp","duration":0,"created":1000000000,
            "is_collabo":false,"is_protected":false,
            "max_view_count":0,"current_view_count":1,"total_view_count":1,
            "hls_url":"https://x.com/stream.m3u8"
        },
        "broadcaster":{
            "id":"1","screen_id":"caster","name":"Caster",
            "image":"https://x.com/i.png","profile":"Hi","level":10,
            "last_movie_id":"1","is_live":true
        },
        "tags":["tag"]
    }"#;
    let info: MovieInfo = serde_json::from_str(json).unwrap();
    assert!(info.movie.is_live);
    assert_eq!(info.movie.current_view_count, 1);
    assert!(info.broadcaster.is_live);
}

// ── Movie List ────────────────────────────────────────────────────────────────

#[test]
fn movie_list() {
    let json = r#"{
        "total_count":5,
        "movies":[
            {
                "id":"323387579","user_id":"2880417757","title":"ライブ #323387579",
                "subtitle":"ライブ配信中！","last_owner_comment":"こんにちは",
                "category":"girls_jcjk_jp",
                "link":"https://twitcasting.tv/twitcasting_pr/movie/323387579",
                "is_live":false,"is_recorded":false,"comment_count":64,
                "large_thumbnail":"https://example.com/l.jpg",
                "small_thumbnail":"https://example.com/s.jpg",
                "country":"jp","duration":995,"created":1479379075,
                "is_collabo":false,"is_protected":false,
                "max_view_count":22,"current_view_count":71,"total_view_count":71,
                "hls_url":"https://twitcasting.tv/twitcasting_pr/metastream.m3u8/?video=1"
            }
        ]
    }"#;
    let list: MovieList = serde_json::from_str(json).unwrap();
    assert_eq!(list.total_count, 5);
    assert_eq!(list.movies.len(), 1);
    assert_eq!(list.movies[0].id.as_str(), "323387579");
    assert_eq!(list.movies[0].comment_count, 64);
}

#[test]
fn movie_list_empty() {
    let json = r#"{"total_count":0,"movies":[]}"#;
    let list: MovieList = serde_json::from_str(json).unwrap();
    assert_eq!(list.total_count, 0);
    assert!(list.movies.is_empty());
}

// ── Subtitle & Hashtag ────────────────────────────────────────────────────────

#[test]
fn subtitle_update_set() {
    let json = r#"{"movie_id":"323387579","subtitle":"初見さん大歓迎！"}"#;
    let update: SubtitleUpdate = serde_json::from_str(json).unwrap();
    assert_eq!(update.movie_id.as_str(), "323387579");
    assert_eq!(update.subtitle.as_deref(), Some("初見さん大歓迎！"));
}

#[test]
fn subtitle_update_unset() {
    let json = r#"{"movie_id":"323387579","subtitle":null}"#;
    let update: SubtitleUpdate = serde_json::from_str(json).unwrap();
    assert!(update.subtitle.is_none());
}

#[test]
fn hashtag_update_set() {
    let json = "{\"movie_id\":\"323387579\",\"hashtag\":\"#初見さん大歓迎\"}";
    let update: HashtagUpdate = serde_json::from_str(json).unwrap();
    assert_eq!(update.hashtag.as_deref(), Some("#初見さん大歓迎"));
}

#[test]
fn hashtag_update_unset() {
    let json = r#"{"movie_id":"323387579","hashtag":null}"#;
    let update: HashtagUpdate = serde_json::from_str(json).unwrap();
    assert!(update.hashtag.is_none());
}

// ── Comment ───────────────────────────────────────────────────────────────────

#[test]
fn comment_list() {
    let json = r#"{
        "movie_id":"189037369",
        "all_count":2124,
        "comments":[
            {
                "id":"7134775954",
                "message":"モイ！",
                "from_user":{
                    "id":"182224938","screen_id":"twitcasting_jp","name":"ツイキャス公式",
                    "image":"https://example.com/icon.png","profile":"公式","level":24,
                    "last_movie_id":"189037369","is_live":false,
                    "supporter_count":0,"supporting_count":0,"created":0
                },
                "created":1479579471
            }
        ]
    }"#;
    let list: CommentList = serde_json::from_str(json).unwrap();
    assert_eq!(list.movie_id.as_str(), "189037369");
    assert_eq!(list.all_count, 2124);
    assert_eq!(list.comments.len(), 1);
    assert_eq!(list.comments[0].id.as_str(), "7134775954");
    assert_eq!(list.comments[0].message, "モイ！");
}

#[test]
fn posted_comment() {
    let json = r#"{
        "movie_id":"189037369",
        "all_count":2125,
        "comment":{
            "id":"7134775954","message":"モイ！",
            "from_user":{
                "id":"182224938","screen_id":"twitcasting_jp","name":"ツイキャス公式",
                "image":"https://example.com/icon.png","profile":"公式","level":24,
                "last_movie_id":"189037369","is_live":false,
                "supporter_count":0,"supporting_count":0,"created":0
            },
            "created":1479579471
        }
    }"#;
    let posted: PostedComment = serde_json::from_str(json).unwrap();
    assert_eq!(posted.all_count, 2125);
    assert_eq!(posted.comment.message, "モイ！");
}

#[test]
fn deleted_comment() {
    let json = r#"{"comment_id":"123456"}"#;
    let deleted: DeletedComment = serde_json::from_str(json).unwrap();
    assert_eq!(deleted.comment_id.as_str(), "123456");
}

// ── Gift ──────────────────────────────────────────────────────────────────────

#[test]
fn gift_list_with_string_id() {
    let json = r#"{
        "slice_id":2124,
        "gifts":[{
            "id":"2125","message":"モイ！",
            "item_image":"https://twitcasting.tv/img/item_tea.png",
            "item_sub_image":null,
            "item_id":"tea","item_mp":"10","item_name":"お茶",
            "user_image":"https://example.com/user.png",
            "user_screen_id":"twitcasting_jp",
            "user_screen_name":"twitcasting_jp",
            "user_name":"ツイキャス公式"
        }]
    }"#;
    let list: GiftList = serde_json::from_str(json).unwrap();
    assert_eq!(list.slice_id, 2124);
    assert_eq!(list.gifts[0].id, "2125");
    assert_eq!(list.gifts[0].item_id, "tea");
    assert!(list.gifts[0].item_sub_image.is_none());
}

#[test]
fn gift_id_accepts_documented_string_and_numeric_wire_forms() {
    for id in [r#""2125""#, "2125"] {
        let json = format!(
            r#"{{"slice_id":2125,"gifts":[{{"id":{},"message":"Moi","item_image":"https://x.com/item.png","item_sub_image":null,"item_id":"tea","item_mp":"10","item_name":"Tea","user_image":"https://x.com/user.png","user_screen_id":"caster","user_screen_name":"caster","user_name":"Caster"}}]}}"#,
            id
        );
        let list: GiftList = serde_json::from_str(&json).unwrap();
        assert_eq!(list.gifts[0].id, "2125");
    }
}

#[test]
fn gift_list_empty() {
    let json = r#"{"slice_id":-1,"gifts":[]}"#;
    let list: GiftList = serde_json::from_str(json).unwrap();
    assert_eq!(list.slice_id, -1);
    assert!(list.gifts.is_empty());
}

// ── Supporter ─────────────────────────────────────────────────────────────────

#[test]
fn supporting_status_is_supporting() {
    let json = r#"{
        "is_supporting":true,
        "supported":1700000000,
        "target_user":{
            "id":"182224938","screen_id":"twitcasting_jp","name":"ツイキャス公式",
            "image":"https://example.com/icon.png","profile":"公式","level":24,
            "last_movie_id":"189037369","is_live":false,
            "supporter_count":0,"supporting_count":0,"created":0
        }
    }"#;
    let status: SupportingStatus = serde_json::from_str(json).unwrap();
    assert!(status.is_supporting);
    assert_eq!(status.supported.unwrap().seconds(), 1700000000);
}

#[test]
fn supporting_status_not_supporting() {
    let json = r#"{
        "is_supporting":false,
        "target_user":{
            "id":"182224938","screen_id":"twitcasting_jp","name":"T",
            "image":"https://x.com/i.png","profile":"","level":1,
            "last_movie_id":null,"is_live":false,
            "supporter_count":0,"supporting_count":0,"created":0
        }
    }"#;
    let status: SupportingStatus = serde_json::from_str(json).unwrap();
    assert!(!status.is_supporting);
    assert!(status.supported.is_none());
}

#[test]
fn support_result() {
    let json = r#"{"added_count":3}"#;
    let result: SupportResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.added_count, 3);
}

#[test]
fn unsupport_result() {
    let json = r#"{"removed_count":1}"#;
    let result: UnsupportResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.removed_count, 1);
}

#[test]
fn supporting_list() {
    let json = r#"{
        "total":2,
        "supporting":[
            {
                "id":"182224938","screen_id":"twitcasting_jp","name":"ツイキャス公式",
                "image":"https://example.com/icon.png","profile":"公式","level":24,
                "last_movie_id":"189037369","is_live":false,
                "supported":1700000000,
                "point":100,"total_point":500
            }
        ]
    }"#;
    let list: SupportingList = serde_json::from_str(json).unwrap();
    assert_eq!(list.total, 2);
    assert_eq!(list.supporting.len(), 1);
    assert_eq!(list.supporting[0].point, 100);
    assert_eq!(list.supporting[0].total_point, 500);
}

#[test]
fn supporter_list() {
    let json = r#"{
        "total":1,
        "supporters":[
            {
                "id":"182224938","screen_id":"twitcasting_jp","name":"ツイキャス公式",
                "image":"https://example.com/icon.png","profile":"公式","level":24,
                "last_movie_id":"189037369","is_live":false,
                "supported":1700000000,
                "point":250,"total_point":1000
            }
        ]
    }"#;
    let list: SupporterList = serde_json::from_str(json).unwrap();
    assert_eq!(list.total, 1);
    assert_eq!(list.supporters[0].point, 250);
}

// ── Category ──────────────────────────────────────────────────────────────────

#[test]
fn category_list() {
    let json = r#"{
        "categories":[
            {
                "id":"_channel",
                "name":"チャンネル",
                "sub_categories":[
                    {"id":"_system_channel_5","name":"ミュージックch","count":100},
                    {"id":"_system_channel_6","name":"ママch","count":49}
                ]
            }
        ]
    }"#;
    let list: CategoryList = serde_json::from_str(json).unwrap();
    assert_eq!(list.categories.len(), 1);
    assert_eq!(list.categories[0].id, "_channel");
    assert_eq!(list.categories[0].sub_categories.len(), 2);
    assert_eq!(list.categories[0].sub_categories[0].count, 100);
}

#[test]
fn category_list_empty() {
    let json = r#"{"categories":[]}"#;
    let list: CategoryList = serde_json::from_str(json).unwrap();
    assert!(list.categories.is_empty());
}

// ── Search ────────────────────────────────────────────────────────────────────

#[test]
fn user_search_results() {
    let json = r#"{
        "users":[
            {
                "id":"182224938","screen_id":"twitcasting_jp","name":"ツイキャス公式",
                "image":"https://example.com/icon.png","profile":"公式","level":24,
                "last_movie_id":"189037369","is_live":false,
                "supporter_count":0,"supporting_count":0,"created":0
            }
        ]
    }"#;
    let results: UserSearchResults = serde_json::from_str(json).unwrap();
    assert_eq!(results.users.len(), 1);
    assert_eq!(results.users[0].screen_id.as_str(), "twitcasting_jp");
}

#[test]
fn live_search_results() {
    let json = r#"{
        "movies":[
            {
                "movie":{
                    "id":"1","user_id":"1","title":"Live",
                    "subtitle":null,"last_owner_comment":null,"category":null,
                    "link":"https://x.com/m/1","is_live":true,"is_recorded":false,
                    "comment_count":5,
                    "large_thumbnail":"https://x.com/l.jpg","small_thumbnail":"https://x.com/s.jpg",
                    "country":"jp","duration":0,"created":1000000000,
                    "is_collabo":false,"is_protected":false,
                    "max_view_count":0,"current_view_count":10,"total_view_count":10,
                    "hls_url":null
                },
                "broadcaster":{
                    "id":"1","screen_id":"caster","name":"Caster",
                    "image":"https://x.com/i.png","profile":"Hi","level":10,
                    "last_movie_id":"1","is_live":true
                },
                "tags":["人気","雑談"]
            }
        ]
    }"#;
    let results: LiveSearchResults = serde_json::from_str(json).unwrap();
    assert_eq!(results.movies.len(), 1);
    assert_eq!(results.movies[0].tags, vec!["人気", "雑談"]);
}

// ── WebHook ───────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "webhooks")]
fn webhook_list() {
    let json = r#"{
        "all_count":2,
        "webhooks":[
            {"user_id":"182224938","event":"livestart"},
            {"user_id":"182224938","event":"liveend"}
        ]
    }"#;
    let list: WebhookList = serde_json::from_str(json).unwrap();
    assert_eq!(list.all_count, 2);
    assert_eq!(list.webhooks[0].event, WebhookEvent::LiveStart);
    assert_eq!(list.webhooks[1].event, WebhookEvent::LiveEnd);
}

#[test]
#[cfg(feature = "webhooks")]
fn added_webhooks() {
    let json = r#"{"user_id":"182224938","added_events":["livestart","liveend"]}"#;
    let added: AddedWebhooks = serde_json::from_str(json).unwrap();
    assert_eq!(added.user_id.as_str(), "182224938");
    assert_eq!(
        added.added_events,
        vec![WebhookEvent::LiveStart, WebhookEvent::LiveEnd]
    );
}

#[test]
#[cfg(feature = "webhooks")]
fn deleted_webhooks() {
    let json = r#"{"user_id":"182224938","deleted_events":["livescheduledelete"]}"#;
    let deleted: DeletedWebhooks = serde_json::from_str(json).unwrap();
    assert_eq!(
        deleted.deleted_events,
        vec![WebhookEvent::LiveScheduleDelete]
    );
}

// ── Broadcasting ──────────────────────────────────────────────────────────────

#[test]
fn rtmp_credentials_enabled() {
    let json = r#"{
        "enabled":true,
        "url":"rtmp://rtmp02.twitcasting.tv/...",
        "stream_key":"twitcasting_jp"
    }"#;
    let creds: RtmpCredentials = serde_json::from_str(json).unwrap();
    assert!(creds.enabled);
    assert!(creds.url.is_some());
    assert!(creds.stream_key.is_some());
}

#[test]
fn rtmp_credentials_disabled() {
    let json = r#"{"enabled":false,"url":null,"stream_key":null}"#;
    let creds: RtmpCredentials = serde_json::from_str(json).unwrap();
    assert!(!creds.enabled);
    assert!(creds.url.is_none());
    assert!(creds.stream_key.is_none());
}

// ── Access Token ──────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "oauth")]
fn access_token() {
    let json = r#"{
        "token_type":"Bearer",
        "expires_in":15552000,
        "access_token":"ACCESS_TOKEN_VALUE"
    }"#;
    let token: AccessToken = serde_json::from_str(json).unwrap();
    assert_eq!(token.token_type, TokenType::Bearer);
    assert_eq!(token.expires_in, 15552000);
    assert_eq!(token.access_token.expose_secret(), "ACCESS_TOKEN_VALUE");
}
