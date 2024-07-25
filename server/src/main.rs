use std::error::Error;

use log::info;
use tokio::{net::TcpListener, signal};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	dotenvy::dotenv().ok();
	env_logger::try_init()?;

	// Grab our router
	let app = teawie_api::router();

	// Start up our listener
	let listener = listener().await?;
	info!("Starting server on {}", listener.local_addr()?);

	// Serve our app
	// and exit nicely if it fails
	if let Err(why) = axum::serve(listener, app)
		.with_graceful_shutdown(shutdown_signal())
		.await
	{
		eprintln!("Server exited with error! {why:#?}");
		std::process::exit(1)
	}

	Ok(())
}

/// Create our listener
async fn listener() -> Result<TcpListener, Box<dyn Error>> {
	let Ok(listen_address) = std::env::var("LISTEN_ADDR") else {
		return Err("Couldn't find LISTEN_ADDR in environment! Bailing".into());
	};
	let listener = TcpListener::bind(&listen_address).await?;

	Ok(listener)
}

/// Handle ctrl+c and SIGTERM signals like a good process
async fn shutdown_signal() {
	let ctrl_c = async {
		signal::ctrl_c()
			.await
			.expect("Couldn't install Ctrl+C handler");
	};

	#[cfg(unix)]
	let terminate = async {
		signal::unix::signal(signal::unix::SignalKind::terminate())
			.expect("Couldn't install signal handler")
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
