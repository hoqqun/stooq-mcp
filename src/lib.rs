use rmcp::{
    ErrorData as McpError, ServerHandler, handler::server::tool::ToolRouter, model::{CallToolResult, Content, ServerCapabilities, ServerInfo}, schemars::JsonSchema, serde::Deserialize, tool_handler, tool_router
};

#[derive(Clone)]
pub struct StooqServer {
    tool_router: ToolRouter<Self>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Market {
    Jp,  // Japan
    Us,  // USA
    Uk,  // United Kingdom
    Hk,  // Hong Kong
    De,  // Germany
}

impl Market {
    pub fn as_str(&self) -> &'static str {
        match self {
            Market::Jp => "jp",
            Market::Us => "us",
            Market::Uk => "uk",
            Market::Hk => "hk",
            Market::De => "de",
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StockParams {
    pub ticker: String,
    pub market: Market,
    #[schemars(regex(pattern = r"^\d{8}$"), description = "Start date in YYYYMMDD format (e.g. 20240101)")]
    pub start_date: Option<String>,
    #[schemars(regex(pattern = r"^\d{8}$"), description = "End date in YYYYMMDD format (e.g. 20241231)")]
    pub end_date: Option<String>,
}

/// Build stooq URL for stock data
pub fn build_stooq_url(ticker: &str, market: &Market, start_date: Option<&str>, end_date: Option<&str>) -> String {
    let market_str = market.as_str();
    match (start_date, end_date) {
        (Some(start), Some(end)) => {
            // Historical data
            format!("https://stooq.com/q/d/l/?s={}.{}&d1={}&d2={}&i=d", ticker, market_str, start, end)
        }
        _ => {
            // Latest data
            format!("https://stooq.com/q/l/?s={}.{}&f=sd2t2ohlcv&h&e=csv", ticker, market_str)
        }
    }
}

#[tool_router]
impl StooqServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[rmcp::tool(name = "get_stock_price", description = "Get stock price (market: jp or us)")]
    async fn get_stock_price(&self, params: rmcp::handler::server::wrapper::Parameters<StockParams>) -> Result<CallToolResult, McpError> {
        let ticker = &params.0.ticker;
        let market = &params.0.market;

        let url = build_stooq_url(
            ticker,
            market,
            params.0.start_date.as_deref(),
            params.0.end_date.as_deref(),
        );
        
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

impl Default for StooqServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler]
impl ServerHandler for StooqServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Stooq Stock Price MCP Server".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests;
