# Contribution

## 环境安装

### MacOS

```bash
# 安装数据库
brew install postgres
# 安装postgres数据库扩展
brew install postgis
# 安装redis数据库 6.0+版本
brew install redis
# 安装后端语言rust的管理工具 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


```

### Debian

1. [Debian Server Setup](https://wiki.owenyoung.com/debian-server-setup/)
2. 安装 postgres 数据库扩展

> 参考： [Postgres Setup](https://wiki.owenyoung.com/postgres-setup-for-debian), [postgis setup](https://trac.osgeo.org/postgis/wiki/UsersWikiPostGIS24UbuntuPGSQL10Apt)

```bash
sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
sudo apt-get update
sudo apt -y install postgresql-14 postgresql-14-postgis-3 postgresql-14-postgis-3-scripts postgresql-14-postgis-3-dbgsym
```

3. [Rust Enviroment setup](https://wiki.owenyoung.com/rust-enviroment-setup-for-debian/)

```bash
# 安装redis数据库
# 6.0+版本
curl https://packages.redis.io/gpg | sudo apt-key add -
echo "deb https://packages.redis.io/deb $(lsb_release -cs) main" | sudo tee /etc/apt/sources.list.d/redis.list
sudo apt-get update
sudo apt-get install redis
# 登录postgres用户
sudo su - postgres
# 进行psql命令行
psql

# 创建数据库扩展
CREATE EXTENSION IF NOT EXISTS postgis;
# 创建数据库
CREATE DATABASE chat;
# 创建postgres用户
create user chat_postgres with encrypted password 'chat_postgres_password';
# 授权给用户刚创建的数据库的权限
grant all privileges on database chat to chat_postgres;
# 退出到当前用户
exit;
exit
```

## 初始化

初始化 rust 开发编译环境

```bash
# 使用rust nightly版本
rustup default nightly
# 安装开发自动刷新工具 cargo-watch
cargo install cargo-watch
```

生产环境`.env`参考:

```ini
DATABASE_URL=postgres://chat_postgres:chat_postgres_password@localhost/chat
RUST_ENV=prod
```

```bash
# 下载源代码
git clone git@github.com:susuwoniu/chat-server.git
# 和同事获取开发环境的的 .env 文件，粘贴到根目录，把.env里的数据库地址改成你本地对应的地址
# 比如 DATABASE_URL=postgres://chat_postgres:chat_postgres_password@localhost/chat
# or
# cp sample.env .env
# 改变 .env里的值

# 拷贝配置文件，拷贝 config/default.toml, config/dev.toml, config/prod.toml


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

## 创建 IM 的管理员账户

```bash
make create-admin
```

## 部署 IM 服务

See <https://github.com/susuwoniu/chat-im-server>

把刚生成的管理员 id 添加到 `chat-im-server`的配置文件里的`manager->appManagerUid`中,以及添加以下内容到`chat-server`的配置文件:

```bash
[im]
admin_account_id = <admin_account_id>
```

> 参考 <https://github.com/OpenIMSDK/Open-IM-Server>

## 开发

```bash
make start
```

## 生产环境

```bash
make build
```

### Setup as system service

You have to create a `chat.service` file in `/etc/systemd/system` that would contain the following text:

```bash
sudo vim /etc/systemd/system/chat.service
```

```bash
[Unit]
Description=Chat Daemon
After=syslog.target network.target

# After=syslog.target network.target sonarr.service radarr.service

[Service]
WorkingDirectory=/home/green/chat-server
User=green
Group=admin
UMask=0002
Restart=on-failure
RestartSec=5
Type=simple
ExecStart=/home/green/chat-server/target/release/chat server start
KillSignal=SIGINT
TimeoutStopSec=20
SyslogIdentifier=chat
[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now chat
sudo systemctl status chat
sudo systemctl restart chat

```

## Install nginx as reverse proxy

See <https://wiki.owenyoung.com/nginx-setup-for-debian/>

### Nginx conf

See [online conf](https://www.digitalocean.com/community/tools/nginx?domains.0.server.domain=chat.scuinfo.com&domains.0.https.certType=custom&domains.0.php.php=false&domains.0.reverseProxy.reverseProxy=true&domains.0.routing.root=false&global.app.lang=zhCN) generate

Or See `supports/nginx`

Generate `dhparam.pem`:

```bash
openssl dhparam -out /etc/nginx/dhparam.pem 2048
```

## Generate SSL

```
acme.sh --issue --dns dns_cf -d chat.scuinfo.com
```

```
acme.sh --install-cert -d chat.scuinfo.com \
--key-file       /etc/nginx/ssl/chat.scuinfo.com.key  \
--fullchain-file /etc/nginx/ssl/chat.scuinfo.com.crt \
--reloadcmd     "service nginx force-reload"
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
