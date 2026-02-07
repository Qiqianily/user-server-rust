use std::{ops::Deref, sync::Arc};

use crate::{factory::client::GrpcUserClientFactory, response::ApiResult};

// AppStateInner is a struct that holds the inner state of the application.
// It is used to store application-specific data that needs to be shared between different parts of the application.
#[derive(Debug, Clone)]
pub struct AppStateInner {}

/// AppState app 的状态
///
/// # 成员
/// - grpc_factory: client 里面维持了一个连接池
/// - inner: 内部共享状态
// AppState is a struct that holds the state of the application.
// It contains a pool of postgres connections and a reference to an inner struct.
#[derive(Debug, Clone)]
pub struct AppState {
    pub grpc_factory: GrpcUserClientFactory,
    pub inner: Arc<AppStateInner>,
}
// construct a new AppState object with a pool of postgres connections and an inner struct.
impl AppState {
    pub async fn new(grpc_addr: &str) -> ApiResult<Self> {
        // 创建 grpc factory
        let grpc_factory = GrpcUserClientFactory::new(grpc_addr).await?;

        Ok(Self {
            grpc_factory,
            inner: Arc::new(AppStateInner {}),
        })
    }
}

// Deref allows us to access the inner struct of an AppState object using the dot operator.
// This is useful for accessing application-specific data stored in the inner struct.
impl Deref for AppState {
    type Target = AppStateInner; // the type of the inner struct
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
