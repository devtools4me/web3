use dydx_bot::service::utils::*;

#[tokio::test]
async fn format_price_works() {
    assert_eq!(Some(49404.0), format_price(49403.703, 1.0));
    assert_eq!(Some(49403.7), format_price(49403.703, 0.1));
    assert_eq!(Some(49403.71), format_price(49403.703, 0.01));
    assert_eq!(Some(49403.703), format_price(49403.703, 0.001));
}