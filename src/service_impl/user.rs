use std::{ops::Deref, sync::Arc};

use sqlx::PgPool;
use tonic::{Request, Response, Status};

use crate::{
    pb::user::{
        UserExistsRequest, UserExistsResponse, UserLoginRequest, UserLoginResponse,
        UserRegisterRequest, UserRegisterResponse, user_service_server::UserService,
    },
    utils::crypto::encode_password,
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
        _request: Request<UserLoginRequest>,
    ) -> std::result::Result<Response<UserLoginResponse>, Status> {
        todo!()
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
