# Contribution


## Pre Requirement

MacOS

### 后端环境：

```bash
# 安装数据库
brew install postgres
# 安装数据库扩展
brew install postgis
# 安装后端语言rust的管理工具 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 使用rust nightly版本
rustup default nightly
# 安装数据库管理工具
cargo install sqlx-cli --no-default-features --features postgres
```

### 客户端环境：

```bash
# 安装flutter
brew install --cask flutter
```

## 初始化


```bash
# 和同事获取开发环境的的 .env 文件，粘贴到根目录
# or
# cp sample.env .env
# 改变 .env里的值
# 初始化数据库等
make init

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

当前使用的 postgres  版本是14，如果后续有升级的话，可以运行下面的命令升级：

```bash
brew postgresql-upgrade-database
```