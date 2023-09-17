Automated trading system.
        
                            ┌────────────────┐
                    ┌───────┤       Me       │
                    │       └─┬────────────┬─┘
                    │         │            │
                Pinescript    │            │
                    &         │            │
                 webhooks     │     logs/info/stats
                    │         │            │
                    │     Strategies       │
                    │     management       │
                    │         │            │
                    │         │            │
                    │         ▼            ▼
                    │     ┌────────────────────┐
                    │     │ Automated trading  │
                    │     │       system       │
                    │     └────────────────────┘
                    │         ▲            ▲
                    │         │            │
                    │      Webhook        API
                    │       alerts    interaction
                    │         │            │
                    │         │            │
      ┌───────────────────────│────────────│────────────────────┐
      │ External    │         │            │                    │
      │             ▼         │            │                    │
      │          ┌───────────────┐   ┌──────────────┐           │
      │          │  TradingView  │   │    Broker    │           │
      │          └───────────────┘   └──────────────┘           │
      │                                                         │
      └─────────────────────────────────────────────────────────┘
Complete C4 model - https://app.icepanel.io/landscapes/dqpAzgBtZpGxcn7sQvJo/versions/latest/diagrams
        
Webhook alert contains a strategy ID, each strategy contains information about the broker.
The choice of which broker to use is made at the strategy level.
Strategies are defined in `config/default.toml`
