use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    id: i32,
    author_id: i32,
    content: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}
