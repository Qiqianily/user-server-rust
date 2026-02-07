use axum::{debug_handler, extract::State};

use crate::{
    common::valid::ValidJson,
    handlers::common::model::{RegisterResult, RegisterUserParam},
    pb::user::{UserExistsRequest, UserRegisterRequest},
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::app_state::AppState,
};

#[debug_handler]
pub async fn user_register_handler(
    State(AppState { grpc_factory, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    // ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<RegisterUserParam>,
) -> ApiResult<ApiResponse<RegisterResult>> {
    // 查询用户名是否已经存在
    let mut client = grpc_factory.create_client().await?;
    let username = UserExistsRequest {
        username: params.username.clone(),
    };
    let exists = client.user_exists(username).await?;
    // 如果用户名存在就返回
    if exists.into_inner().exists {
        return Err(ApiError::Biz(String::from("要注册的帐号已经存在!")));
    }
    // 转换成 UserRegisterRequest
    let register_request: UserRegisterRequest = params.into();
    let grpc_response = match client.user_register(register_request).await {
        Ok(response) => response.into_inner(),
        Err(status) => {
            tracing::error!("grpc error: {:?}", status);
            return Err(ApiError::GrpcError(status));
        }
    };
    Ok(ApiResponse::success(RegisterResult {
        result: grpc_response.result,
    }))
}
