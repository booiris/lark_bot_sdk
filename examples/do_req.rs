use std::sync::OnceLock;

use dotenv_codegen::dotenv;
use lark_bot_sdk::{
    api::{BaseResp, HasBaseResp},
    core::{
        model::{ApiRequest, ReqParam, StreamReqParam},
        DefaultLarkClient, Lark,
    },
};
use serde::{Deserialize, Serialize};

fn client() -> &'static DefaultLarkClient {
    static CLIENT: OnceLock<DefaultLarkClient> = OnceLock::new();

    CLIENT.get_or_init(|| Lark::new(dotenv!("app_id"), dotenv!("app_secret")))
}

#[derive(Debug, Serialize, Clone, Default, lark_bot_sdk::macros::ApiReqParams)]
pub struct SendRawMessageReq {
    #[api(
        kind = "query",
        name = "receive_id_type",
        v_type = "var",
        option = "false"
    )]
    pub receive_id_type: String,
    #[api(kind = "body", name = "receive_id")]
    pub receive_id: String,
    #[api(kind = "body", name = "msg_type")]
    pub msg_type: String,
    #[api(kind = "body", name = "content")]
    pub content: String,
    #[api(kind = "body", name = "uuid")]
    pub uuid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk::macros::ApiBaseResp)]
struct Resp {
    #[serde(flatten)]
    pub data: Option<serde_json::Value>,
    #[serde(flatten)]
    pub base: BaseResp,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk::macros::ApiBaseResp)]
struct BotResp {
    #[serde(rename = "bot")]
    pub data: Option<BotSubResp>,
    #[serde(flatten)]
    pub base: BaseResp,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotSubResp {
    #[serde(rename = "activate_status")]
    pub activate_status: i64,
    #[serde(rename = "app_name")]
    pub app_name: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "ip_white_list")]
    pub ip_white_list: Vec<String>,
    #[serde(rename = "open_id")]
    pub open_id: String,
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .try_init()
        .unwrap();
    let req = ApiRequest {
        param_data: SendRawMessageReq {
            receive_id_type: "open_id".to_owned(),
            receive_id: dotenv!("open_id").to_owned(),
            msg_type: "text".into(),
            content: "{\"text\":\"test content\"}".into(),
            uuid: None,
        }
        .gen_param(),
        scope: "Message",
        api: "SendRawMessage",
        method: http::Method::POST,
        url: client().open_base_url().to_string() + "/open-apis/im/v1/messages",
        need_tenant_access_token: true,
        ..Default::default()
    };
    let resp = client().do_req::<_, Resp>(req.clone()).await;
    println!("{:?}", resp);
    assert!(resp.is_ok());

    let resp = client().do_req::<_, Resp>(req.clone()).await;
    println!("{:?}", resp);
    assert!(resp.is_ok());

    // 获取 bot 信息
    let req = ApiRequest::<()> {
        scope: "Bot",
        api: "GetBotInfo",
        method: http::Method::GET,
        url: client().open_base_url().to_string() + "/open-apis/bot/v3/info/",
        need_tenant_access_token: true,
        ..Default::default()
    };
    let resp = client().do_req::<_, BotResp>(req.clone()).await;
    println!("{:?}", resp);
    assert!(resp.is_ok());

    let resp = client().do_req::<_, BotResp>(req.clone()).await;
    println!("{:?}", resp);
    assert!(resp.is_ok());
}
