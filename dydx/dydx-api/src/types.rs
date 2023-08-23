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
    EMA,
    HMA,
    DEMA,
    DMA,
    RMA,
    SMA,
    SWMA,
    TEMA,
    TMA,
    TRIMA,
    VWMA,
    Vidya,
    WMA,
    WSMA
}

impl ToString for AverageType {
    fn to_string(&self) -> String {
        match self {
            AverageType::SMA => String::from("SMA"),
            AverageType::EMA => String::from("EMA"),
            AverageType::HMA => String::from("HMA"),
            AverageType::DEMA => String::from("DEMA"),
            AverageType::DMA => String::from("DMA"),
            AverageType::RMA => String::from("RMA"),
            AverageType::SWMA => String::from("SWMA"),
            AverageType::TEMA => String::from("TEMA"),
            AverageType::TMA => String::from("TMA"),
            AverageType::TRIMA => String::from("TRIMA"),
            AverageType::VWMA => String::from("VWMA"),
            AverageType::Vidya => String::from("Vidya"),
            AverageType::WMA => String::from("WMA"),
            AverageType::WSMA => String::from("WSMA"),
        }
    }
}

impl AverageType {
    pub fn parse(s: &str) -> AverageType {
        match s {
            "SMA" => AverageType::SMA,
            "EMA" => AverageType::EMA,
            "HMA" => AverageType::HMA,
            "DEMA" => AverageType::DEMA,
            "DMA" => AverageType::DMA,
            "RMA" => AverageType::RMA,
            "SWMA" => AverageType::SWMA,
            "TEMA" => AverageType::TEMA,
            "TMA" => AverageType::TMA,
            "TRIMA" => AverageType::TRIMA,
            "VWMA" => AverageType::VWMA,
            "Vidya" => AverageType::Vidya,
            "WMA" => AverageType::WMA,
            "WSMA" => AverageType::WSMA,
            _ => AverageType::SMA
        }
    }

    pub fn description(average_type: &AverageType) -> &str {
        match average_type {
            AverageType::SMA => "Simple Moving Average",
            AverageType::EMA => "Exponential Moving Average",
            AverageType::HMA => "Hull Moving Average",
            AverageType::DEMA => "Hull Moving Average",
            AverageType::DMA => "EMA over EMA",
            AverageType::RMA => "Running Moving Average",
            AverageType::SWMA => "Symmetrically Weighted Moving Average",
            AverageType::TEMA => "Triple Exponential Moving Average",
            AverageType::TMA => "DMA over EMA",
            AverageType::TRIMA => "Triangular Moving Average",
            AverageType::VWMA => "Volume Weighed Moving Average",
            AverageType::Vidya => "Variable Index Dynamic Average",
            AverageType::WMA => "Weighted Moving Average",
            AverageType::WSMA => "Wilder’s Smoothing Average",
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