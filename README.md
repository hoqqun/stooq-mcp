# stooq-mcp

[日本語](README.ja.md)

A Model Context Protocol (MCP) server written in Rust that fetches stock price data from [stooq.com](https://stooq.com).

## Features

- **Latest stock prices** - Get real-time stock price data
- **Historical data** - Retrieve historical stock data with custom date ranges
- **Multi-market support** - Access stocks from 5 major markets

## Supported Markets

| Market Code | Country |
|-------------|---------|
| `jp` | Japan |
| `us` | United States |
| `uk` | United Kingdom |
| `hk` | Hong Kong |
| `de` | Germany |

## Installation

### Prerequisites

- Rust 1.75+
- Cargo

### One-liner Install

```bash
curl -fsSL https://raw.githubusercontent.com/hoqqun/stooq-mcp/main/install.sh | bash
```

This will install to `~/.stooq-mcp/`. To customize:

```bash
STOOQ_MCP_DIR=/your/path curl -fsSL https://raw.githubusercontent.com/hoqqun/stooq-mcp/main/install.sh | bash
```

### Quick Install

```bash
git clone https://github.com/hoqqun/stooq-mcp.git
cd stooq-mcp
./install.sh
```

The install script will:
1. Build the project
2. Register with Claude Code and/or Claude Desktop

### Manual Build

```bash
cargo build --release
```

The binary will be located at `./target/release/stooq-mcp`.

## Usage

### Claude Code

Register the MCP server with Claude Code:

```bash
claude mcp add stooq-mcp /path/to/stooq-mcp/target/release/stooq-mcp
```

### Claude Desktop

Add the following to your Claude Desktop configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`  
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`  
**Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "stooq-mcp": {
      "command": "/path/to/stooq-mcp/target/release/stooq-mcp"
    }
  }
}
```

## Tools

### `get_stock_price`

Fetches stock price data from stooq.com.

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `ticker` | string | ✅ | Stock ticker symbol (e.g., `"7203"`, `"AAPL"`) |
| `market` | string | ✅ | Market code: `jp`, `us`, `uk`, `hk`, `de` |
| `start_date` | string | ❌ | Start date in `YYYYMMDD` format (e.g., `"20240101"`) |
| `end_date` | string | ❌ | End date in `YYYYMMDD` format (e.g., `"20241231"`) |

## Examples

### Get latest stock price

**Toyota Motor (Japan):**
```
Get the current stock price for Toyota (7203) in Japan market.
```

**Apple (US):**
```
Get the latest AAPL stock price from US market.
```

### Get historical data

**Sony (Japan) - 2024 full year:**
```
Get Sony (6758) stock price history from January 1, 2024 to December 31, 2024.
```

### Response format

**Latest data:**
```csv
Symbol,Date,Time,Open,High,Low,Close,Volume
7203.JP,2024-12-27,16:00:00,2500,2520,2480,2510,1000000
```

**Historical data:**
```csv
Date,Open,High,Low,Close,Volume
2024-01-04,2450,2480,2440,2470,800000
2024-01-05,2470,2490,2460,2485,750000
...
```

## Tech Stack

- **Rust** - Systems programming language
- **[rmcp](https://crates.io/crates/rmcp)** - MCP server implementation
- **[reqwest](https://crates.io/crates/reqwest)** - HTTP client
- **[tokio](https://crates.io/crates/tokio)** - Async runtime

## Limitations

> ⚠️ **Important Notice**

- stooq.com is an **unofficial API** and may change without notice
- **Daily rate limits** apply - excessive requests may be blocked
- Intended for **personal use and learning purposes** only
- Data accuracy is not guaranteed

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
