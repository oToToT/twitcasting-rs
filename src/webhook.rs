use serde_json::Value;

use crate::{LiveSchedule, LiveScheduleId, Movie, SecretString, User};

/// A decoded incoming webhook.
///
/// The `signature` is preserved but not verified because TwitCasting's public
/// documentation does not define a verification algorithm.
#[derive(Clone, Debug)]
pub enum WebhookPayload {
    /// A broadcast started.
    LiveStart {
        /// Opaque signature supplied by TwitCasting.
        signature: SecretString,
        /// Movie.
        movie: Movie,
        /// Broadcaster.
        broadcaster: User,
    },
    /// A broadcast ended.
    LiveEnd {
        /// Opaque signature supplied by TwitCasting.
        signature: SecretString,
        /// Movie.
        movie: Movie,
        /// Broadcaster.
        broadcaster: User,
    },
    /// A schedule was created.
    LiveScheduleCreate {
        /// Opaque signature supplied by TwitCasting.
        signature: SecretString,
        /// Schedule.
        live_schedule: LiveSchedule,
    },
    /// A schedule was updated.
    LiveScheduleUpdate {
        /// Opaque signature supplied by TwitCasting.
        signature: SecretString,
        /// Schedule.
        live_schedule: LiveSchedule,
    },
    /// A schedule was deleted.
    LiveScheduleDelete {
        /// Opaque signature supplied by TwitCasting.
        signature: SecretString,
        /// Deleted schedule ID.
        live_schedule_id: LiveScheduleId,
    },
    /// An event introduced after this crate version.
    Unknown {
        /// Unknown event key.
        event: String,
        /// Opaque signature, if supplied.
        signature: Option<SecretString>,
        /// Complete JSON body for forward-compatible handling.
        body: Value,
    },
}

