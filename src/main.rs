use rmcp::{
    ErrorData as McpError, ServerHandler, ServiceExt, handler::server::tool::ToolRouter, model::{CallToolResult, Content, ServerCapabilities, ServerInfo}, schemars::JsonSchema, serde::Deserialize, tool_handler, tool_router
};
use anyhow::Result;

#[derive(Clone)]
struct MyServer {
    tool_router: ToolRouter<Self>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
enum Market {
    Jp,  // Japan
    Us,  // USA
    Uk,  // United Kingdom
    Hk,  // Hong Kong
    De,  // Germany
}


#[derive(Debug, Deserialize, JsonSchema)]
struct StockParams {
    ticker: String,  // Ticker（Example: "7203"）
    market: Market,  // Market（Example "jp"）
    #[schemars(regex(pattern = r"^\d{8}$"), description = "Start date in YYYYMMDD format (e.g. 20240101)")]
    start_date: Option<String>,  // "20240101"
    #[schemars(regex(pattern = r"^\d{8}$"), description = "End date in YYYYMMDD format (e.g. 20241231)")]
    end_date: Option<String>,    // "20241231"
}    

#[tool_router]
impl MyServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[rmcp::tool(name = "get_stock_price", description = "Get stock price (market: jp or us)")]
    async fn get_stock_price(&self, params: rmcp::handler::server::wrapper::Parameters<StockParams>) -> Result<CallToolResult, McpError> {
        let ticker = &params.0.ticker;
        let market_str = match params.0.market {
            Market::Jp => "jp",
            Market::Us => "us",
            Market::Uk => "uk",
            Market::Hk => "hk",
            Market::De => "de",
        };

        let url = match (&params.0.start_date, &params.0.end_date) {
            (Some(start), Some(end)) => {
                // 履歴データ
                format!("https://stooq.com/q/d/l/?s={}.{}&d1={}&d2={}&i=d", ticker, market_str, start, end)
            }
            _ => {
                // 最新データ
                format!("https://stooq.com/q/l/?s={}.{}&f=sd2t2ohlcv&h&e=csv", ticker, market_str)
            }
        };
        
        let response = reqwest::get(&url).await;
        
        match response {
            Ok(res) => {
                let text = res.text().await.unwrap_or_else(|_| "Failed to read response".to_string());
                Ok(CallToolResult::success(vec![Content::text(text)]))
            }
            Err(e) => {
                Ok(CallToolResult::success(vec![Content::text(format!("Error: {}", e))]))
            }
        }
    } 
}

#[tool_handler]
impl ServerHandler for MyServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Tsubo Square Meter Converter".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let server = MyServer::new();
    let transport = rmcp::transport::io::stdio();
    let service = server.serve(transport).await?;
    service.waiting().await?;
    Ok(())
}