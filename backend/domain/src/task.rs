use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub done: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
