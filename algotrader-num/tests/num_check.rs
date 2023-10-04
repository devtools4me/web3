pub mod test_utils;

#[cfg(test)]
mod test {
    use algotrader_num::num::{get_cointegration, get_spread_z_score};
    use algotrader_num::types::*;

    use crate::test_utils;

    #[test]
    fn test_get_cointegration() {
        let btc_close_series = test_utils::read_ohlc("resources/test/BTC-USD-1DAY.json")
            .iter()
            .map(|x| x.close.parse::<f32>().unwrap())
            .collect::<Vec<_>>();

        let eth_close_series = test_utils::read_ohlc("resources/test/ETH-USD-1DAY.json")
            .iter()
            .map(|x| x.close.parse::<f32>().unwrap())
            .collect::<Vec<_>>();

        let result = get_cointegration(CointRequest {
            series_1: btc_close_series.clone(),
            series_2: eth_close_series.clone(),
        }).unwrap();
        println!("{:?}", result);

        assert_eq!(result, CointResponse {
            p_value: 0.25439155,
            coint_t: -2.557318,
            c_value: -3.3985443,
            hedge_ratio: 15.991336,
            zero_crossings: 8
        });

        let hedge_ration = result.hedge_ratio;
        let result = get_spread_z_score(SpreadRequest {
            series_1: btc_close_series.clone(),
            series_2: eth_close_series.clone(),
            hedge_ratio: hedge_ration,
            z_score_window: 21
        });
        println!("{:?}", result);
    }
}