Automated trading system.
        
                            ┌────────────────┐
                    ┌───────┤       Me       │
                    │       └─┬──────────────┘
                    │         │            ▲
                Pinescript    │            │
                    &         │            │
                 webhooks     │     logs/info/stats
                    │         │            │
                    │   1(2). │            │
                    │         │            │
                    │     Strategies       │
                    │     management       │
                    │         │         4. │
                    │         │            │
              1(1). │         ▼            │
                    │     ┌────────────────────┐
                    │     │ Automated trading  │
                    │     │       system       │
                    │     └────────────────────┘
                    │         ▲            │
                    │         │            │
                    │      Webhook        API
                    │       alerts    interaction
                    │         │            │
                    │      2. │            │
                    │         │         3. │
      ┌───────────────────────│────────────│──────────────────┐
      │ External    │         │            │                  │
      │             ▼         │            ▼                  │
      │          ┌───────────────┐   ┌────────────┐           │
      │          │  TradingView  │   │  Exchange  │           │
      │          └───────────────┘   └────────────┘           │
      │                                                       │
      └───────────────────────────────────────────────────────┘
Complete C4 model - https://app.icepanel.io/landscapes/dqpAzgBtZpGxcn7sQvJo/versions/latest/diagrams
        
Webhook alert contains a strategy ID, each strategy contains information about the exchange.
The choice of which exchange to use is made at the strategy level.
Strategies are defined in `config/default.toml`
