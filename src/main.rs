use rmcp::{
    Error, ServerHandler, ServiceExt, model::CallToolResult, model::Content, model::Implementation,
    model::ProtocolVersion, model::ServerCapabilities, model::ServerInfo, tool, transport::stdio,
};
use std::error::Error as StdError;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let mcp_server = HelloWorld::new().serve(stdio()).await.inspect_err(|e| {
        println!("{e}");
    })?;

    mcp_server.waiting().await?;

    Ok(())
}

#[derive(Clone, Debug)]
pub struct HelloWorld {
    counter: Arc<Mutex<u32>>,
}
#[tool(tool_box)]
impl HelloWorld {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    #[tool(description = "Get the current counter value")]
    async fn get(&self) -> Result<CallToolResult, Error> {
        let counter = self.counter.lock().await;
        let result = CallToolResult::success(vec![Content::text(counter.to_string())]);

        Ok(result)
    }

    #[tool(description = "Increment the counter by 1")]
    async fn increment(&self) -> Result<CallToolResult, Error> {
        let mut counter = self.counter.lock().await;
        *counter += 1;
        let result = CallToolResult::success(vec![Content::text(counter.to_string())]);

        Ok(result)
    }

    #[tool(description = "Decrement the counter by 1")]
    async fn decrement(&self) -> Result<CallToolResult, Error> {
        let mut counter = self.counter.lock().await;
        *counter -= 1;
        let result = CallToolResult::success(vec![Content::text(counter.to_string())]);

        Ok(result)
    }

    #[tool(description = "HelloWorld message")]
    async fn message(&self) -> Result<CallToolResult, Error> {
        let msg = "Hello from MCP server".to_string();
        let result = CallToolResult::success(vec![Content::text(msg)]);
        Ok(result)
    }
}

impl Default for HelloWorld {
    fn default() -> Self {
        Self::new()
    }
}

#[tool(tool_box)]
impl ServerHandler for HelloWorld {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Example MCP server that provides a counter tool that can increment, decrement and provide values. Also, provides a message tool that gets a hello world message.".to_string(),
            ),
        }
    }
}
