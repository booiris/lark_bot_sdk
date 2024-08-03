use std::{
    collections::HashMap,
    future::Future,
    sync::{Arc, RwLock},
    time::Duration,
};

use ahash::RandomState;
use chrono::Utc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("[lark sdk] store lock poison Err: {0}")]
    ErrStoreLockPoison(String),
    #[error("[lark sdk] store data not found")]
    ErrStoreNotFound,
    #[error("[lark sdk] store ttl overflow")]
    ErrStoreTTLOverflow,
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl<T> From<std::sync::PoisonError<T>> for StoreError {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Self::ErrStoreLockPoison(err.to_string())
    }
}

pub trait Store: Send + Sync + 'static {
    type GetV: Send + ToString + Clone + Sync;

    /// when key not exist or expired, return Err(ErrStoreNotFound)
    fn get(
        &self,
        key: &str,
    ) -> impl Future<Output = Result<(Self::GetV, Option<Duration>), StoreError>> + Send;
    fn set(
        &self,
        key: String,
        val: String,
        ttl: Option<Duration>,
    ) -> impl Future<Output = Result<(), StoreError>> + Send;
}

#[derive(Default)]
pub struct RWStoreMemory {
    data: RwLock<HashMap<String, MemoryElem, RandomState>>,
}

struct MemoryElem {
    data: Arc<String>,
    expire_at: i64,
}

impl Store for RWStoreMemory {
    type GetV = Arc<String>;

    async fn get(&self, key: &str) -> Result<(Arc<String>, Option<Duration>), StoreError> {
        let data = self.data.read()?;
        let elem = data.get(key).ok_or(StoreError::ErrStoreNotFound)?;
        if elem.expire_at < 0 {
            return Ok((elem.data.clone(), None));
        }
        let ttl = elem
            .expire_at
            .checked_sub(Utc::now().timestamp())
            .ok_or(StoreError::ErrStoreTTLOverflow)?;

        if ttl <= 0 {
            drop(data);
            let mut data = self.data.write()?;
            data.remove(key);
            return Err(StoreError::ErrStoreNotFound);
        }

        Ok((elem.data.clone(), Some(Duration::from_secs(ttl as u64))))
    }

    async fn set(&self, key: String, val: String, ttl: Option<Duration>) -> Result<(), StoreError> {
        let mut data = self.data.write()?;
        let mut elem = MemoryElem {
            data: Arc::new(val),
            expire_at: -1,
        };
        if let Some(ttl) = ttl {
            elem.expire_at = Utc::now()
                .timestamp()
                .checked_add(ttl.as_secs() as i64)
                .ok_or(StoreError::ErrStoreTTLOverflow)?;
        }
        data.insert(key, elem);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_memory_set_get() {
        let store = RWStoreMemory::default();
        let key = "test_key".to_string();
        let value = "test_value".to_string();
        let ttl = Some(Duration::from_secs(5));

        let set_result = store.set(key.clone(), value.clone(), ttl).await;
        assert!(set_result.is_ok(), "Set operation failed");

        let get_result = store.get(&key).await;
        assert!(get_result.is_ok(), "Get operation failed");

        let (retrieved_value, retrieved_ttl) = get_result.unwrap();
        assert_eq!(
            *retrieved_value, value,
            "Retrieved value does not match set value"
        );
        assert!(retrieved_ttl.is_some(), "Retrieved TTL is None");
        assert!(
            retrieved_ttl.unwrap() <= Duration::from_secs(5),
            "Retrieved TTL is wrong"
        )
    }

    #[tokio::test]
    async fn test_memory_ttl_expiry() {
        let store = RWStoreMemory::default();
        let key = "test_key".to_string();
        let value = "test_value".to_string();
        let ttl = Some(Duration::from_secs(0));

        let set_result = store.set(key.clone(), value.clone(), ttl).await;
        assert!(set_result.is_ok(), "Set operation failed");

        let get_result = store.get(&key).await;
        assert!(
            get_result.is_err(),
            "Get operation should fail due to TTL expiry"
        );
        let err = get_result.unwrap_err();
        assert!(matches!(err, StoreError::ErrStoreNotFound))
    }
}
