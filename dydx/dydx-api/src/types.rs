use std::str::FromStr;
use strum_macros::EnumString;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

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
    pub volume: String,
    pub timestamp: String,
}

#[derive(Debug, Eq, PartialEq, EnumString)]
pub enum AverageType {
    #[strum(serialize = "EMA", serialize = "ema")]
    EMA,
    #[strum(serialize = "HMA", serialize = "hma")]
    HMA,
    #[strum(serialize = "DEMA", serialize = "dema")]
    DEMA,
    #[strum(serialize = "DMA", serialize = "dma")]
    DMA,
    #[strum(serialize = "MOMENTUM", serialize = "Momentum", serialize = "momentum")]
    Momentum,
    #[strum(serialize = "RMA", serialize = "rma")]
    RMA,
    #[strum(serialize = "SMA", serialize = "sma")]
    SMA,
    #[strum(serialize = "SWMA", serialize = "swma")]
    SWMA,
    #[strum(serialize = "TEMA", serialize = "tema")]
    TEMA,
    #[strum(serialize = "TMA", serialize = "tma")]
    TMA,
    #[strum(serialize = "TRIMA", serialize = "trima")]
    TRIMA,
    #[strum(serialize = "VWMA", serialize = "vwma")]
    VWMA,
    #[strum(serialize = "Vidya", serialize = "vidya")]
    Vidya,
    #[strum(serialize = "WMA", serialize = "wma")]
    WMA,
    #[strum(serialize = "WSMA", serialize = "wsma")]
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
            AverageType::Momentum => String::from("MOMENTUM"),
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
            "MOMENTUM" => AverageType::Momentum,
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
            AverageType::Momentum => "Momentum",
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

#[derive(Debug, Eq, PartialEq, EnumString)]
pub enum IndicatorType {
    #[strum(serialize = "MACD", serialize = "macd")]
    MACD,
    #[strum(serialize = "RSI", serialize = "rsi")]
    RSI,
    #[strum(serialize = "RUN_TOGETHER", serialize = "run_together")]
    RunTogether,
}

impl ToString for IndicatorType {
    fn to_string(&self) -> String {
        match self {
            IndicatorType::MACD => String::from("MACD"),
            IndicatorType::RSI => String::from("RSI"),
            IndicatorType::RunTogether => String::from("RUN_TOGETHER"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ActionType {
    Buy,
    None,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Indicator {
    pub signals: Vec<ActionType>,
    pub values: Vec<String>,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RsiIndicator {
    pub enter_over_zone_signal: ActionType,
    pub leave_over_zone_signal: ActionType,
    pub value: String,
    pub timestamp: String,
}

impl RsiIndicator {
    pub fn from_i(i: &Indicator) -> Self {
        Self {
            enter_over_zone_signal: i.signals.get(0).unwrap().clone(),
            leave_over_zone_signal: i.signals.get(1).unwrap().clone(),
            value: i.values.first().unwrap().clone(),
            timestamp: i.timestamp.clone(),
        }
    }
}