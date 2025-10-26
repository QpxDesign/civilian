use futures::stream::{self, StreamExt};
use futures::FutureExt;
use geo::{Contains, Point, Polygon};
use geojson::Feature;
use google_maps::prelude::*;
use indicatif::ProgressBar;
use lazy_static::lazy_static;
use rand::Rng;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::fs;
use std::io::Read;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref GOOGLE_MAPS_CLIENT: GoogleMapsClient =
        google_maps::Client::try_new(&env::vars().find(|v| v.0 == "GOOGLE_MAPS_API").unwrap().1)
            .unwrap();
}

fn dist(x_1: f64, y_1: f64, x_2: f64, y_2: f64) -> f64 {
    return ((x_2 - x_1).powf(2.0) + (y_2 - y_1).powf(2.0)).sqrt();
}

pub fn file_to_geojson(path: &str) -> Polygon {
    let contents = file_to_string(path);
    let geojson = contents.parse::<Feature>().unwrap();
    let polygon: Polygon = geojson.geometry.unwrap().try_into().unwrap();
    return polygon;
}

pub fn file_to_string(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut contents: String = "".to_string();
    let _ = file.read_to_string(&mut contents);
    return contents;
}

#[derive(Clone)]
struct Location {
    index: i32,
    lat: f32,
    long: f32,
    address: String,
}

static MAX_ITEMS: usize = 1000;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::vars().find(|v| v.0 == "DATABASE_URL").unwrap().1)
        .await
        .unwrap();

    let mut map_data: Vec<Polygon> = Vec::new();
    map_data.push(file_to_geojson("./assets/LA_1.geojson"));
    map_data.push(file_to_geojson("./assets/LA_2.geojson"));
    let mut points: Vec<Point> = Vec::new();
    println!("Finding Points:");
    let points_bar = ProgressBar::new(MAX_ITEMS.try_into().unwrap());
    let mut index = 0;
    let mut output: Vec<Location> = Vec::new();
    while points.len() < MAX_ITEMS {
        let lon = rand::rng().random_range(33.727868427886385..34.31995588728928); // 34.31995588728928, -118.47737352341
        let lat = rand::rng().random_range(-118.599659..-117.973890);
        let p = Point::new(lat, lon);
        for polygon in &map_data {
            if polygon.contains(&p)
            //    && points
            //.iter()
            //.find(|o| dist(p.x(), p.y(), o.x(), o.y()) < 0.015)
            // .is_none()
            {
                let x = geolocate(p.y() as f32, p.x() as f32).await;

                let _ = sqlx::query(
                    "INSERT INTO locations (index, lat, long, address) VALUES ($1, $2, $3, $4)",
                )
                .bind(index)
                .bind(x.0)
                .bind(x.1)
                .bind(x.2)
                .execute(&pool)
                .await;

                index += 1;
                points.push(p);
                points_bar.inc(1);
                break;
            }
        }
    }
}

async fn geolocate(lat: f32, long: f32) -> (f32, f32, String) {
    let mut out = (0.0, 0.0, "".to_string());
    let addr: String = GOOGLE_MAPS_CLIENT
        .reverse_geocoding(LatLng {
            lat: lat.try_into().unwrap(),
            lng: long.try_into().unwrap(),
        })
        .execute()
        .await
        .unwrap()
        .results[0]
        .formatted_address
        .clone();
    let rev_addr = GOOGLE_MAPS_CLIENT
        .geocoding()
        .with_address(&addr)
        .execute()
        .await
        .unwrap()
        .results[0]
        .geometry
        .location;
    out.0 = rev_addr.lat.try_into().unwrap();
    out.1 = rev_addr.lng.try_into().unwrap();
    out.2 = addr;
    return out;
}
