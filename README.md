# lark_bot_sdk

飞书/Lark 的开放接口 Rust SDK，参考 [go sdk lark](https://github.com/chyroc/lark)。

特点:

- 支持 Mock 以支持测试
- 支持 UserAccessToken
- 支持流式上传下载
- 详细的注释

## 安装

为了加快编译速度，参照 [go sdk lark](https://github.com/chyroc/lark)，sdk 根据接口功能聚合成不同的 service，每个 service 对应一个 feature，如果要使用对应的 api 接口，请启用对应的 feature。(默认启用了 message)

```toml
[dependencies]
lark_bot_sdk = { version = "0.1", features = ["message", "ai",...] }
```

查询 api 方法参考: [cargo doc](https://booiris.github.io/lark_bot_sdk/lark_bot_sdk/) (`ctrl` + `shift` + `f`)

![case](https://cdn.jsdelivr.net/gh/booiris-cdn/img/case.gif)

## 使用

### 创建 Lark 客户端

```rust
let lark = Lark::new("<APP_ID>", "<APP_SECRET>");
```

client 可复用，所以一般来说实例化一次 client 即可。

```rust
fn client() -> &'static DefaultLarkClient {
    static CLIENT: OnceLock<DefaultLarkClient> = OnceLock::new();

    CLIENT.get_or_init(|| Lark::new("<APP_ID>", "<APP_SECRET>"))
}
```
### 配置 Lark 客户端

使用 `LarkBuilder` 配置客户端，具体参数参见: [parma](https://booiris.github.io/lark_bot_sdk/lark_bot_sdk/core/struct.LarkBuilder.html) 。

```rust
let client = LarkBuilder::default()
    .timeout(Duration::from_secs(10))
    .is_isv(false)
    .normal()
    .build(dotenv!("app_id"), dotenv!("app_secret"));

let custom_client = LarkBuilder::default().build_with_store_and_client(
    RWStoreMemory::default(),
    DefaultClient::default(),
    dotenv!("app_id"),
    dotenv!("app_secret"),
);
```

具体代码参见: [./examples/build_client.rs](./examples/build_client.rs)

### 例子: 发送消息

api 的调用格式为 `client.{service}.{func}`。发送消息的方法为 `send_raw_message`，属于 `message` service 下，所以调用方法为:

```rust
let req = SendRawMessageReq {
    receive_id_type: "open_id".to_owned(),
    receive_id: "".to_owned(),
    msg_type: "text".into(),
    content: "{\"text\":\"test content\"}".into(),
    ..Default::default()
};
client().message().send_raw_message(req).await;
```

具体代码参见: [./examples/send_message.rs](./examples/send_message.rs)

### 例子: 上传 api

上传接口需要传递 buffer 类型的数据。

```rust
let file = tokio::fs::File::open("test.jpg").await.unwrap();
let buffer = tokio::io::BufReader::new(file);
let req = UploadFileReq {
    file_name: "test.jpg".into(),
    file_type: "stream".into(),
    duration: None,
    data: buffer,
};
client().file().upload_file(req).await;
```

具体代码参见: [./examples/upload_file.rs](./examples/upload_file.rs)

### 例子: 下载 api

下载接口会返回一个 stream。

```rust
let req = DownloadFileReq {
    file_key: dotenv!("download_file_id").into(),
};
let res = client().file().download_file(req).await.unwrap();
let mut file = tokio::fs::File::create("test.jpg").await.unwrap();
while let Some(data) = resp.data.next().await {
    file.write_all(&data.unwrap()).await.unwrap();
}
```

具体代码参见: [./examples/download_file.rs](./examples/download_file.rs)

### 例子: 原生 api 调用

有些老版本的开放接口，不能生成结构化的 API， 导致 SDK 内无法提供结构化的使用方式，这时可使用原生模式进行调用:

```rust
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct ReqBody {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, lark_macros::ApiBaseResp)]
struct Resp {
    #[serde(flatten)]
    pub data: Option<String>,
    #[serde(flatten)]
    pub base: BaseResp,
}
#[tokio::main]
async fn main() {
    let req = ApiRequest {
        param_data: ReqParam {
            body: Some(ReqBody {
                name: "test".into(),
            }),
            ..Default::default()
        },
        ..Default::default()
    };
    client().do_req::<_, Resp>(req).await.ok();
}
```

tips: 可以使用宏简化构建参数过程，具体使用方法请参考示例和[文档](https://booiris.github.io/lark_bot_sdk/lark_bot_sdk/core/model/struct.ApiRequest.html)。

具体代码参见：[./examples/do_req.rs](examples/do_req.rs)

## Mock 方法

sdk 支持 Mock 以支持测试。要使用 mock 方法，需要启用 `test-util` feature。为了避免 mock 代码对正常逻辑的影响，**要在 `dev-dependencies` 中**
开启测试 feature。

```toml
[dev-dependencies]
lark_bot_sdk = { version = "0.1", features = ["test-util"] }
```
当前 mock 支持 `times` (最大 mock 次数) 和 `when` (mock 生效条件) 功能。使用变量的生命周期绑定 mock 生效时间，在函数结束后 mock 自动解除。

**! 使 mock 生效必须调用 `build` 方法。在 mock 范围内再次 mock 相同函数会导致 panic。**

### 普通方法 mock

在创建 client 后，使用 `client.{service}.mock().mock_{func_name}` 即可获取对应的 mocker 实例:

```rust
let _mocker = client()
    .message()
    .mock()
    .mock_send_raw_message(|_| {
        Ok((
            SendRawMessageResp {
                data: MessageSubResp {
                    message_id: "test".into(),
                    ..Default::default()
                },
            },
            CommonResponse::default(),
        ))
    })
    .times(2)
    .when(|req| req.receive_id == "1")
    .build();
```

具体代码参见: [./examples/mock_send_message.rs](./examples/mock_send_message.rs)

### 原生 api 调用 mock

原生 api mock 方法较为特殊。mock 方法的原理为每个类型生成一个对应的 mocker 。如果需要防止相同的函数类型相互覆盖（比如两个方法 A、B 的输入输出共用相同的结构体，如果不做区分的话那么方法 A 的 mock 会 覆盖方法 B 的 mock ）,在 mock 时需要提供一个额外的 marker 类型:

```rust
let _mocker = client()
        .mock()
        .mock_do_marker_req::<A, _, _>(|req| {
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

assert!(client().do_marker_req::<A, _, Resp>(req.clone()).await.is_ok());
// B 类型的调用不生效
assert!(client().do_marker_req::<B, _, Resp>(req.clone()).await.is_err());
```

具体代码参见: [./examples/mock_do_req.rs](./examples/mock_do_req.rs)

## Todo

- [ ] 常用接口封装
- [ ] 处理事件回调
- [ ] 事件回调支持 websocket
- [ ] 消息卡片支持
- [ ] 旧 api 生成
- [ ] api 自动更新

## 鸣谢

* [chyroc/lark](https://github.com/chyroc/lark)
