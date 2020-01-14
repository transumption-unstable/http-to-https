use failure::*;

use hyper::header::{HOST, LOCATION};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

use tracing::*;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

async fn handle(req: Request<Body>) -> Fallible<Response<Body>> {
    let host = req
        .headers()
        .get(HOST)
        .ok_or(err_msg("Host header is missing"))?
        .to_str()?;

    let resp = Response::builder()
        .header(LOCATION, format!("https://{}{}", host, req.uri()))
        .status(StatusCode::MOVED_PERMANENTLY)
        .body(Body::empty())?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Fallible<()> {
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let addr = "[::]:80".parse()?;
    let make_service = make_service_fn(|_| async { Ok::<_, Error>(service_fn(handle)) });
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        error!("{}", e);
    }

    Ok(())
}
