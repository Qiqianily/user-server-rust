use user_server::middlewares::auth::{
    identity::Identity,
    jwt::{JWT, JwtConfig},
    principal::Principal,
};

fn main() -> anyhow::Result<()> {
    let principal = Principal {
        id: 123,
        username: "1234567890".to_string(),
        identity: Identity::Admin,
    };
    let jwt = JWT::new(JwtConfig::default());
    let token = jwt.encode(principal).unwrap();
    println!("access_token: {token}");
    let principal = jwt.decode(&token).unwrap();
    println!("principal: {principal:?}");
    Ok(())
}
