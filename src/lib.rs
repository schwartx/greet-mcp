use anyhow::{Context, Result};
use pyo3::prelude::*;
use rmcp::{
    ServiceExt, handler::server::router::tool::ToolRouter, handler::server::wrapper::Parameters,
    model::*, schemars, serde, tool, tool_handler, tool_router, transport::stdio,
};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GreetRequest {
    number: i32,
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

    /// greet tool
    ///
    /// Returns input number plus one.
    #[tool(description = "Return input number plus one")]
    fn greet(&self, Parameters(GreetRequest { number }): Parameters<GreetRequest>) -> String {
        (number + 1).to_string()
    }
}

// Implement the server handler
#[tool_handler]
impl rmcp::ServerHandler for GreetService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("A simple MCP service that returns a greeting message.".into()),
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
