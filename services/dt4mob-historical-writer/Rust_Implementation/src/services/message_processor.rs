use chrono::{DateTime, Utc};
use serde_json::Value;
use tracing::warn;

use crate::models::ditto_event::{Action, DittoEvent};

pub fn parse_message(msg: &Value) -> Result<DittoEvent, String> {
    let topic = msg
        .get("topic")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing 'topic' field".to_string())?;

    let topic_parts: Vec<&str> = topic.split('/').collect();
    if topic_parts.len() < 6 {
        return Err(format!("Invalid topic format: {}", topic));
    }

    let thing_id = format!("{}:{}", topic_parts[0], topic_parts[1]);

    let action_str = topic_parts[5];
    let action = Action::try_from(action_str)?;

    let path = msg
        .get("path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Missing 'path' field".to_string())?;

    let revision = msg.get("revision").and_then(|v| v.as_i64()).map(|v| v as i32);

    let value = msg.get("value").cloned();

    let headers = msg.get("headers").and_then(|v| v.as_object());

    let override_timestamp = headers
        .and_then(|h| h.get("dt4mob-historic-timestamp-override"))
        .and_then(|v| v.as_str());

    let event_time = if let Some(ts) = override_timestamp {
        let normalized = ts.replace("Z", "+00:00");
        match DateTime::parse_from_rfc3339(&normalized) {
            Ok(dt) => Some(dt.with_timezone(&Utc)),
            Err(_) => {
                match DateTime::parse_from_str(&normalized, "%+") {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(_) => {
                        warn!(
                            "Malformed historic override timestamp detected: '{}'. Falling back to message default timestamp.",
                            ts
                        );
                        None
                    }
                }
            }
        }
    } else {
        None
    };

    let event_time = event_time.unwrap_or_else(|| {
        msg.get("timestamp")
            .and_then(|v| v.as_str())
            .and_then(|ts| {
                let normalized = ts.replace("Z", "+00:00");
                DateTime::parse_from_rfc3339(&normalized)
                    .ok()
                    .map(|dt| dt.with_timezone(&Utc))
            })
            .unwrap_or_else(|| Utc::now())
    });

    Ok(DittoEvent {
        time: event_time,
        thing_id,
        action,
        path: path.to_string(),
        revision,
        value: if value.as_ref().is_some_and(|v| v.is_null()) {
            None
        } else {
            value
        },
    })
}
