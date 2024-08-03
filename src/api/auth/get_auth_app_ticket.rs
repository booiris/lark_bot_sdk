use crate::api::auth::gen_isv_app_ticket_key;

use crate::{
    core::{
        http_client::HttpClient,
        store::{Store, StoreError},
    },
    error::Error,
};

use super::service::AuthService;

impl<'c, IStore: Store, IClient: HttpClient> AuthService<'c, IStore, IClient> {
    /// docs: <https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token_internal>
    ///
    /// docs: <https://open.feishu.cn/document/ukTMukTMukTM/ukDNz4SO0MjL5QzM/auth-v3/auth/tenant_access_token>
    pub async fn get_auth_app_ticket(&self) -> Result<String, Error> {
        #[cfg(feature = "test-util")]
        {
            if let Some(f) = self.mock().get_mock_auth_app_ticket(&()) {
                tracing::info!("[lark] Auth#GetAuthAppTicket **mocking** api");
                return f();
            }
        }

        tracing::info!("[lark] Auth#GetAuthAppTicket call api");

        let key = &gen_isv_app_ticket_key(&self.cli.app_id);
        match self.cli.store.get(key).await {
            Ok(token) => {
                return Ok(token.0.to_string());
            }
            Err(e) if !matches!(e, StoreError::ErrStoreNotFound) => {
                tracing::error!(
                    "[lark] Auth#GetAuthAppTicket get token from store error: {}",
                    e
                );
            }
            _ => {}
        }

        let ticket = match self.cli.get_app_ticket_func {
            Some(f) => f(self.cli, &self.cli.app_id).await?,
            None => return Err(Error::ErrGetTicketFuncMissing),
        };

        Ok(ticket)
    }
}

#[cfg(feature = "test-util")]
mod test_utils {
    use self::auth::service::AuthServiceMocker;
    use crate::core::mocker::*;

    use super::*;
    use crate::api::*;

    pub struct Mocker;

    type MockFunType = fn() -> Result<String, Error>;
    impl<'c, IStore: Store, IClient: HttpClient> AuthServiceMocker<'c, IStore, IClient> {
        pub fn mock_auth_app_ticket(
            &self,
            f: MockFunType,
        ) -> MockerBuilder<Mocker, (), String, MockFunType> {
            MockerBuilder::new(self.cli.instance_id, f)
        }

        pub(super) fn get_mock_auth_app_ticket(&self, req: &()) -> Option<MockFunType> {
            do_mock::<Mocker, (), String, MockFunType>(self.cli.instance_id, req)
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use crate::core::Lark;

    #[tokio::test]
    #[cfg(feature = "test-util")]
    async fn test_get_auth_app_ticket() {
        let lark = Lark::new("".to_owned(), "".to_owned());

        let _mocker = lark
            .auth()
            .mock()
            .mock_auth_app_ticket(|| Ok(String::new()))
            .build();

        let token = lark.auth().get_auth_app_ticket().await.unwrap();
        assert_eq!(token, "");
    }

    #[tokio::test]
    async fn test_wrong_resp() {
        let lark = Lark::new("".to_owned(), "".to_owned());
        let token = lark.auth().get_auth_app_ticket().await;
        assert!(token.is_err());
        dbg!(token).ok();
    }
}
