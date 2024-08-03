use std::sync::OnceLock;

use dotenv_codegen::dotenv;
use lark_bot_sdk::api::file::upload_file::UploadFileReq;
use lark_bot_sdk::core::{DefaultLarkClient, Lark};
use lark_bot_sdk::error::Error::ErrApiResponse;

fn client() -> &'static DefaultLarkClient {
    static CLIENT: OnceLock<DefaultLarkClient> = OnceLock::new();

    CLIENT.get_or_init(|| Lark::new(dotenv!("app_id"), dotenv!("app_secret")))
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .try_init()
        .unwrap();
    let file = tokio::fs::File::open("source/秋后.jpg").await.unwrap();
    let buffer = tokio::io::BufReader::new(file);
    let req = UploadFileReq {
        file_name: "秋后.jpg".into(),
        file_type: "stream".into(),
        duration: None,
        data: buffer,
    };

    let (resp, common_resp) = match client().file().upload_file(req).await {
        Ok(resp) => resp,
        Err(e) => {
            if let ErrApiResponse(base_resp, common_resp) = &e {
                println!("{:?}\n{:?}", base_resp, common_resp);
                return;
            } else {
                panic!("{:?}", e)
            }
        }
    };
    println!("{:?}", resp);
    println!("{:?}\n", common_resp);
}
