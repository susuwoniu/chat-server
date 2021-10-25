# Contribution

## Pre Requirement

MacOS

### 后端环境：

```bash
# 安装数据库
brew install postgres
# 安装postgres数据库扩展
brew install postgis
# 安装redis数据库
brew install redis
# 安装后端语言rust的管理工具 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 使用rust nightly版本
rustup default nightly
# 安装数据库管理工具
cargo install sqlx-cli --no-default-features --features postgres
```

## 初始化

```bash
# 和同事获取开发环境的的 .env 文件，粘贴到根目录
# or
# cp sample.env .env
# 改变 .env里的值
# 初始化数据库等
make init

# 生成access_token密钥对
make keygen

# 保存密码到配置文件 config/server-dev.toml or config/server-prod.toml
[auth]
secret_key = ""
public_key = ""


# 生成refresh_token密钥对
make keygen

# 保存密码到配置文件 config/server-dev.toml or config/server-prod.toml
[auth]
refresh_token_secret_key = ""
refresh_token_public_key = ""

# 生成一个客户端，用于后续所有的接口请求，以及部分接口的签名生成,理论上，ios一个，安卓1个

make client

# 保存client信息到配置文件 config/server-dev.toml or config/server-prod.toml

[[clients]]
client_id = 123456
client_secret = ""
name = "iPhone 客户端"

```

## 开发

```bash
make start
```

## 生产环境

```bash
make build && make serve
```

## Update database schema

```bash
sqlx migrate add <name>
```

or

```bash
make db name=<name>
```

## Upgrade

当前使用的 postgres 版本是 14，如果后续有升级的话，可以运行下面的命令升级：

```bash
brew postgresql-upgrade-database
```

### 客户端环境：

```bash
# 安装flutter
brew install --cask flutter
```
