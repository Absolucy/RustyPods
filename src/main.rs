#[macro_use]
extern crate tracing;

mod bt;
mod dbus;
mod info;

use anyhow::{Context, Result};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
	// Set up logging
	let subscriber = FmtSubscriber::builder()
		.with_max_level(Level::INFO)
		.finish();
	tracing::subscriber::set_global_default(subscriber)
		.context("setting default subscriber failed")?;
	// Set up the DBus server
	let current_info = Arc::new(RwLock::new(None));
	let conn = zbus::ConnectionBuilder::session()
		.context("failed to get zbus connection")?
		.serve_at(
			"/moe/absolucy/RustyPods",
			dbus::RustyPodsService {
				current_info: current_info.clone(),
			},
		)
		.context("Failed to serve at /moe/absolucy/RustyPods")?
		.name("moe.absolucy.RustyPods")
		.context("Failed to set name to moe.absolucy.RustyPods")?
		.internal_executor(false)
		.build()
		.await
		.context("Failed to set up DBus server")?;
	// Run the bluetooth listener
	tokio::spawn(bt::run_bluetooth_listener(current_info));
	// Tick the D-Bus connector
	loop {
		conn.executor().tick().await;
	}
}
