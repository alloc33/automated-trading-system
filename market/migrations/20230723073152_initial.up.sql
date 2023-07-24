CREATE TABLE trading_alerts
(
	trading_alert_id Uuid,
  	created_at        Timestamptz NOT NULL,
  	modified_at       Timestamptz NOT NULL,

  	PRIMARY KEY (trading_alert_id)
)
