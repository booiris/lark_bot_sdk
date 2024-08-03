use std::sync::OnceLock;

use lark_bot_sdk::api::message::send_raw_message::{MessageSubResp, SendRawMessageResp};
use lark_bot_sdk::core::model::CommonResponse;
use lark_bot_sdk::core::DefaultLarkClient;
use lark_bot_sdk::{api::message::send_raw_message::SendRawMessageReq, core::Lark};

fn client() -> &'static DefaultLarkClient {
    static CLIENT: OnceLock<DefaultLarkClient> = OnceLock::new();

    CLIENT.get_or_init(|| Lark::new("", ""))
}

#[tokio::main]
async fn main() {
    // 必须使用一个变量绑定 mock 的生命周期，在函数结束后 mock 会自动释放，也可以手动调用 clear() 函数解除 mock
    // 注意不可以使用 let _ = xxx 绑定，具体原因参见 https://xuanwo.io/reports/2022-41/
    // **一定要调用 build 方法**
    let data = "test";
    let _mocker = client()
        .message()
        .mock()
        .mock_send_raw_message(move |_| {
            Ok((
                SendRawMessageResp {
                    data: MessageSubResp {
                        message_id: data.into(),
                        ..Default::default()
                    },
                },
                CommonResponse::default(),
            ))
        })
        .times(2)
        .when(|req| req.receive_id == "1")
        .build();

    let mock_req = SendRawMessageReq {
        receive_id: "1".to_owned(),
        ..Default::default()
    };
    let res = client().message().send_raw_message(mock_req.clone()).await;
    assert!(res.is_ok());
    let (resp, _) = res.unwrap();
    assert_eq!(resp.data.message_id, "test");

    // mock 条件不满足，mock 不生效
    let un_mock_req = SendRawMessageReq {
        receive_id: "2".to_owned(),
        ..Default::default()
    };
    let res = client().message().send_raw_message(un_mock_req).await;
    assert!(res.is_err());

    // 每个 mock 绑定单独的 client，不会相互影响
    let lark = Lark::new("".to_owned(), "".to_owned());
    assert!(lark
        .message()
        .send_raw_message(mock_req.clone())
        .await
        .is_err());

    // 消耗 mock 次数
    assert!(client().message().send_raw_message(mock_req).await.is_ok());

    // mock times 达到上限，mock 不生效
    let mock_req = SendRawMessageReq {
        receive_id: "1".to_owned(),
        ..Default::default()
    };
    let res = client().message().send_raw_message(mock_req).await;
    assert!(res.is_err());
}
