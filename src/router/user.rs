use crate::{handlers, state::app_state::AppState};

/// 创建解析相关的路由，专门用来管理与原文解析相关的操作
pub fn create_user_router() -> axum::Router<AppState> {
    axum::Router::new().route(
        "/register",
        axum::routing::post(handlers::user::register::user_register_handler),
    )
}
