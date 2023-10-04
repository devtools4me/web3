use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CointRequest {
    pub series_1: Vec<f32>,
    pub series_2: Vec<f32>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CointResponse {
    pub p_value: f32,
    pub coint_t: f32,
    pub c_value: f32,
    pub hedge_ratio: f32,
    pub zero_crossings: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SpreadRequest {
    pub series_1: Vec<f32>,
    pub series_2: Vec<f32>,
    pub hedge_ratio: f32,
    pub z_score_window: i64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SpreadResponse {
    pub spread: Vec<String>,
    pub z_score: Vec<String>,
}