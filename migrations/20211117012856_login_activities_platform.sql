-- Add migration script here
CREATE TYPE client_platform AS ENUM ('iOS','Android', 'Web','Windows','macOS','Linux','WechatMini');
ALTER TABLE login_activities
  ADD client_platform client_platform NOT NULL DEFAULT 'iOS';