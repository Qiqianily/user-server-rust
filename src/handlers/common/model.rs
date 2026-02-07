use crate::pb::user::{UserLoginRequest, UserRegisterRequest};

/// 定义注册用户参数
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, validator::Validate)]
pub struct RegisterUserParam {
    #[validate(length(min = 2, max = 20, message = "用户名长度必须在 2-20 之间"))]
    pub username: String,
    #[validate(length(min = 6, max = 20, message = "密码长度必须在 6-20 之间"))]
    pub password: String,
}

/// 定义登录用户参数
#[derive(Debug, serde::Deserialize, Clone, validator::Validate)]
pub struct LoginUserParam {
    #[validate(length(min = 2, max = 20, message = "用户名长度必须在 2-20 之间"))]
    pub username: String,
    #[validate(length(min = 6, max = 20, message = "密码长度必须在 6-20 之间"))]
    pub password: String,
}

impl From<RegisterUserParam> for UserRegisterRequest {
    fn from(value: RegisterUserParam) -> Self {
        UserRegisterRequest {
            username: value.username,
            password: value.password,
        }
    }
}

impl From<LoginUserParam> for UserLoginRequest {
    fn from(value: LoginUserParam) -> Self {
        UserLoginRequest {
            username: value.username,
            password: value.password,
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    pub access_token: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterResult {
    pub result: String,
}
