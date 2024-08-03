use std::sync::OnceLock;

use lark_bot_sdk::{
    api::{BaseResp, HasBaseResp},
    core::{
        model::{ApiRequest, ReqParam},
        DefaultLarkClient, Lark,
    },
};
use serde::{Deserialize, Serialize};

fn client() -> &'static DefaultLarkClient {
    static CLIENT: OnceLock<DefaultLarkClient> = OnceLock::new();

    CLIENT.get_or_init(|| Lark::new("", ""))
}

struct User;

struct Author;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resp {
    pub data: Option<String>,
    #[serde(flatten)]
    pub base: BaseResp,
}

impl HasBaseResp for Resp {
    fn base_resp(&self) -> &BaseResp {
        &self.base
    }

    fn take_base_resp(self) -> BaseResp {
        self.base
    }
}

#[tokio::main]
async fn main() {
    let _mocker = client()
        .mock()
        .mock_do_marker_req::<User, _, _>(|req| {
            Ok((
                Resp {
                    data: Some(req.param_data.body.unwrap()),
                    base: BaseResp::default(),
                },
                Default::default(),
            ))
        })
        .build();

    let req = ApiRequest {
        param_data: ReqParam {
            body: Some("req_body".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let res = client().do_marker_req::<User, _, Resp>(req).await;
    assert!(res.is_ok());
    let (resp, _) = res.unwrap();
    assert_eq!(resp.data.unwrap(), "req_body");

    let req = ApiRequest {
        param_data: ReqParam {
            body: Some("req_body".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    // Author 类型的 mock 不会生效
    let res = client().do_marker_req::<Author, _, Resp>(req).await;
    assert!(res.is_err());

    // common mock
    let _mocker = client()
        .mock()
        .mock_do_req(|req| {
            Ok((
                Resp {
                    data: Some(req.param_data.body.unwrap()),
                    base: BaseResp::default(),
                },
                Default::default(),
            ))
        })
        .build();
    let req = ApiRequest {
        param_data: ReqParam {
            body: Some("req_body".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    let res = client().do_req::<_, Resp>(req).await;
    assert!(res.is_ok())
}
