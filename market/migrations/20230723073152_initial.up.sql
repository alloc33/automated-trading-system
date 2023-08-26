CREATE TABLE alerts
(
	alert_id  Uuid,
	ticker			  Text NOT NULL,
	timeframe		  Text NOT NULL,
	exchange		  Text NOT NULL,
	alert_type        Text NOT NULL,
	bar_time          Timestamptz NOT NULL,
	bar_open		  Decimal(16, 2) NOT NULL,
	bar_high		  Decimal(16, 2) NOT NULL,
	bar_low			  Decimal(16, 2) NOT NULL,
	bar_close		  Decimal(16, 2) NOT NULL,
	bar_volume		  Decimal(16, 2) NOT NULL,
	alert_fire_time   Timestamptz NOT NULL,
  	created_at        Timestamptz NOT NULL,
  	modified_at       Timestamptz NOT NULL,

  	PRIMARY KEY (alert_id)
);

CREATE INDEX idx_alerts_ticker ON alerts (ticker);
CREATE INDEX idx_alerts_alert_type ON alerts (alert_type);
