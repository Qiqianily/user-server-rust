use user_server::utils::crypto::{encode_password, verify_password};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let passworld = "123456";
    let hashed_password = encode_password(passworld)?;
    println!("Hashed password: {}", hashed_password);
    println!("len: {}", hashed_password.len());
    let verify = verify_password(passworld, &hashed_password)?;
    println!("Verification result: {}", verify);
    Ok(())
}
