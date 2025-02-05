use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use maxminddb::geoip2::city::{City, Country, Location};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const OUR_LOCATIONS: [&str; 3] = ["217.144.185.110", "45.143.203.154", "95.215.108.47"];
const GEOIP_DATA: &[u8] = include_bytes!("../geoip.mmdb");

pub fn get_nest() -> Router {
    Router::new().route("/", get(get_our_locations))
}

#[derive(Deserialize, Serialize)]
struct Response<'a> {
    #[serde(borrow)]
    city: Option<City<'a>>,
    country: Option<Country<'a>>,
    location: Option<Location<'a>>,
}

async fn get_our_locations() -> Result<impl IntoResponse, StatusCode> {
    let reader = maxminddb::Reader::from_source(GEOIP_DATA)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut locations = BTreeMap::new();

    for addr in OUR_LOCATIONS {
        let city = reader
            .lookup::<Response>(addr.parse().unwrap())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        locations.insert(addr, city);
    }

    let mock_data =
        serde_json::to_string(&locations).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(mock_data))
}
