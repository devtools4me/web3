use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

pub trait New<T> {
    fn new(value: T) -> Self;
}

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

type OhlcTuple<'a> = (&'a str, &'a str, &'a str, &'a str, &'a str, &'a str);

impl New<OhlcTuple<'_>> for Ohlc {
    fn new(t: OhlcTuple) -> Self {
        Ohlc {
            open: String::from(t.0),
            high: String::from(t.1),
            low: String::from(t.2),
            close: String::from(t.3),
            volume: String::from(t.4),
            timestamp: String::from(t.5)
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Display, EnumString, EnumIter)]
pub enum AverageType {
    #[strum(serialize = "EMA")]
    EMA,
    #[strum(serialize = "HMA")]
    HMA,
    #[strum(serialize = "DEMA")]
    DEMA,
    #[strum(serialize = "DMA")]
    DMA,
    #[strum(serialize = "Momentum")]
    Momentum,
    #[strum(serialize = "RMA")]
    RMA,
    #[strum(serialize = "SMA")]
    SMA,
    #[strum(serialize = "SWMA")]
    SWMA,
    #[strum(serialize = "TEMA")]
    TEMA,
    #[strum(serialize = "TMA")]
    TMA,
    #[strum(serialize = "TRIMA")]
    TRIMA,
    #[strum(serialize = "VWMA")]
    VWMA,
    #[strum(serialize = "Vidya")]
    Vidya,
    #[strum(serialize = "WMA")]
    WMA,
    #[strum(serialize = "WSMA")]
    WSMA
}

impl AverageType {

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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TimeseriesF32 {
    pub value: f32,
    pub timestamp: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Display, EnumString, EnumIter)]
pub enum IndicatorType {
    #[strum(serialize = "MACD")]
    MACD,
    #[strum(serialize = "RSI")]
    RSI,
    #[strum(serialize = "RunTogether")]
    RunTogether,
    #[strum(serialize = "SellVolatility")]
    SellVolatility,
}

impl IndicatorType {
    pub fn description(indicator_type: &IndicatorType) -> &str {
        match indicator_type {
            IndicatorType::MACD => "Moving Average Convergence Divergence",
            IndicatorType::RSI => "Relative Strength Index",
            IndicatorType::RunTogether => "Run Together",
            IndicatorType::SellVolatility => "Sell Volatility",
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
pub struct MacdIndicator {
    pub macd_value: String,
    pub sigline_value: String,
    pub timestamp: String,
}

impl New<&Indicator> for MacdIndicator {
    fn new(i: &Indicator) -> Self {
        Self {
            macd_value: i.values.get(0).unwrap().clone(),
            sigline_value: i.values.get(1).unwrap().clone(),
            timestamp: i.timestamp.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RsiIndicator {
    pub enter_over_zone_signal: ActionType,
    pub leave_over_zone_signal: ActionType,
    pub value: String,
    pub timestamp: String,
}

impl New<&Indicator> for RsiIndicator {
    fn new(i: &Indicator) -> Self {
        Self {
            enter_over_zone_signal: i.signals.get(0).unwrap().clone(),
            leave_over_zone_signal: i.signals.get(1).unwrap().clone(),
            value: i.values.first().unwrap().clone(),
            timestamp: i.timestamp.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SourceChangeIndicator {
    pub signal: ActionType,
    pub value: String,
    pub timestamp: String,
}

impl New<&Indicator> for SourceChangeIndicator {
    fn new(i: &Indicator) -> Self {
        Self {
            signal: i.signals.get(0).unwrap().clone(),
            value: i.values.get(0).unwrap().clone(),
            timestamp: i.timestamp.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CointegrationData {
    pub p_value: f32,
    pub coint_t: f32,
    pub c_value: f32,
    pub hedge_ratio: f32,
    pub zero_crossings: i64,
}

impl Default for CointegrationData {
    fn default() -> Self {
        Self {
            p_value: 0.0,
            coint_t: 0.0,
            c_value: 0.0,
            hedge_ratio: 0.0,
            zero_crossings: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SpreadZScoreData {
    pub series_1: Vec<f32>,
    pub series_2: Vec<f32>,
    pub spread: Vec<String>,
    pub z_score: Vec<String>,
    pub timestamp: Vec<String>,
}

impl Default for SpreadZScoreData {
    fn default() -> Self {
        Self {
            series_1: vec![],
            series_2: vec![],
            spread: vec![],
            z_score: vec![],
            timestamp: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct TrendsData {
    pub cointegration_data: CointegrationData,
    pub spread_zscore_data: SpreadZScoreData,
}

impl Default for TrendsData {
    fn default() -> Self {
        Self {
            cointegration_data: CointegrationData::default(),
            spread_zscore_data: SpreadZScoreData::default(),
        }
    }
}