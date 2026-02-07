use std::{sync::Arc, time::Duration};

use crate::{pb::user::user_service_client::UserServiceClient, response::ApiResult};
/// 定义一个 GRPC 客户端工厂
#[derive(Debug, Clone)]
pub struct GrpcUserClientFactory {
    channel: Arc<tonic::transport::Channel>,
}

impl GrpcUserClientFactory {
    /// 创建一个新的 GRPC 客户端工厂。
    pub async fn new(addr: &str) -> ApiResult<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(addr.to_string())
            .expect("创建 EndPoint 时出错了！")
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5))
            .keep_alive_while_idle(true);

        let channel = Arc::new(endpoint.connect_lazy());
        Ok(Self { channel })
    }

    /// 创建一个新的 GRPC 客户端。
    pub async fn create_client(&self) -> ApiResult<UserServiceClient<tonic::transport::Channel>> {
        Ok(UserServiceClient::new(self.channel.as_ref().clone()))
    }
}
