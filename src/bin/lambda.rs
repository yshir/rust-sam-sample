use lambda_http::{
    handler,
    http::Method,
    lambda_runtime::{Context, Error},
    Body, Request, Response,
};
use rust_sam_sample::{app::App, handlers::ping};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = App::default();
    lambda_runtime::run(handler(|req: Request, ctx: Context| routes(&app, req, ctx))).await?;
    Ok(())
}

async fn routes(app: &App, req: Request, ctx: Context) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => ping(app, req, ctx).await,
        _ => Ok(not_found()),
    }
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(404)
        .header("Content-Type", "application/json")
        .body(Body::from(json!({"status": 404}).to_string()))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_http::IntoResponse;

    #[tokio::test]
    async fn test_ping() {
        let app = App::default();
        let req = Request::default();
        let ctx = Context::default();

        let res = routes(&app, req, ctx)
            .await
            .expect("Failed to request")
            .into_response();
        assert_eq!(res.status(), 200);
    }

    #[tokio::test]
    async fn test_404() {
        let app = App::default();
        let req = {
            let mut req = Request::default();
            *req.uri_mut() = "/foo".parse().unwrap();
            req
        };
        let ctx = Context::default();

        let res = routes(&app, req, ctx)
            .await
            .expect("Failed to request")
            .into_response();
        assert_eq!(res.status(), 404);
    }
}
