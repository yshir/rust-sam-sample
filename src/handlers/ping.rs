use lambda_http::{Body, Context, Request, Response};
use lambda_runtime::Error;
use serde_json::json;

use crate::app::App;

pub async fn ping(_: &App, _: Request, _: Context) -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::from(json!({"ok": true}).to_string()))
        .unwrap())
}
