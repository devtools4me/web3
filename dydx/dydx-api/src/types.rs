use std::str::FromStr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct OhlcResponse {
    pub list: Vec<Ohlc>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Ohlc {
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub timestamp: String,
}

#[derive(Debug, Eq, PartialEq, EnumString)]
pub enum AverageType {
    Unknown,
    SMA,
    EMA,
}

impl ToString for AverageType {
    fn to_string(&self) -> String {
        match self {
            AverageType::SMA => String::from("SMA"),
            AverageType::EMA => String::from("EMA"),
            _ => String::from("Unknown")
        }
    }
}

impl AverageType {
    pub fn parse(s: &str) -> AverageType {
        match s {
            "SMA" => AverageType::SMA,
            "EMA" => AverageType::EMA,
            _ => AverageType::Unknown
        }
    }

    pub fn description(average_type: &AverageType) -> &str {
        match average_type {
            AverageType::SMA => "Simple Moving Average",
            _ => "NA"
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeseriesResponse {
    pub list: Vec<Timeseries>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Timeseries {
    pub value: String,
    pub timestamp: String,
}