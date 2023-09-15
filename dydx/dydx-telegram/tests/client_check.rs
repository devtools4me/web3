#[cfg(test)]
mod test {
    use dydx_telegram::client::{TelegramApi, TelegramClient};
    use dydx_common::utils::env_utils;

    #[tokio::test]
    async fn test_client_send_message() {
        let client = TelegramClient {
            token: env_utils::get_telegram_api_token().clone(),
            chat_id: env_utils::get_telegram_chat_id().clone(),
        };
        let res = client.send_message("test")
            .await
            .unwrap();
        println!("{:?}", res);

        //assert_eq!(sut.ma, MA::EMA(10));
    }
}