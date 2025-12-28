use stooq_mcp::StooqServer;
use rmcp::ServiceExt;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let server = StooqServer::new();
    let transport = rmcp::transport::io::stdio();
    let service = server.serve(transport).await?;
    service.waiting().await?;
    Ok(())
}