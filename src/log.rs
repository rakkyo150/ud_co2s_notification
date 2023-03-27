use serde::{Serialize,Deserialize};
use chrono::{DateTime, Local};

#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct UDCO2SStat {
    pub co2ppm: i64,
    pub humidity: f32,
    pub temperature: f32,
}

#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct Log{
    pub time: DateTime<Local>,
    pub status: UDCO2SStat
}

impl UDCO2SStat {
    pub fn new(co2ppm: i64, humidity: f32, temperature: f32) -> Self {
        UDCO2SStat {
            co2ppm,
            humidity,
            temperature,
        }
    }
}