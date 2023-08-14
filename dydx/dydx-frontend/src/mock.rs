use dydx_api::types::*;

pub struct MockDydxApi {}

impl DydxApi for MockDydxApi {
    fn get_ohlc_data(&self) -> Vec<Ohlc> {
        vec![
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:09:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:08:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:07:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:06:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:05:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:04:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:03:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:02:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:01:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T09:00:00.000Z".to_string(),
            },
            Ohlc {
                open: "29450".to_string(),
                high: "29450".to_string(),
                low: "29450".to_string(),
                close: "29450".to_string(),
                timestamp: "2023-08-14T08:59:00.000Z".to_string(),
            },
        ]
    }
}