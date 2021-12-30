-- _type: 0: unknown, 1: offensive 辱骂/攻击/冒犯 2: ad 广告 3: spam 垃圾信息 4: porn 色情低俗 5: politics 政治相关 6: illegal 违法违规 7: leak 泄漏他人隐私 8: violate 侵犯我的权益, 9: complaint 其他投诉 80: feedback bug反馈,功能建议, 81: ask 咨询 99: other 其他

-- state: 0: open, 1: closed
ALTER TABLE reports
  DROP COLUMN IF EXISTS report_type,
  ADD COLUMN _type smallint NOT NULL DEFAULT 0,
  DROP COLUMN action_taken,
  ADD COLUMN state smallint NOT NULL DEFAULT 0,
  DROP COLUMN target_account_id,
  ADD COLUMN related_post_id bigint,
  ADD COLUMN related_account_id bigint,
  DROP COLUMN action_comment,
  ADD replied_content text,
  DROP COLUMN action_taken_by_account_id,
  ADD replied_by bigint,
  ADD replied_at timestamp;