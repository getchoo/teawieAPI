use std::error::Error;

use tokio::{net::TcpListener, signal};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenvy::dotenv().ok();
	init_tracing();

	let app = teawie_api::router();

	let listener = listener().await?;
	info!("Starting server on {}", listener.local_addr()?);

	if let Err(why) = axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal())
		.await
	{
		eprintln!("Server exited with error! {why:#?}");
		std::process::exit(1)
	}

	Ok(())
}

fn init_tracing() {
	let fmt_layer = tracing_subscriber::fmt::layer().pretty();
	let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
		.unwrap_or_else(|_| "teawie_api=info,server=info,tower_http=debug,axum=trace,warn".into());

	tracing_subscriber::registry()
		.with(fmt_layer)
		.with(env_filter)
		.init();
}

#[tracing::instrument]
async fn listener() -> Result<TcpListener, Box<dyn Error>> {
	let Ok(listen_address) = std::env::var("LISTEN_ADDR") else {
		return Err("Couldn't find LISTEN_ADDR in environment! Bailing".into());
	};
	let listener = TcpListener::bind(&listen_address).await?;

	Ok(listener)
}

#[tracing::instrument]
async fn shutdown_signal() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("failed to install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("failed to install signal handler")
			.recv()
			.await;
	};

	#[cfg(not(unix))]
	let terminate = std::future::pending::<()>();

	tokio::select! {
		() = ctrl_c => {},
		() = terminate => {},
	}
}
