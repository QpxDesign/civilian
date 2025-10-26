use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    pub title: String,
    pub icon_slug: String,
}
