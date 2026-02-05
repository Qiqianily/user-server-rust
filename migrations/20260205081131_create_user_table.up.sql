-- Add up migration script here
-- 创建枚举类型
CREATE TYPE user_level AS ENUM ('guest', 'member', 'vip', 'admin');

-- 创建 user 表
CREATE TABLE IF NOT EXISTS "user" (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(100) NULL,
    is_open BOOLEAN NOT NULL DEFAULT TRUE,
    level user_level NOT NULL DEFAULT 'member',
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMPTZ NULL
);

-- 添加表注释
COMMENT ON TABLE "user" IS '用户表';

-- 添加列注释
COMMENT ON COLUMN "user".id IS '用户ID';
COMMENT ON COLUMN "user".username IS '用户名';
COMMENT ON COLUMN "user".password IS '密码';
COMMENT ON COLUMN "user".email IS '邮箱';
COMMENT ON COLUMN "user".is_open IS '是否开放/启用：true-是，false-否';
COMMENT ON COLUMN "user".level IS '用户等级：guest, member, vip, admin';
COMMENT ON COLUMN "user".created_at IS '创建时间';
COMMENT ON COLUMN "user".last_login IS '最后登录时间';

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_user_email ON "user"(email);
CREATE INDEX IF NOT EXISTS idx_user_level ON "user"(level);
CREATE INDEX IF NOT EXISTS idx_user_created_at ON "user"(created_at);
