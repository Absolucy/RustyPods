#[macro_use]
extern crate tracing;

pub mod info;

use bluez_async::{BluetoothEvent, BluetoothSession, DeviceEvent, DiscoveryFilter};
use futures_util::StreamExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let (_, session) = BluetoothSession::new().await?;
	let mut events = session.event_stream().await?;
	session
		.start_discovery_with_filter(&DiscoveryFilter {
			duplicate_data: Some(true),
			..DiscoveryFilter::default()
		})
		.await?;

	println!("Events:");
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
		if let Ok(info) = info::AirpodsInfo::parse_from_beacon(data) {
			println!("{:?}\n{}\n", info, hex::encode(data));
		}
	}

	Ok(())
}
