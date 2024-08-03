use core::time;
use std::{sync::Arc, time::Duration};

use derive_builder::Builder;
use futures_util::Future;

use crate::error::Error;

use self::{
    http_client::{DefaultClient, HttpClient},
    store::{RWStoreMemory, Store},
};

pub mod http_client;
mod impl_request;
pub mod model;
pub mod store;

#[cfg_attr(docsrs, doc(cfg(feature = "test-util")))]
#[cfg(feature = "test-util")]
pub mod mocker;

#[cfg_attr(docsrs, doc(cfg(feature = "test-util")))]
#[cfg(feature = "test-util")]
pub mod impl_request_mock;

pub struct Lark<IStore, IHttpClient> {
    pub(crate) inner: Arc<LarkInner<IStore, IHttpClient>>,
}

impl<IStore, IHttpClient> Clone for Lark<IStore, IHttpClient> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T, U> Lark<T, U> {
    pub fn builder() -> LarkBuilder<T, U> {
        LarkBuilder::default()
    }

    pub fn open_base_url(&self) -> &str {
        &self.inner.open_base_url
    }

    pub fn www_base_url(&self) -> &str {
        &self.inner.www_base_url
    }

    pub fn app_id(&self) -> &str {
        &self.inner.app_id
    }
}

pub type DefaultLarkClient = Lark<RWStoreMemory, DefaultClient>;

impl Lark<RWStoreMemory, DefaultClient> {
    // 除了 app_id 和 app_secret 都有默认值，所以 build 的时候不应该有 error
    pub fn new<S: Into<String>>(app_id: S, app_secret: S) -> Self {
        LarkBuilder::default()
            .normal()
            .build(app_id, app_secret)
            .expect("build client error")
    }
}

#[derive(Builder)]
#[builder(
    setter(into),
    name = "LarkBuilder",
    public,
    pattern = "owned",
    build_fn(private, name = "build_from_builder")
)]
pub(crate) struct LarkInner<IStore, IHttpClient> {
    #[builder(private)]
    pub app_id: String,
    #[builder(private)]
    pub app_secret: String,

    #[builder(default, private)]
    /// 用于消息回调消息验证。
    pub encrypt_key: String,
    /// 用于消息回调消息验证。
    #[builder(default, private)]
    pub verification_token: String,

    #[builder(default, private)]
    pub help_desk_id: String,
    #[builder(default, private)]
    pub help_desk_token: String,

    #[builder(default = "time::Duration::from_secs(3)")]
    /// 设置 SDK 内置的 Http Client 的请求超时时间，默认值为 3s。
    pub timeout: Duration,
    #[builder(default = "false")]
    pub is_isv: bool,
    #[builder(default)]
    /// "设置租户 key, 当开发者开发商店应用时，必须设置该选项。
    pub tenant_key: String,
    #[builder(default = "String::from(\"https://open.feishu.cn\").into()")]
    /// 飞书域名，默认值为 <https://open.feishu.cn>
    pub open_base_url: String,
    #[builder(default = "String::from(\"https://www.feishu.cn\").into()")]
    pub www_base_url: String,
    #[builder(private)]
    /// 设置 HttpClient，用于替换 SDK 提供的默认实现。
    pub http_client: IHttpClient,
    #[builder(private)]
    /// 设置 token 缓存器，用来缓存 token 和 appTicket。
    pub store: IStore,
    #[builder(default)]
    #[allow(clippy::type_complexity)]
    pub get_app_ticket_func: Option<
        fn(
            &LarkInner<IStore, IHttpClient>,
            appID: &str,
        ) -> Box<dyn Future<Output = Result<String, Error>> + Unpin + Send>,
    >,
    #[builder(default = "true")]
    /// 是否定时刷新token，如果为 false, 则是在 token 缓存过期时才获取新 token，默认值为 true。
    pub auto_refresh_token: bool,

    #[builder(private, default = "Self::init_instance_id()")]
    #[cfg(feature = "test-util")]
    // 用于 mock 数据，为了防止 mock 冲突，每个实例都有一个唯一 id。
    pub instance_id: usize,
}

impl<T, U> LarkBuilder<T, U> {
    #[cfg(feature = "test-util")]
    fn init_instance_id() -> usize {
        INSTANCE_NUM.fetch_add(1, Ordering::Relaxed)
    }
}

#[cfg(feature = "test-util")]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(feature = "test-util")]
static INSTANCE_NUM: AtomicUsize = AtomicUsize::new(0);

impl<IStore: Store, IClient: HttpClient> Lark<IStore, IClient> {
    fn auto_refresh_token(&self) {
        if !self.inner.auto_refresh_token {
            return;
        }
        let cli = self.clone();
        tokio::task::spawn(async move {
            let mut wrong_times = 0;
            const DEFAULT_WAIT_TIME: std::time::Duration = Duration::from_secs(60);
            loop {
                let app_access_token = cli.inner.auth().get_app_access_token().await;
                let tenant_access_token = cli.inner.auth().get_tenant_access_token().await;
                if let (Ok(t1), Ok(t2)) = (app_access_token, tenant_access_token) {
                    let wait_time =
                        t1.0.expire
                            .max(t2.0.expire)
                            .map(|x| x / 3 * 2)
                            .unwrap_or(DEFAULT_WAIT_TIME);
                    tracing::debug!("refresh token success, wait time: {:?}", wait_time);
                    tokio::time::sleep(wait_time).await;
                } else {
                    wrong_times += 1;
                    if wrong_times > 5 {
                        wrong_times = 0;
                        tracing::error!(
                            "refresh token failed , wait time: {:?}",
                            DEFAULT_WAIT_TIME
                        );
                    }
                    tokio::time::sleep(DEFAULT_WAIT_TIME).await;
                }
            }
        });
    }
}

