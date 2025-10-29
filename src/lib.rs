use anyhow::{Context, Result};
use pyo3::prelude::*;
use rmcp::{
    ErrorData as McpError, ServiceExt, handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::*, schemars, serde, tool, tool_handler,
    tool_router, transport::stdio,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GreetRequest {
    number: i32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct MeetRequest {
    name: String,
}

pub struct GreetService {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl GreetService {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Return input number plus one")]
    fn greet(&self, Parameters(GreetRequest { number }): Parameters<GreetRequest>) -> String {
        (number + 1).to_string()
    }

    #[tool(description = "Generate a personalized greeting for the given name")]
    fn meet(
        &self,
        Parameters(MeetRequest { name }): Parameters<MeetRequest>,
    ) -> Result<CallToolResult, McpError> {
        if name.len() < 10 {
            Ok(CallToolResult::error(vec![Content::text(
                "Name must be at least 10 characters long",
            )]))
        } else {
            let greeting = format!("Hello, {}! Nice to meet you!", name);
            Ok(CallToolResult::success(vec![Content::text(greeting)]))
        }
    }
}

// Implement the server handler
#[tool_handler]
impl rmcp::ServerHandler for GreetService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A simple MCP service that provides greeting and number manipulation tools. Includes 'greet' for incrementing numbers and 'meet' for generating personalized greetings.".into()),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[pyfunction]
fn start_server(py: Python) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async {
        let service = GreetService::new()
            .serve(stdio())
            .await
            .context("Failed to start RMCP server with stdio transport")?;

        service
            .waiting()
            .await
            .context("Server terminated unexpectedly while waiting")?;

        Ok(())
    })
}

#[pymodule]
fn greet_mcp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    Ok(())
}
