![Automated trading system Diagram 1 (Current)](https://github.com/user-attachments/assets/ef392b17-16e3-4862-ae08-1b40d8a1e80d)

# SignalFlow - Automated Trading System

A high-performance automated trading system built with Rust, designed to execute trading strategies based on webhook alerts from TradingView or other sources.

## Features

- **Webhook Alert Processing**: Receive and process trading signals via HTTP webhooks
- **Multiple Broker Support**: Currently supports Alpaca with extensible architecture for additional brokers
- **Strategy Management**: Configure and manage multiple trading strategies with different parameters
- **Order Management**: Automated order placement with retry logic and error handling
- **Asset Support**: Trade both crypto and stock assets
- **RESTful API**: Complete API for account management, positions, orders, and activities
- **PostgreSQL Integration**: Persistent storage for alerts, strategies, and trading history
- **Docker Support**: Easy deployment with Docker and Docker Compose

## Architecture

The system is built as a modular Rust application with the following components:

- **API Layer**: Axum-based HTTP server handling webhooks and REST endpoints
- **Core Trading Engine**: Processes trade signals and manages order execution
- **Broker Clients**: Abstracted broker interfaces for easy integration
- **Database Layer**: SQLx-based PostgreSQL integration with migrations
- **Configuration**: Flexible configuration system using environment variables

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- PostgreSQL 12 or higher
- Docker and Docker Compose (optional)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/market.git
cd market
```

2. Set up environment variables:
```bash
cp .env.example .env
# Edit .env with your broker API credentials and database settings
```

3. Run database migrations:
```bash
sqlx migrate run
```

4. Build and run the application:
```bash
cargo run --bin market
```

### Docker Deployment

Deploy the entire stack using Docker Compose:

```bash
docker-compose up -d
```

This will start:
- The Market trading system on port 80
- PostgreSQL database on port 5432

## Configuration

The application uses a configuration file and environment variables. Key settings include:

- **Broker Configuration**: API keys and endpoints
- **Database Settings**: PostgreSQL connection parameters
- **Strategy Parameters**: Max retries, retry delays, enabled strategies
- **Server Settings**: Port and host bindings

## API Endpoints

### Webhook
- `POST /webhook` - Receive trading alerts

### Account Management
- `GET /account` - Get account information
- `GET /assets` - List available assets
- `GET /watchlists` - Get watchlists

### Trading
- `GET /positions` - List current positions
- `GET /orders` - List orders with filtering
- `GET /activities` - Get account activities

### Strategy Management
- Strategy endpoints for CRUD operations (implementation in progress)

## Development

### Project Structure

```
market/
├── market/           # Main trading application
│   ├── src/
│   │   ├── api/     # HTTP API handlers and objects
│   │   ├── clients.rs # Broker client implementations
│   │   ├── core.rs   # Core trading logic
│   │   ├── strategy.rs # Strategy definitions
│   │   └── main.rs   # Application entry point
│   └── migrations/   # Database migrations
├── m-cli/           # CLI tool (in development)
├── docker-compose.yml
└── Dockerfile
```

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is proprietary software. All rights reserved.