impl<T, U> LarkBuilder<T, U> {
    pub fn with_event_callback_verify<S: Into<String>>(
        self,
        encrypt_key: S,
        verification_token: S,
    ) -> Self {
        self.encrypt_key(encrypt_key)
            .verification_token(verification_token)
    }

    pub fn with_help_desk<S: Into<String>>(self, help_desk_id: S, help_desk_token: S) -> Self {
        self.help_desk_id(help_desk_id)
            .help_desk_token(help_desk_token)
    }
}

impl<T: Store, U: HttpClient> LarkBuilder<T, U> {
    pub fn build_with_store_and_client<S: Into<String>>(
        self,
        store: T,
        client: U,
        app_id: S,
        app_secret: S,
    ) -> Result<Lark<T, U>, Error> {
        let lark = match self
            .store(store)
            .http_client(client)
            .app_id(app_id)
            .app_secret(app_secret)
            .build_from_builder()
        {
            Ok(c) => c,
            Err(err) => return Err(Error::ErrBuildClient(err.to_string())),
        };
        let lark = Lark {
            inner: Arc::new(lark),
        };
        lark.auto_refresh_token();
        Ok(lark)
    }
}

impl LarkBuilder<RWStoreMemory, DefaultClient> {
    pub fn normal(self) -> Self {
        self
    }

    pub fn build<S: Into<String>>(
        self,
        app_id: S,
        app_secret: S,
    ) -> Result<Lark<RWStoreMemory, DefaultClient>, Error> {
        let lark = match self
            .store(RWStoreMemory::default())
            .http_client(DefaultClient::default())
            .app_id(app_id)
            .app_secret(app_secret)
            .build_from_builder()
        {
            Ok(c) => c,
            Err(err) => return Err(Error::ErrBuildClient(err.to_string())),
        };
        let lark = Lark {
            inner: Arc::new(lark),
        };
        lark.auto_refresh_token();
        Ok(lark)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_lark_new() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        let lark = Lark::new(app_id.to_owned(), app_secret.to_owned());

        assert_eq!(lark.inner.app_id, app_id);
        assert_eq!(lark.inner.app_secret, app_secret);
        assert_eq!(lark.inner.encrypt_key, String::new());
        assert_eq!(lark.inner.verification_token, String::new());
        assert_eq!(lark.inner.help_desk_id, String::new());
        assert_eq!(lark.inner.help_desk_token, String::new());
        assert_eq!(lark.inner.timeout, Duration::from_secs(3));
        assert!(!lark.inner.is_isv);
        assert_eq!(lark.inner.tenant_key, String::new());
        assert_eq!(
            lark.inner.open_base_url,
            String::from("https://open.feishu.cn")
        );
        assert_eq!(
            lark.inner.www_base_url,
            String::from("https://www.feishu.cn")
        );
    }

    #[tokio::test]
    async fn test_lark_builder() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        let lark = LarkBuilder::default()
            .normal()
            .build(app_id.to_owned(), app_secret.to_owned())
            .unwrap();

        assert_eq!(lark.inner.app_id, app_id);
        assert_eq!(lark.inner.app_secret, app_secret);
        assert_eq!(lark.inner.encrypt_key, String::new());
        assert_eq!(lark.inner.verification_token, String::new());
        assert_eq!(lark.inner.help_desk_id, String::new());
        assert_eq!(lark.inner.help_desk_token, String::new());
        assert_eq!(lark.inner.timeout, Duration::from_secs(3));
        assert!(!lark.inner.is_isv);
        assert_eq!(lark.inner.tenant_key, String::new());
        assert_eq!(
            lark.inner.open_base_url,
            String::from("https://open.feishu.cn")
        );
        assert_eq!(
            lark.inner.www_base_url,
            String::from("https://www.feishu.cn")
        );
    }

    #[tokio::test]
    async fn test_lark_custom_builder() {
        let app_id = "test_app_id".to_string();
        let app_secret = "test_app_secret".to_string();

        let lark = LarkBuilder::default()
            .build_with_store_and_client(
                RWStoreMemory::default(),
                DefaultClient::default(),
                app_id.clone(),
                app_secret.clone(),
            )
            .unwrap();

        assert_eq!(lark.inner.app_id, app_id);
        assert_eq!(lark.inner.app_secret, app_secret);
        assert_eq!(lark.inner.encrypt_key, String::new());
        assert_eq!(lark.inner.verification_token, String::new());
        assert_eq!(lark.inner.help_desk_id, String::new());
        assert_eq!(lark.inner.help_desk_token, String::new());
        assert_eq!(lark.inner.timeout, Duration::from_secs(3));
        assert!(!lark.inner.is_isv);
        assert_eq!(lark.inner.tenant_key, String::new());
        assert_eq!(
            lark.inner.open_base_url,
            String::from("https://open.feishu.cn")
        );
        assert_eq!(
            lark.inner.www_base_url,
            String::from("https://www.feishu.cn")
        );
    }
}
