pub mod test_utils;

#[cfg(test)]
mod test {
    use algotrader_num::num::get_cointegration;
    use algotrader_num::types::CointRequest;

    #[test]
    fn test_get_cointegration() {
        let result = get_cointegration(CointRequest {
            series_1: vec![0.8286],
            series_2: vec![0.8286],
        });
        println!("{:?}", result);
    }
}