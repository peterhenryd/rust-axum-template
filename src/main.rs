use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use dotenvy::dotenv;
use tokio::net::TcpListener;
use tracing::{error, info, warn};
use crate::app_state::AppState;
use crate::error::{IntoNamedVarError, Res};
use crate::route::router;

pub mod app_state;
pub mod error;
pub mod route;

async fn run() -> Res<()> {
    let router = router().with_state(AppState::new()?);

    let socket_addr_string = env::var("SOCKET_ADDR").named("SOCKET_ADDR")?;
    let socket_addr = SocketAddr::from_str(&socket_addr_string)?;

    let tcp_listener = TcpListener::bind(&socket_addr).await?;

    info!("starting server at http://{}", &socket_addr);

    axum::serve(tcp_listener, router.into_make_service()).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(e) = dotenv() {
        warn!("error reading .env file: {e}");
    }

    if let Err(e) = run().await {
        error!("exiting due to error: {e}");
    }
}