/// Decodes an incoming TwitCasting webhook body.
pub fn decode_webhook(body: &[u8]) -> Result<WebhookPayload, serde_json::Error> {
    let value: Value = serde_json::from_slice(body)?;
    let event = value
        .get("event")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();

    match event.as_str() {
        "livestart" => {
            #[derive(serde::Deserialize)]
            struct Payload {
                signature: SecretString,
                movie: Movie,
                broadcaster: User,
            }
            let payload: Payload = serde_json::from_value(value)?;
            Ok(WebhookPayload::LiveStart {
                signature: payload.signature,
                movie: payload.movie,
                broadcaster: payload.broadcaster,
            })
        }
        "liveend" => {
            #[derive(serde::Deserialize)]
            struct Payload {
                signature: SecretString,
                movie: Movie,
                broadcaster: User,
            }
            let payload: Payload = serde_json::from_value(value)?;
            Ok(WebhookPayload::LiveEnd {
                signature: payload.signature,
                movie: payload.movie,
                broadcaster: payload.broadcaster,
            })
        }
        "liveschedulecreate" | "livescheduleupdate" => {
            #[derive(serde::Deserialize)]
            struct Payload {
                signature: SecretString,
                live_schedule: LiveSchedule,
            }
            let payload: Payload = serde_json::from_value(value)?;
            if event == "liveschedulecreate" {
                Ok(WebhookPayload::LiveScheduleCreate {
                    signature: payload.signature,
                    live_schedule: payload.live_schedule,
                })
            } else {
                Ok(WebhookPayload::LiveScheduleUpdate {
                    signature: payload.signature,
                    live_schedule: payload.live_schedule,
                })
            }
        }
        "livescheduledelete" => {
            #[derive(serde::Deserialize)]
            struct Payload {
                signature: SecretString,
                live_schedule_id: LiveScheduleId,
            }
            let payload: Payload = serde_json::from_value(value)?;
            Ok(WebhookPayload::LiveScheduleDelete {
                signature: payload.signature,
                live_schedule_id: payload.live_schedule_id,
            })
        }
        _ => {
            let signature = value
                .get("signature")
                .and_then(Value::as_str)
                .map(SecretString::new);
            Ok(WebhookPayload::Unknown {
                event,
                signature,
                body: value,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{WebhookPayload, decode_webhook};

    #[test]
    fn unknown_payload_is_preserved() {
        let body = br#"{"event":"future","signature":"opaque","new_field":42}"#;
        let payload = decode_webhook(body).unwrap();
        let WebhookPayload::Unknown { event, body, .. } = payload else {
            panic!("expected unknown payload");
        };
        assert_eq!(event, "future");
        assert_eq!(body["new_field"], 42);
    }

    #[test]
    fn live_start_payload() {
        // Use .as_bytes() to support non-ASCII characters like Japanese.
        let body = r#"{
            "event":"livestart",
            "signature":"sig123",
            "movie":{
                "id":"189037369","user_id":"182224938","title":"Live #189037369",
                "subtitle":"Now streaming","last_owner_comment":"hello everyone",
                "category":"girls_jcjk_jp",
                "link":"https://twitcasting.tv/twitcasting_jp/movie/189037369",
                "is_live":true,"is_recorded":false,"comment_count":2124,
                "large_thumbnail":"https://example.com/l.jpg",
                "small_thumbnail":"https://example.com/s.jpg",
                "country":"jp","duration":1186,"created":1438500282,
                "is_collabo":false,"is_protected":false,
                "max_view_count":0,"current_view_count":1,"total_view_count":1,
                "hls_url":"https://twitcasting.tv/twitcasting_jp/metastream.m3u8/?video=1"
            },
            "broadcaster":{
                "id":"182224938","screen_id":"twitcasting_jp","name":"Official",
                "image":"https://example.com/icon.png","profile":"Official account","level":24,
                "last_movie_id":"189037369","is_live":true,
                "supporter_count":0,"supporting_count":0,"created":0
            }
        }"#
        .as_bytes();
        let payload = decode_webhook(body).unwrap();
        let WebhookPayload::LiveStart {
            signature,
            movie,
            broadcaster,
        } = payload
        else {
            panic!("expected LiveStart");
        };
        assert_eq!(signature.expose_secret(), "sig123");
        assert_eq!(movie.id.as_str(), "189037369");
        assert_eq!(broadcaster.id.as_str(), "182224938");
    }

    #[test]
    fn live_end_payload() {
        let body = r#"{
            "event":"liveend",
            "signature":"sig456",
            "movie":{
                "id":"189037369","user_id":"182224938","title":"Live #189037369",
                "subtitle":null,"last_owner_comment":"bye",
                "category":null,
                "link":"https://twitcasting.tv/twitcasting_jp/movie/189037369",
                "is_live":false,"is_recorded":true,"comment_count":2124,
                "large_thumbnail":"https://example.com/l.jpg",
                "small_thumbnail":"https://example.com/s.jpg",
                "country":"jp","duration":1186,"created":1438500282,
                "is_collabo":false,"is_protected":false,
                "max_view_count":100,"current_view_count":0,"total_view_count":20848,
                "hls_url":null
            },
            "broadcaster":{
                "id":"182224938","screen_id":"twitcasting_jp","name":"Official",
                "image":"https://example.com/icon.png","profile":"Official","level":24,
                "last_movie_id":"189037369","is_live":false,
                "supporter_count":0,"supporting_count":0,"created":0
            }
        }"#
        .as_bytes();
        let payload = decode_webhook(body).unwrap();
        let WebhookPayload::LiveEnd {
            signature, movie, ..
        } = payload
        else {
            panic!("expected LiveEnd");
        };
        assert_eq!(signature.expose_secret(), "sig456");
        assert!(!movie.is_live);
    }

    #[test]
    fn live_schedule_create_payload() {
        let body = r#"{
            "event":"liveschedulecreate",
            "signature":"sig789",
            "live_schedule":{
                "id":"timetable-80959",
                "user_id":"1025221958827311105",
                "user_screen_id":"twitcasting_dev",
                "start_at":1767193200,
                "title":"Upcoming test stream",
                "thumbnail":"https://example.com/thumb.jpg"
            }
        }"#
        .as_bytes();
        let payload = decode_webhook(body).unwrap();
        let WebhookPayload::LiveScheduleCreate {
            signature,
            live_schedule,
        } = payload
        else {
            panic!("expected LiveScheduleCreate");
        };
        assert_eq!(signature.expose_secret(), "sig789");
        assert_eq!(live_schedule.id.as_str(), "timetable-80959");
        assert_eq!(live_schedule.title, "Upcoming test stream");
    }

    #[test]
    fn live_schedule_update_payload() {
        let body = r#"{
            "event":"livescheduleupdate",
            "signature":"sig-upd",
            "live_schedule":{
                "id":"timetable-80959",
                "user_id":"1025221958827311105",
                "user_screen_id":"twitcasting_dev",
                "start_at":1767193200,
                "title":"Updated title",
                "thumbnail":null
            }
        }"#
        .as_bytes();
        let payload = decode_webhook(body).unwrap();
        let WebhookPayload::LiveScheduleUpdate {
            signature,
            live_schedule,
        } = payload
        else {
            panic!("expected LiveScheduleUpdate");
        };
        assert_eq!(signature.expose_secret(), "sig-upd");
        assert_eq!(live_schedule.title, "Updated title");
        assert!(live_schedule.thumbnail.is_none());
    }

    #[test]
    fn live_schedule_delete_payload() {
        let body = br#"{
            "event":"livescheduledelete",
            "signature":"sig-del",
            "live_schedule_id":"timetable-80959"
        }"#;
        let payload = decode_webhook(body).unwrap();
        let WebhookPayload::LiveScheduleDelete {
            signature,
            live_schedule_id,
        } = payload
        else {
            panic!("expected LiveScheduleDelete");
        };
        assert_eq!(signature.expose_secret(), "sig-del");
        assert_eq!(live_schedule_id.as_str(), "timetable-80959");
    }

    #[test]
    fn webhook_without_signature_is_error() {
        let body = br#"{"event":"livescheduledelete","live_schedule_id":"ts-1"}"#;
        let result = decode_webhook(body);
        assert!(
            result.is_err(),
            "missing signature should be a decode error"
        );
    }

    #[test]
    fn webhook_unknown_event_preserves_body() {
        let body = br#"{"event":"custom_event","signature":"s","data":{"key":"val"}}"#;
        let payload = decode_webhook(body).unwrap();
        let WebhookPayload::Unknown {
            event,
            signature,
            body,
        } = payload
        else {
            panic!("expected Unknown");
        };
        assert_eq!(event, "custom_event");
        assert_eq!(signature.unwrap().expose_secret(), "s");
        assert_eq!(body["data"]["key"], "val");
    }

    #[test]
    fn webhook_unknown_event_without_signature() {
        let body = br#"{"event":"no_sig_event","extra":true}"#;
        let payload = decode_webhook(body).unwrap();
        let WebhookPayload::Unknown { signature, .. } = payload else {
            panic!("expected Unknown");
        };
        assert!(signature.is_none());
    }
}
