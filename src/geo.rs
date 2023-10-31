use crate::config::Location;
use std::time::{SystemTime, UNIX_EPOCH};

fn calculate_distance_km(start: &Location, end: &Location) -> f64 {
    let lat1 = start.lat.to_radians();
    let lat2 = end.lat.to_radians();
    let delta_lat = (end.lat - start.lat).to_radians();
    let delta_lng = (end.lng - start.lng).to_radians();

    let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin() +
        lat1.cos() * lat2.cos() *
        (delta_lng / 2.0).sin() * (delta_lng / 2.0).sin();
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    let radius = 6371.0;
    radius * c
}

fn calculate_trip_duration_seconds(speed: f64, start: &Location, end: &Location) -> u64 {
    let distance = calculate_distance_km(start, end);
    (distance * 3600.0 / speed) as u64
}

fn add_progress(start: f64, end: f64, progress: f64, is_returning: bool) -> f64 {
    if is_returning {
        end + (start - end) * progress
    } else {
        start + (end - start) * progress
    }
}

pub fn calculate_current_position(start: &Location, end: &Location, speed: f64) -> Location {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("something went really wrong lol")
        .as_secs();

    let trip_duration = calculate_trip_duration_seconds(speed, start, end); // seconds
    let trip_progress = (now % trip_duration) as f64;
    let is_returning = (now % (trip_duration * 2)) > trip_duration;
    let trip_progress_ratio = trip_progress / (trip_duration as f64);

    Location {
        lat: add_progress(start.lat, end.lat, trip_progress_ratio, is_returning),
        lng: add_progress(start.lng, end.lng, trip_progress_ratio, is_returning),
    }
}