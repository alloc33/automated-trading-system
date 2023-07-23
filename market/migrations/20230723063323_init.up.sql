-- Add up migration script here

CREATE TABLE users
(
	user_id Uuid,
	name    TEXT,

  	created_at      Timestamptz NOT NULL,
  	modified_at     Timestamptz NOT NULL,

  	PRIMARY KEY (user_id)
);
