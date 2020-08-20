use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub id: Uuid,
    pub tag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
