use std::sync::OnceLock;

use dotenv_codegen::dotenv;
use lark_bot_sdk::core::DefaultLarkClient;
use lark_bot_sdk::error::Error::ErrApiResponse;
use lark_bot_sdk::{api::message::send_raw_message::SendRawMessageReq, core::Lark};

fn client() -> &'static DefaultLarkClient {
    static CLIENT: OnceLock<DefaultLarkClient> = OnceLock::new();

    CLIENT.get_or_init(|| Lark::new(dotenv!("app_id"), dotenv!("app_secret")))
}

#[tokio::main]
async fn main() {
    let req = SendRawMessageReq {
        receive_id_type: "open_id".to_owned(),
        receive_id: dotenv!("open_id").to_owned(),
        msg_type: "text".into(),
        content: "{\"text\":\"test content\"}".into(),
        ..Default::default()
    };
    let (resp, _) = match client().message().send_raw_message(req).await {
        Ok(resp) => resp,
        Err(e) => {
            if let ErrApiResponse(base_resp, _) = &e {
                println!("{:?}", base_resp);
                return;
            } else {
                panic!("{:?}", e)
            }
        }
    };
    println!("{:?}", resp);
}
