use std::fmt::Debug;

use serde::Serialize;

use crate::{
    core::mocker::{do_mock, MockerBuilder},
    error::Error,
};

use super::{
    http_client::HttpClient,
    impl_request::DoReqMarker,
    model::{ApiRequest, CommonResponse},
    store::Store,
    Lark, LarkInner,
};

#[cfg(feature = "test-util")]
impl<Store, Client> Lark<Store, Client> {
    pub fn mock(&self) -> LarkMocker<Store, Client> {
        LarkMocker {
            cli: self.inner.as_ref(),
        }
    }
}

type MockFunType<Req, Resp> = fn(Req) -> Result<(Resp, CommonResponse), Error>;

#[cfg(feature = "test-util")]
pub struct LarkMocker<'client, Store, Client> {
    cli: &'client LarkInner<Store, Client>,
}

impl<'c, IStore: Store, IClient: HttpClient> LarkMocker<'c, IStore, IClient> {
    pub fn mock_do_marker_req<
        Maker: 'static,
        ReqBody: Debug + Serialize + 'static,
        Resp: 'static,
    >(
        &self,
        f: MockFunType<ApiRequest<ReqBody>, Resp>,
    ) -> MockerBuilder<Maker, ApiRequest<ReqBody>, Resp, MockFunType<ApiRequest<ReqBody>, Resp>>
    {
        MockerBuilder::new(self.cli.instance_id, f)
    }

    pub fn mock_do_req<ReqBody: Debug + Serialize + 'static, Resp: 'static>(
        &self,
        f: MockFunType<ApiRequest<ReqBody>, Resp>,
    ) -> MockerBuilder<DoReqMarker, ApiRequest<ReqBody>, Resp, MockFunType<ApiRequest<ReqBody>, Resp>>
    {
        MockerBuilder::new(self.cli.instance_id, f)
    }

    pub(super) fn get_mock_do_req<
        Maker: 'static,
        ReqBody: Debug + Serialize + 'static,
        Resp: 'static,
    >(
        &self,
        req: &ApiRequest<ReqBody>,
    ) -> Option<MockFunType<ApiRequest<ReqBody>, Resp>> {
        do_mock::<Maker, ApiRequest<ReqBody>, Resp, MockFunType<ApiRequest<ReqBody>, Resp>>(
            self.cli.instance_id,
            req,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::api::HasBaseResp;
    use crate::{api::BaseResp, core::model::ReqParam};
    use serde::{Deserialize, Serialize};

    use crate::core::{model::ApiRequest, Lark};

    #[derive(Debug, Serialize, Deserialize, Clone, lark_bot_sdk_macros::ApiBaseResp)]
    struct TestResp {
        pub data: Option<String>,
        #[serde(flatten)]
        pub base: BaseResp,
    }

    #[tokio::test]
    async fn test_mock_do_req() {
        let lark = Lark::new("", "");

        let yes = true;
        let _mocker = lark
            .mock()
            .mock_do_req(|req| {
                Ok((
                    TestResp {
                        data: Some(req.param_data.body.unwrap()),
                        base: BaseResp::default(),
                    },
                    Default::default(),
                ))
            })
            .times(10)
            .when(move |req| req.param_data.body.is_some() && yes)
            .build();

        let req = ApiRequest {
            param_data: ReqParam {
                body: Some("hello".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        let resp: (TestResp, _) = lark.do_req(req).await.unwrap();
        assert_eq!(resp.0.data.unwrap(), "hello");

        let req = ApiRequest::<String>::default();
        let res: Result<(TestResp, _), _> = lark.do_req(req).await;
        tracing::info!("{:?}", res);
        assert!(res.is_err());
    }
}
