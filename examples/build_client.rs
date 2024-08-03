use std::time::Duration;

use dotenv_codegen::dotenv;
use lark_bot_sdk::core::{http_client::DefaultClient, store::RWStoreMemory, LarkBuilder};

#[tokio::main]
async fn main() {
    let _client = LarkBuilder::default()
        .timeout(Duration::from_secs(10))
        .is_isv(false)
        .normal()
        .build(dotenv!("app_id"), dotenv!("app_secret"));

    let _custom_client = LarkBuilder::default().build_with_store_and_client(
        RWStoreMemory::default(),
        DefaultClient::default(),
        dotenv!("app_id"),
        dotenv!("app_secret"),
    );
}
