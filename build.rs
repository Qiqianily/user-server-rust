fn main() -> anyhow::Result<()> {
    // 如果目录不存在则创建
    std::fs::create_dir_all("src/pb")?;
    let build = tonic_prost_build::configure()
        .type_attribute(
            "user.UserLoginResponse",
            r#"
            #[derive(
                serde::Serialize,
                serde::Deserialize
            )]
            #[serde(rename_all = "camelCase")]
            "#,
        )
        .type_attribute(
            "user.UserExistsResponse",
            r#"
            #[derive(sqlx::FromRow)]
            "#,
        );
    build
        .out_dir("src/pb")
        .compile_protos(&["proto/user/user.proto"], &["proto"])?;
    Ok(())
}
