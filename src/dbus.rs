mod info;

use self::info::{DbusAirpodsBattery, DbusAirpodsInfo};
use crate::dbus::info::{DbusAirpodsCharging, DbusAirpodsModel};
use crate::info::{AirpodsInEar, AirpodsInfo};
use std::sync::Arc;
use tokio::sync::RwLock;
use zbus::dbus_interface;
use zvariant::Optional;

pub struct RustyPodsService {
	pub(crate) current_info: Arc<RwLock<Option<AirpodsInfo>>>,
}

#[dbus_interface(name = "moe.absolucy.RustyPods1")]
impl RustyPodsService {
	async fn info(&self) -> Optional<DbusAirpodsInfo> {
		self.current_info
			.read()
			.await
			.map(DbusAirpodsInfo::from)
			.into()
	}

	async fn battery(&self) -> Optional<DbusAirpodsBattery> {
		self.current_info
			.read()
			.await
			.map(DbusAirpodsInfo::from)
			.map(|info| info.battery)
			.into()
	}

	async fn charging(&self) -> Optional<DbusAirpodsCharging> {
		self.current_info
			.read()
			.await
			.map(DbusAirpodsInfo::from)
			.map(|info| info.charging)
			.into()
	}

	async fn in_ears(&self) -> Optional<AirpodsInEar> {
		self.current_info
			.read()
			.await
			.map(DbusAirpodsInfo::from)
			.and_then(|mut info| info.ears.take())
			.into()
	}

	async fn model(&self) -> Optional<DbusAirpodsModel> {
		self.current_info
			.read()
			.await
			.map(DbusAirpodsInfo::from)
			.map(|info| info.model)
			.into()
	}
}
