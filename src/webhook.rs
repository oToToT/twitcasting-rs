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
}
