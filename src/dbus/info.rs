use crate::info::{AirpodsBattery, AirpodsCharging, AirpodsInEar, AirpodsInfo, AirpodsModel};
use serde::{Deserialize, Serialize};
use zvariant::{DeserializeDict, Optional, SerializeDict, Type, TypeDict};

#[derive(Debug, DeserializeDict, SerializeDict, TypeDict)]
pub struct DbusAirpodsInfo {
	pub battery: DbusAirpodsBattery,
	pub charging: DbusAirpodsCharging,
	pub ears: Optional<AirpodsInEar>,
	pub model: DbusAirpodsModel,
}

impl From<AirpodsInfo> for DbusAirpodsInfo {
	fn from(info: AirpodsInfo) -> Self {
		Self {
			battery: info.battery.into(),
			charging: info.charging.into(),
			ears: info.ears.into(),
			model: info.model.into(),
		}
	}
}

impl Default for DbusAirpodsInfo {
	fn default() -> Self {
		Self {
			battery: DbusAirpodsBattery::default(),
			charging: DbusAirpodsCharging::default(),
			ears: None.into(),
			model: DbusAirpodsModel::default(),
		}
	}
}

#[derive(Debug, Clone, DeserializeDict, SerializeDict, TypeDict)]
pub struct DbusAirpodsCharging {
	single: Optional<bool>,
	left: Optional<bool>,
	right: Optional<bool>,
	case: Optional<bool>,
}

impl From<AirpodsCharging> for DbusAirpodsCharging {
	fn from(c: AirpodsCharging) -> Self {
		match c {
			AirpodsCharging::Single(single) => Self {
				single: Some(single).into(),
				left: None.into(),
				right: None.into(),
				case: None.into(),
			},
			AirpodsCharging::Pods { left, right, case } => Self {
				single: None.into(),
				left: Some(left).into(),
				right: Some(right).into(),
				case: Some(case).into(),
			},
		}
	}
}

impl Default for DbusAirpodsCharging {
	fn default() -> Self {
		Self {
			single: None.into(),
			left: None.into(),
			right: None.into(),
			case: None.into(),
		}
	}
}

#[derive(Debug, Clone, DeserializeDict, SerializeDict, TypeDict)]
pub struct DbusAirpodsBattery {
	single: Optional<u8>,
	left: Optional<u8>,
	right: Optional<u8>,
	case: Optional<u8>,
}

impl Default for DbusAirpodsBattery {
	fn default() -> Self {
		Self {
			single: None.into(),
			left: None.into(),
			right: None.into(),
			case: None.into(),
		}
	}
}

impl From<AirpodsBattery> for DbusAirpodsBattery {
	fn from(c: AirpodsBattery) -> Self {
		match c {
			AirpodsBattery::Single { charge } => Self {
				single: charge.into(),
				left: None.into(),
				right: None.into(),
				case: None.into(),
			},
			AirpodsBattery::Pods { left, right, case } => Self {
				single: None.into(),
				left: left.into(),
				right: right.into(),
				case: case.into(),
			},
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
pub enum DbusAirpodsModel {
	Airpods1G,
	Airpods2G,
	AirpodsPro,
	AirpodsMax,
	PowerbeatsPro,
	BeatsX,
	BeatsFlex,
	BeatsSolo3,
	BeatsStudio3,
	Powerbeats3,
	Unknown,
}

impl Default for DbusAirpodsModel {
	fn default() -> Self {
		Self::Unknown
	}
}

impl From<AirpodsModel> for DbusAirpodsModel {
	fn from(model: AirpodsModel) -> Self {
		match model {
			AirpodsModel::Airpods1G => Self::Airpods1G,
			AirpodsModel::Airpods2G => Self::Airpods2G,
			AirpodsModel::AirpodsPro => Self::AirpodsPro,
			AirpodsModel::AirpodsMax => Self::AirpodsMax,
			AirpodsModel::PowerbeatsPro => Self::PowerbeatsPro,
			AirpodsModel::BeatsX => Self::BeatsX,
			AirpodsModel::BeatsFlex => Self::BeatsFlex,
			AirpodsModel::BeatsSolo3 => Self::BeatsSolo3,
			AirpodsModel::BeatsStudio3 => Self::BeatsStudio3,
			AirpodsModel::Powerbeats3 => Self::Powerbeats3,
			AirpodsModel::Unknown(_) => Self::Unknown,
		}
	}
}
