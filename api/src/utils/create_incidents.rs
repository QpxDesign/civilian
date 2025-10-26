use crate::structs::Event::Event;
use crate::structs::Incident::Incident;
use crate::utils::file_handler;
use futures_util::TryStreamExt;
use rand::Rng;
use sqlx::Pool;
use sqlx::Postgres;
use sqlx::Row;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

fn dist(x_1: f32, y_1: f32, x_2: f32, y_2: f32) -> f32 {
    return ((x_2 - x_1).powf(2.0) + (y_2 - y_1).powf(2.0)).sqrt();
}

pub async fn create_incidents(n: i64, pool: &Pool<Postgres>) -> Vec<Incident> {
    let _ = dotenvy::dotenv();

    let events: Vec<Event> =
        serde_json::from_str(&file_handler::file_to_string("./assets/Events.json")).unwrap();

    let mut output: Vec<Incident> = Vec::new();
    let mut rows = sqlx::query("SELECT * FROM locations ORDER BY RANDOM() LIMIT 100").fetch(pool);
    while let Some(row) = rows.try_next().await.expect("Woops") {
        let lat: f32 = row.try_get("lat").unwrap_or(0.0);
        let long: f32 = row.try_get("long").unwrap_or(0.0);
        let address: String = row.try_get("address").unwrap_or("".to_string());

        if !output
            .iter()
            .find(|o| dist(lat, long, o.lat, o.long) < 0.015)
            .is_none()
        {
            continue;
        }
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let e: Event = events[rand::rng().random_range(0..events.len())].clone();
        let o = Incident {
            incident_id: Uuid::new_v4().to_string(),
            title: e.title,
            reporter: "".to_string(),
            description: "".to_string(),
            time_unix: rand::rng()
                .random_range((ts - 60 * 60 * 6)..(ts - 100))
                .try_into()
                .unwrap(),
            lat: lat,
            long: long,
            incident_type: e.icon_slug,
            child_of: None,
            level: rand::rng().random_range(0..=5),
            address: address,
        };
        output.push(o);
    }
    println!("{}", output.len());
    return output;
}
