use crate::structs::AppState::AppState;
use crate::structs::Incident::Incident;
use axum::extract::State;
use futures_util::TryStreamExt;
use sqlx::Postgres;
use sqlx::Row;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
static GOAL: usize = 100;

pub async fn send_incidents(State(state): State<AppState>) -> String {
    let mut out = state.out.lock().await;
    let mut rows = sqlx::query("SELECT * FROM incidents").fetch(&state.pool);
    while let Some(row) = rows.try_next().await.expect("Woops")
        && out.len() < GOAL
    {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let o = Incident {
            incident_id: row.try_get("incident_id").unwrap_or("".to_string()),
            title: row.try_get("title").unwrap_or("".to_string()),
            reporter: row.try_get("reporter").unwrap_or("".to_string()),
            lat: row.try_get("lat").unwrap_or(0.0),
            long: row.try_get("long").unwrap_or(0.0),
            description: row.try_get("description").unwrap_or(".".to_string()),
            time_unix: row.try_get("time_unix").unwrap_or(ts.try_into().unwrap()),
            address: row.try_get("address").unwrap_or("".to_string()),
            incident_type: row.try_get("incident_type").unwrap_or("".to_string()),
            child_of: Some(row.try_get("child_of").unwrap_or("".to_string())),
            level: row.try_get("level").unwrap_or(1),
        };
        if o.time_unix > (ts - 60 * 60 * 6).try_into().unwrap() {
            out.push(o);
        } else {
            let _ = sqlx::query("DELETE FROM incidents WHERE incident_id=$1")
                .bind(o.incident_id)
                .execute(&state.pool)
                .await;
        }
    }
    if out.len() >= GOAL {
        return serde_json::to_string(&*out).unwrap();
    }
    let missing = GOAL - out.len();
    if missing > 0 {
        let new: Vec<Incident> = crate::utils::create_incidents::create_incidents(
            (missing).try_into().unwrap(),
            &state.pool,
        )
        .await;
        for n in new {
            out.push(n.clone());
            sqlx::query("INSERT INTO incidents (incident_id, reporter, time_unix, lat, long, title, incident_type, child_of, description, level, address) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
                .bind(n.incident_id)
                .bind(n.reporter)
                .bind(n.time_unix)
                .bind(n.lat)
                .bind(n.long)
                .bind(n.title)
                .bind(n.incident_type)
                .bind(n.child_of)
                .bind(n.description)
                .bind(n.level)
                .bind(n.address)
                .execute(&state.pool)
                .await
                .expect("woops");
        }
    }
    return serde_json::to_string(&*out).unwrap();
}
