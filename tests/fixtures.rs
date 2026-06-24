//! Official-shape JSON fixture tests.

use twitcasting::{GiftList, MovieInfo, UserInfo};

#[test]
fn user_fixture_accepts_deprecated_and_unknown_fields() {
    let fixture = r#"{
        "user": {
            "id": "182224938",
            "screen_id": "twitcasting_jp",
            "name": "TwitCasting",
            "image": "https://example.com/icon.png",
            "profile": "official",
            "level": 24,
            "last_movie_id": null,
            "is_live": false,
            "supporter_count": 0,
            "supporting_count": 0,
            "created": 0,
            "future_field": true
        },
        "supporter_count": 10,
        "supporting_count": 24
    }"#;
    let response: UserInfo = serde_json::from_str(fixture).unwrap();
    assert_eq!(response.user.id.as_str(), "182224938");
    assert!(response.user.last_movie_id.is_none());
}

#[test]
fn movie_fixture_accepts_nullable_fields() {
    let fixture = r#"{
        "movie": {
            "id": "189037369",
            "user_id": "182224938",
            "title": "Live",
            "subtitle": null,
            "last_owner_comment": null,
            "category": null,
            "link": "https://twitcasting.tv/u/movie/189037369",
            "is_live": false,
            "is_recorded": false,
            "comment_count": 0,
            "large_thumbnail": "https://example.com/large.jpg",
            "small_thumbnail": "https://example.com/small.jpg",
            "country": "jp",
            "duration": 1,
            "created": 1438500282,
            "is_collabo": false,
            "is_protected": false,
            "max_view_count": 0,
            "current_view_count": 0,
            "total_view_count": 0,
            "hls_url": null
        },
        "broadcaster": {
            "id": "182224938",
            "screen_id": "twitcasting_jp",
            "name": "TwitCasting",
            "image": "https://example.com/icon.png",
            "profile": "",
            "level": 24,
            "last_movie_id": "189037369",
            "is_live": false
        },
        "tags": []
    }"#;
    let response: MovieInfo = serde_json::from_str(fixture).unwrap();
    assert!(response.movie.hls_url.is_none());
    assert!(response.movie.subtitle.is_none());
}

#[test]
fn gift_id_accepts_documented_string_and_numeric_wire_forms() {
    for id in [r#""2125""#, "2125"] {
        let fixture = format!(
            r#"{{
                "slice_id": 2125,
                "gifts": [{{
                    "id": {id},
                    "message": "Moi",
                    "item_image": "https://example.com/item.png",
                    "item_sub_image": null,
                    "item_id": "tea",
                    "item_mp": "10",
                    "item_name": "Tea",
                    "user_image": "https://example.com/user.png",
                    "user_screen_id": "caster",
                    "user_screen_name": "caster",
                    "user_name": "Caster"
                }}]
            }}"#
        );
        let response: GiftList = serde_json::from_str(&fixture).unwrap();
        assert_eq!(response.gifts[0].id, "2125");
    }
}
