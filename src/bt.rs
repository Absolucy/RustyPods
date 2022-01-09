use crate::info::AirpodsInfo;
use anyhow::{Context, Result};
use bluez_async::{BluetoothEvent, BluetoothSession, DeviceEvent, DiscoveryFilter};
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn run_bluetooth_listener(current_info: Arc<RwLock<Option<AirpodsInfo>>>) -> Result<()> {
	let (_, session) = BluetoothSession::new()
		.await
		.context("Failed to set up bluetooth session")?;
	let mut events = session
		.event_stream()
		.await
		.context("Failed to set up bluetooth event stream")?;
	session
		.start_discovery_with_filter(&DiscoveryFilter {
			duplicate_data: Some(true),
			..DiscoveryFilter::default()
		})
		.await
		.context("Failed to start bluetooth discovery")?;

	while let Some(event) = events.next().await {
		let manufacturer_data = match event {
			BluetoothEvent::Device {
				event: DeviceEvent::ManufacturerData { manufacturer_data },
				..
			} => manufacturer_data,
			_ => continue,
		};
		let data = match manufacturer_data.get(&76) {
			Some(data) => data,
			None => {
				continue;
			}
		};
		debug!("got airpods manufacturer data: {:?}", manufacturer_data);
		if data.len() != 27 {
			debug!("this ain't what we want, nevermind");
			continue;
		}
		if let Ok(info) = AirpodsInfo::parse_from_beacon(data) {
			current_info.write().await.replace(info);
		}
	}

	Ok(())
}
