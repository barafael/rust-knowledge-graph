//! A normal struct has members of any other type. It's somewhat like a POD in C++, or a struct in C.

use chrono::{DateTime, Utc};

pub struct Message {
    pub priority: u32,
    pub to: String,
    pub from: String,
    pub content: String,
    pub date: DateTime<Utc>,
}
