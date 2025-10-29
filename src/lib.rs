use anyhow::{Context, Result};
use pyo3::prelude::*;
use rmcp::{
    ErrorData as McpError, ServiceExt, handler::server::router::tool::ToolRouter,
    handler::server::wrapper::Parameters, model::*, schemars, serde, tool, tool_handler,
    tool_router, transport::stdio,
};

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct GreetRequest {
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

    #[tool(description = "Greet")]
    fn greet(&self, Parameters(GreetRequest { name }): Parameters<GreetRequest>) -> String {
        format!("Hello, {}!", name)
    }

    #[tool(description = "Meet")]
    fn meet(
        &self,
        Parameters(GreetRequest { name }): Parameters<GreetRequest>,
    ) -> Result<CallToolResult, McpError> {
        if name.len() < 10 {
            Ok(CallToolResult::error(vec![Content::text(
                "Too short",
            )]))
        } else {
            let greeting = format!("Hello, {}!", name);
            Ok(CallToolResult::success(vec![Content::text(greeting)]))
        }
    }
}

#[tool_handler]
impl rmcp::ServerHandler for GreetService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some("Greeting tools".into()),
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
            .context("Server start failed")?;

        service
            .waiting()
            .await
            .context("Server failed")?;

        Ok(())
    })
}

#[pymodule]
fn greet_mcp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    Ok(())
}
