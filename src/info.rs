use anyhow::{anyhow, Result};
use zvariant::{DeserializeDict, SerializeDict, TypeDict};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AirpodsInfo {
	pub battery: AirpodsBattery,
	pub charging: AirpodsCharging,
	pub ears: Option<AirpodsInEar>,
	pub model: AirpodsModel,
}

impl AirpodsInfo {
	pub fn parse_from_beacon(data: &[u8]) -> Result<Self> {
		assert_eq!(data.len(), 27);
		let model = Self::model(data);
		let battery = Self::battery(data, model)?;
		let charging = Self::charging(data, model);
		let ears = Self::ears(data, model);
		Ok(Self {
			battery,
			charging,
			ears,
			model,
		})
	}

	fn flipped(data: &[u8]) -> bool {
		Self::get_point(data, 10) & 0x02 == 0
	}

	fn model(data: &[u8]) -> AirpodsModel {
		AirpodsModel::from(Self::get_point(data, 7))
	}

	fn battery(data: &[u8], model: AirpodsModel) -> Result<AirpodsBattery> {
		if model.is_single() {
			let charge = Self::get_battery(Self::get_point(data, 15))?;
			Ok(AirpodsBattery::Single { charge })
		} else {
			let mut left = Self::get_battery(Self::get_point(data, 12))?;
			let mut right = Self::get_battery(Self::get_point(data, 13))?;
			let case = Self::get_battery(Self::get_point(data, 15))?;
			if Self::flipped(data) {
				std::mem::swap(&mut left, &mut right);
			}
			Ok(AirpodsBattery::Pods { left, right, case })
		}
	}

	fn charging(data: &[u8], model: AirpodsModel) -> AirpodsCharging {
		let byte = Self::get_point(data, 14);
		if model.is_single() {
			let charging = byte & (1 << 0) != 0;
			AirpodsCharging::Single(charging)
		} else {
			let mut left = byte & (1 << 0) != 0;
			let mut right = byte & (1 << 1) != 0;
			let case = byte & (1 << 2) != 0;
			if Self::flipped(data) {
				std::mem::swap(&mut left, &mut right);
			}
			AirpodsCharging::Pods { left, right, case }
		}
	}

	fn ears(data: &[u8], model: AirpodsModel) -> Option<AirpodsInEar> {
		if model.is_single() {
			None
		} else {
			let byte = Self::get_point(data, 11);
			let mut left = byte & (1 << 1) != 0;
			let mut right = byte & (1 << 3) != 0;
			if Self::flipped(data) {
				std::mem::swap(&mut left, &mut right);
			}
			Some(AirpodsInEar { left, right })
		}
	}

	fn get_point(data: &[u8], point: usize) -> u8 {
		let offset = point / 2;
		let byte = data[offset];
		if point % 2 != 0 {
			byte & 0x0F
		} else {
			byte >> 4
		}
	}

	fn get_battery(batt: u8) -> Result<Option<u8>> {
		match batt {
			0..=10 => Ok(Some(batt * 10)),
			15 => Ok(None),
			_ => Err(anyhow!("Invalid battery level: {}", batt)),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AirpodsBattery {
	Single {
		charge: Option<u8>,
	},
	Pods {
		left: Option<u8>,
		right: Option<u8>,
		case: Option<u8>,
	},
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, DeserializeDict, SerializeDict, TypeDict)]
pub struct AirpodsInEar {
	left: bool,
	right: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AirpodsCharging {
	Single(bool),
	Pods { left: bool, right: bool, case: bool },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AirpodsModel {
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
	Unknown(u8),
}

impl AirpodsModel {
	fn is_single(&self) -> bool {
		matches!(
			self,
			AirpodsModel::AirpodsMax
				| AirpodsModel::PowerbeatsPro
				| AirpodsModel::BeatsX
				| AirpodsModel::BeatsFlex
				| AirpodsModel::BeatsSolo3
				| AirpodsModel::BeatsStudio3
				| AirpodsModel::Powerbeats3
		)
	}
}

impl From<u8> for AirpodsModel {
	fn from(c: u8) -> Self {
		match c {
			0x2 => AirpodsModel::Airpods1G,
			0xF => AirpodsModel::Airpods2G,
			0xE => AirpodsModel::AirpodsPro,
			0xA => AirpodsModel::AirpodsMax,
			0xB => AirpodsModel::PowerbeatsPro,
			0x5 => AirpodsModel::BeatsX,
			0x0 => AirpodsModel::BeatsFlex,
			0x6 => AirpodsModel::BeatsSolo3,
			0x9 => AirpodsModel::BeatsStudio3,
			0x3 => AirpodsModel::Powerbeats3,
			_ => AirpodsModel::Unknown(c),
		}
	}
}
