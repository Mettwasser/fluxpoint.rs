use chrono::{DateTime, Utc};

use super::macros::model;

model! {
    UserInfo: "/me";
    id: String,
    created: DateTime<Utc>
}
