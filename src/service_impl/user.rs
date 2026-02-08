use std::{ops::Deref, sync::Arc};

use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::{
    middlewares::auth::{identity::Identity, jwt::get_default_jwt, principal::Principal},
    pb::user::{
        UserExistsRequest, UserExistsResponse, UserLoginRequest, UserLoginResponse,
        UserRegisterRequest, UserRegisterResponse, user_service_server::UserService,
    },
    utils::crypto::{encode_password, verify_password},
};

/// 内部数据状态
#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub pool: &'static PgPool,
}

// 实现 UserService trait
#[derive(Debug)]
pub struct UserServiceImpl {
    // 这里面可以放数据库连接池
    pub inner: Arc<AppStateInner>,
}

impl UserServiceImpl {
    pub fn new(pool: &'static PgPool) -> Self {
        Self {
            inner: Arc::new(AppStateInner { pool }),
        }
    }
}
#[derive(Debug, sqlx::FromRow, Clone)]
pub struct UserLoginInfo {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub is_open: bool,
    pub level: Identity,
}

/// 实现解引用操作
impl Deref for UserServiceImpl {
    type Target = AppStateInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn user_login(
        &self,
        request: Request<UserLoginRequest>,
    ) -> std::result::Result<Response<UserLoginResponse>, Status> {
        let user_info_request = request.into_inner();
        // 1. 查询用户信息
        let user_info = sqlx::query_as::<_, UserLoginInfo>(
            r#"SELECT id, username, password, is_open, level FROM "user" WHERE username = $1"#,
        )
        .bind(user_info_request.username)
        .fetch_one(self.inner.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Status::unauthenticated("帐号或密码不正确！"),
            _ => {
                tracing::error!("查询用户失败: {:?}", e);
                Status::internal("服务器内部错误")
            }
        })?;
        // 2. 检查用户状态（如是否被禁用）
        if !user_info.is_open {
            return Err(Status::permission_denied("该账号已被禁用，请联系管理员！"));
        }
        // 3. 验证密码
        if !verify_password(&user_info_request.password, &user_info.password)
            .map_err(|e| Status::internal(format!("Failed to verify password: {}", e)))?
        {
            return Err(Status::unauthenticated("帐号或密码不正确！"));
        }
        // 4. 构建 principal
        let principal = Principal {
            id: user_info.id,
            username: user_info.username.clone(),
            identity: user_info.level,
        };
        // 5. 生成 access_token
        let access_token = get_default_jwt()
            .encode(principal)
            .map_err(|e| Status::internal(format!("Failed to encode JWT: {}", e)))?;
        // 6. 返回 token
        Ok(Response::new(UserLoginResponse { access_token }))
    }
    async fn user_register(
        &self,
        request: Request<UserRegisterRequest>,
    ) -> std::result::Result<Response<UserRegisterResponse>, Status> {
        let user_info = request.into_inner();
        let hash_password = encode_password(&user_info.password).unwrap();
        let id: i32 = sqlx::query_scalar(
            r#"INSERT INTO "user" (username, password) VALUES ($1, $2) RETURNING id"#,
        )
        .bind(&user_info.username)
        .bind(&hash_password)
        .fetch_one(self.inner.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(UserRegisterResponse {
            result: format!("{} 创建成功！id: {}", user_info.username, id),
        }))
    }
    async fn user_exists(
        &self,
        request: Request<UserExistsRequest>,
    ) -> std::result::Result<Response<UserExistsResponse>, Status> {
        let user_name = request.into_inner().username;
        // 查询数据库是否存在
        let result = sqlx::query_as::<_, UserExistsResponse>(
            r#"SELECT EXISTS(SELECT 1 FROM "user" WHERE username = $1) as exists"#,
        )
        .bind(user_name)
        .fetch_one(self.inner.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(result))
    }
}
