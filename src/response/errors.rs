use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    response::IntoResponse,
};
use tonic::{Code, Status};

use crate::response::resp::ApiResponse;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{0}")]
    Biz(String),
    #[error("没找到！")]
    NotFound,
    #[error("方法不支持！")]
    MethodNotAllowed,
    #[error("数据库错误: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("服务端错误！")]
    InternalServerError,
    #[error("参数校验失败：{0}")]
    ValidationError(String),
    #[error("尚未授权：{0}")]
    Unauthenticated(String),
    #[error("无效的 JSON 数据: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("查询参数错误: {0}")]
    QueryError(#[from] QueryRejection),
    #[error("路径参数错误: {0}")]
    PathError(#[from] PathRejection),
    #[error("Body 参数错误: {0}")]
    JsonError(#[from] JsonRejection),
    #[error("密码加密时出错：{0}")]
    Argon2HashingError(#[from] argon2::password_hash::Error),
    #[error("密码加密时出错：{0}")]
    Argon2HashingPHCError(#[from] argon2::password_hash::phc::Error),
    #[error("gRPC 调用错误：{0}")]
    GrpcError(Status), // 移除了 #[from] 属性，因为我们要自定义转换
}

impl ApiError {
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            ApiError::Biz(_) => axum::http::StatusCode::OK,
            ApiError::NotFound => axum::http::StatusCode::NOT_FOUND,
            ApiError::MethodNotAllowed => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            ApiError::Unauthenticated(_) => axum::http::StatusCode::UNAUTHORIZED,
            ApiError::InternalServerError
            | ApiError::ValidationError(_)
            | ApiError::QueryError(_)
            | ApiError::PathError(_)
            | ApiError::JsonError(_)
            | ApiError::InvalidJson(_) => axum::http::StatusCode::BAD_REQUEST,
            ApiError::DatabaseError(_)
            | ApiError::Argon2HashingError(_)
            | ApiError::Argon2HashingPHCError(_)
            | ApiError::GrpcError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

/// 将错误转换为响应
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status_code(),
            axum::Json(ApiResponse::<()>::err(self.to_string())),
        )
            .into_response()
    }
}

/// 把错误转换成返回值
impl From<ApiError> for axum::http::Response<axum::body::Body> {
    fn from(error: ApiError) -> Self {
        error.into_response()
    }
}

/// 为 ApiError 实现转换为校验失败的 trait
impl From<axum_valid::ValidRejection<ApiError>> for ApiError {
    fn from(value: axum_valid::ValidRejection<ApiError>) -> Self {
        match value {
            axum_valid::ValidationRejection::Valid(errors) => {
                ApiError::ValidationError(errors.to_string())
            }
            axum_valid::ValidationRejection::Inner(error) => error,
        }
    }
}

/// 从 tonic::Status 转换为 ApiError
impl From<Status> for ApiError {
    fn from(value: Status) -> Self {
        // 根据 Status 的 code 映射到不同的 ApiError
        match value.code() {
            Code::NotFound => ApiError::NotFound,
            Code::InvalidArgument => ApiError::ValidationError(value.message().to_string()),
            Code::Unauthenticated => ApiError::Unauthenticated(value.message().to_string()),
            Code::PermissionDenied => ApiError::Unauthenticated(value.message().to_string()),
            Code::AlreadyExists => ApiError::ValidationError(value.message().to_string()),
            Code::FailedPrecondition => ApiError::ValidationError(value.message().to_string()),
            Code::OutOfRange => ApiError::ValidationError(value.message().to_string()),
            Code::Unimplemented => ApiError::MethodNotAllowed,

            // 其他错误都视为内部服务器错误
            _ => ApiError::InternalServerError,
        }
    }
}
