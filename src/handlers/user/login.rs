use axum::{debug_handler, extract::State};

use crate::{
    common::valid::ValidJson,
    handlers::common::model::LoginUserParam,
    pb::user::{UserLoginRequest, UserLoginResponse},
    response::{ApiResult, errors::ApiError, resp::ApiResponse},
    state::app_state::AppState,
};

#[debug_handler]
pub async fn user_login_handler(
    State(AppState { grpc_factory, .. }): State<AppState>,
    // Extension(_principal): Extension<Principal>,
    // ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<LoginUserParam>,
) -> ApiResult<ApiResponse<UserLoginResponse>> {
    let user_login_request: UserLoginRequest = params.into();
    // 查询用户名是否已经存在
    let mut client = grpc_factory.create_client().await?;
    let grpc_response = match client.user_login(user_login_request).await {
        Ok(response) => response.into_inner(),
        Err(status) => {
            tracing::error!("grpc error: {:?}", status);
            return Err(ApiError::GrpcError(status));
        }
    };
    Ok(ApiResponse::success(grpc_response))
}
