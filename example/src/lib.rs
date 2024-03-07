use spin_sdk::http::{Params, Request, Response, Router, IntoResponse};
use spin_sdk::http_component;
use anyhow::Result;
use serde_json::json;

use bindings::arithmetic::overflow::overflowing_add::{overflowing_add, Arguments};
mod bindings;

/// A simple Spin HTTP component.
#[http_component]
fn serve(req: Request) -> Response {
    let mut router = Router::new();
    router.post("/add", handle_add);
    router.handle(req)
}

fn handle_add(req: Request, _params: Params) -> Result<impl IntoResponse> {
    let args: Arguments = serde_json::from_slice(req.body())?;
    let (value, overflow) = overflowing_add(args);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(json!({
            "value": value, 
            "overflow": overflow
        }).to_string())
        .build())
}