# stooq-mcp

[English](README.md)

Rust で実装された MCP (Model Context Protocol) サーバーです。[stooq.com](https://stooq.com) から株価データを取得します。

## 機能

- **最新株価取得** - リアルタイムの株価データを取得
- **履歴データ取得** - 日付範囲を指定して過去の株価データを取得
- **複数市場対応** - 5つの主要市場の株式にアクセス可能

## 対応市場

| 市場コード | 国名 |
|------------|------|
| `jp` | 日本 |
| `us` | アメリカ |
| `uk` | イギリス |
| `hk` | 香港 |
| `de` | ドイツ |

## インストール

### 前提条件

- Rust 1.75+
- Cargo

### ワンライナーインストール

```bash
curl -fsSL https://raw.githubusercontent.com/hoqqun/stooq-mcp/main/install.sh | bash
```

`~/.stooq-mcp/` にインストールされます。カスタマイズする場合：

```bash
STOOQ_MCP_DIR=/your/path curl -fsSL https://raw.githubusercontent.com/hoqqun/stooq-mcp/main/install.sh | bash
```

### クイックインストール

```bash
git clone https://github.com/hoqqun/stooq-mcp.git
cd stooq-mcp
./install.sh
```

インストールスクリプトは以下を実行します：
1. プロジェクトのビルド
2. Claude Code / Claude Desktop への登録

### 手動ビルド

```bash
cargo build --release
```

バイナリは `./target/release/stooq-mcp` に生成されます。

## 使い方

### Claude Code

Claude Code に MCP サーバーを登録します：

```bash
claude mcp add stooq-mcp /path/to/stooq-mcp/target/release/stooq-mcp
```

### Claude Desktop

Claude Desktop の設定ファイルに以下を追加します：

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

## ツール

### `get_stock_price`

stooq.com から株価データを取得します。

#### パラメータ

| パラメータ | 型 | 必須 | 説明 |
|-----------|------|------|------|
| `ticker` | string | ✅ | 銘柄コード（例: `"7203"`, `"AAPL"`） |
| `market` | string | ✅ | 市場コード: `jp`, `us`, `uk`, `hk`, `de` |
| `start_date` | string | ❌ | 開始日 `YYYYMMDD` 形式（例: `"20240101"`） |
| `end_date` | string | ❌ | 終了日 `YYYYMMDD` 形式（例: `"20241231"`） |

## 使用例

### 最新株価を取得

**トヨタ自動車（日本）:**
```
トヨタ（7203）の現在の株価を取得して。
```

**Apple（アメリカ）:**
```
AAPLの最新株価を教えて。
```

### 履歴データを取得

**ソニー（日本）- 2024年通年:**
```
ソニー（6758）の2024年1月1日から12月31日までの株価履歴を取得して。
```

### レスポンス形式

**最新データ:**
```csv
Symbol,Date,Time,Open,High,Low,Close,Volume
7203.JP,2024-12-27,16:00:00,2500,2520,2480,2510,1000000
```

**履歴データ:**
```csv
Date,Open,High,Low,Close,Volume
2024-01-04,2450,2480,2440,2470,800000
2024-01-05,2470,2490,2460,2485,750000
...
```

## 技術スタック

- **Rust** - システムプログラミング言語
- **[rmcp](https://crates.io/crates/rmcp)** - MCP サーバー実装
- **[reqwest](https://crates.io/crates/reqwest)** - HTTP クライアント
- **[tokio](https://crates.io/crates/tokio)** - 非同期ランタイム

## 制限事項

> ⚠️ **重要なお知らせ**

- stooq.com は**非公式 API** であり、予告なく変更される可能性があります
- **日次レート制限**があります - 過度なリクエストはブロックされる可能性があります
- **個人利用・学習目的**のみを想定しています
- データの正確性は保証されません

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) を参照してください。

## コントリビューション

コントリビューションは大歓迎です！お気軽にプルリクエストをお送りください。
