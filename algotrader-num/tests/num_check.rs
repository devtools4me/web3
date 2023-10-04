pub mod test_utils;

#[cfg(test)]
mod test {
    use algotrader_num::num::get_cointegration;
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
            series_1: btc_close_series,
            series_2: eth_close_series,
        });
        println!("{:?}", result);

        assert_eq!(result.unwrap(), CointResponse {
            p_value: 0.25439155,
            coint_t: -2.557318,
            c_value: -3.3985443,
            hedge_ratio: 15.991336,
            zero_crossings: 8
        });
    }
}