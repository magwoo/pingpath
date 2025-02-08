use chrono::NaiveDateTime;

use self::addr::LocationAddr;

pub mod addr;

pub struct Location {
    id: i64,
    addr: Option<LocationAddr>,
    country: String,
    city: Option<String>,
    latitude: f64,
    longitude: f64,
    create_date: NaiveDateTime,
}
