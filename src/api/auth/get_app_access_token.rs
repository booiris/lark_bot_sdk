use core::time;

use crate::{
    api::auth::{GetAccessTokenReq, GetAccessTokenResp},
    core::model::{ApiRequest, CommonResponse},
};

use crate::{
    api::auth::gen_app_token_key,
    core::{
        http_client::HttpClient,
        store::{Store, StoreError},
    },
    error::Error,
};

use super::{service::AuthService, TokenExpire};

impl<'c, IStore: Store, IClient: HttpClient> AuthService<'c, IStore, IClient> {
    /// docs: <https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal>
    ///
    /// docs: <https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token>
    pub async fn get_app_access_token(&self) -> Result<(TokenExpire, CommonResponse), Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_app_access_token(&()) {
                tracing::info!("[lark] Auth#GetAppAccessToken **mocking** api");
                return f();
            }
        }

        tracing::info!("[lark] Auth#GetAppAccessToken call api");

        let key = gen_app_token_key(self.cli.is_isv, &self.cli.app_id);
        match self.cli.store.get(&key).await {
            Ok(token) => {
                return Ok((
                    TokenExpire {
                        token: token.0.to_string(),
                        expire: token.1,
                    },
                    CommonResponse::default(),
                ));
            }
            Err(e) if !matches!(e, StoreError::ErrStoreNotFound) => {
                tracing::error!(
                    "[lark] Auth#GetAppAccessToken get token from store error: {}",
                    e
                );
            }
            _ => {}
        }

        let mut url =
            self.cli.open_base_url.clone() + "/open-apis/auth/v3/app_access_token/internal";

        let mut req = GetAccessTokenReq {
            app_id: self.cli.app_id.clone(),
            app_secret: self.cli.app_secret.clone(),
            ..Default::default()
        };

        if self.cli.is_isv {
            let ticket = self.get_auth_app_ticket().await?;
            url = self.cli.open_base_url.clone() + "/open-apis/auth/v3/app_access_token";
            req.app_ticket = ticket;
        }

        let req = ApiRequest {
            scope: "Auth",
            api: "GetAppAccessToken",
            method: http::Method::POST,
            url,
            param_data: req.gen_param(),
            ..Default::default()
        };
        let resp: (GetAccessTokenResp, _) = self.cli.do_req_without_auth(req).await?;

        let data = match resp.0.data {
            Some(data) => data,
            None => {
                return Err(Error::ErrResponse(
                    anyhow::anyhow!("missing response data"),
                    resp.1,
                ));
            }
        };

        self.cli
            .store
            .set(
                key.clone(),
                data.tenant_access_token.clone(),
                Some(time::Duration::from_secs(data.expire - 100)),
            )
            .await
            .map_err(Error::ErrStoreToken)?;

        Ok((
            TokenExpire {
                token: data.tenant_access_token,
                expire: Some(time::Duration::from_secs(data.expire)),
            },
            resp.1,
        ))
    }
}

#[cfg(feature = "test-util")]
mod test_utils {
    use self::auth::service::AuthServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;
    use crate::core::model::*;

    pub struct Mocker;

    type MockFunType = fn() -> Result<(TokenExpire, CommonResponse), Error>;
    impl<'c, IStore: Store, IClient: HttpClient> AuthServiceMocker<'c, IStore, IClient> {
        pub fn mock_app_access_token(
            &self,
            f: MockFunType,
        ) -> MockerBuilder<Mocker, (), TokenExpire, MockFunType> {
            MockerBuilder::new(self.cli.instance_id, f)
        }

        pub(super) fn get_mock_app_access_token(&self, req: &()) -> Option<MockFunType> {
            do_mock::<Mocker, (), TokenExpire, MockFunType>(self.cli.instance_id, req)
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use dotenv_codegen::dotenv;

    use crate::core::Lark;

    #[cfg(feature = "_local_test")]
    #[tokio::test]
    async fn test_get_app_access_token() {
        let lark = Lark::new(
            dotenv!("app_id").to_owned(),
            dotenv!("app_secret").to_owned(),
        );

        let token = lark
            .auth()
            .get_app_access_token()
            .await
            .expect("get token err");
        tracing::info!("{:?}", token);
    }

    #[tokio::test]
    #[cfg(feature = "test-util")]
    async fn test_mock_get_app_access_token() {
        use crate::{api::auth::TokenExpire, core::model::CommonResponse};

        let lark = Lark::new("".to_owned(), "".to_owned());

        let _mocker = lark
            .auth()
            .mock()
            .mock_app_access_token(|| {
                Ok((
                    TokenExpire {
                        token: "123".to_owned(),
                        expire: None,
                    },
                    CommonResponse::default(),
                ))
            })
            .times(1)
            .build();

        let token = lark
            .auth()
            .get_app_access_token()
            .await
            .expect("get token err");
        assert_eq!(token.0.token, "123");

        let res = lark.auth().get_app_access_token().await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_wrong_resp() {
        let lark = Lark::new("".to_owned(), "".to_owned());
        let token = lark.auth().get_app_access_token().await;
        assert!(token.is_err());
        assert!(matches!(
            token,
            Err(crate::error::Error::ErrApiResponse(_, _))
        ));
    }
}
