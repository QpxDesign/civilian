use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Incident {
    pub incident_id: String,
    pub title: String,
    pub reporter: String,
    pub time_unix: i64,
    pub lat: f32,
    pub long: f32,
    pub address: String,
    pub incident_type: String,
    pub child_of: Option<String>,
    pub description: String,
    pub level: i32,
}
