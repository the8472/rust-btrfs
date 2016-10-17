//! This module contains the public types used by this library.
//!
//! Many of these closely match the C types used internally when interfacing
//! with the kernal, but they are desiggned to be idomatically Rust and far
//! simpler to use.

extern crate uuid;

use std::ffi;

use ctypes::*;

#[ derive (Debug, Eq, PartialEq) ]
pub enum GroupType {
	Data,
	System,
	MetaData,
	DataAndMetaData,
	GlobalReserve,
	Unknown,
}

#[ derive (Debug, Eq, PartialEq) ]
pub enum GroupProfile {
	Single,
	Raid0,
	Raid1,
	Raid5,
	Raid6,
	Dup,
	Raid10,
	Unknown,
}

#[ derive (Debug, Eq, PartialEq) ]
pub struct SpaceInfo {
	pub group_type: GroupType,
	pub group_profile: GroupProfile,
	pub total_bytes: u64,
	pub used_bytes: u64,
}

#[ derive (Debug, Eq, PartialEq) ]
pub struct FilesystemInfo {
	pub max_id: u64,
	pub num_devices: u64,
	pub filesystem_id: uuid::Uuid,
}

#[ derive (Debug, Eq, PartialEq) ]
pub struct DeviceInfo {
	pub device_id: u64,
	pub uuid: uuid::Uuid,
	pub bytes_used: u64,
	pub total_bytes: u64,
	pub path: ffi::OsString,
}

#[ derive (Debug, Eq, PartialEq) ]
pub struct DedupeRange {
	pub src_offset: u64,
	pub src_length: u64,
	pub dest_infos: Vec <DedupeRangeDestInfo>,
}

#[ derive (Debug, Eq, PartialEq) ]
pub enum DedupeRangeStatus {
	Same,
	Differs,
}

#[ derive (Debug, Eq, PartialEq) ]
pub struct DedupeRangeDestInfo {
	pub dest_fd: i64,
	pub dest_offset: u64,
	pub bytes_deduped: u64,
	pub status: DedupeRangeStatus,
}

impl From <u64> for GroupType {

	fn from (
		flags: u64,
	) -> GroupType {

		match flags & BLOCK_GROUP_TYPE_AND_RESERVED_MASK {

			BLOCK_GROUP_DATA =>
				GroupType::Data,

			BLOCK_GROUP_SYSTEM =>
				GroupType::System,

			BLOCK_GROUP_METADATA =>
				GroupType::MetaData,

			BLOCK_GROUP_DATA_AND_METADATA =>
				GroupType::DataAndMetaData,

			BLOCK_GROUP_RESERVED =>
				GroupType::GlobalReserve,

			_ =>
				GroupType::Unknown,

		}

	}

}

impl GroupProfile {

	pub fn from_string (
		string_value: & str,
	) -> Option <GroupProfile> {

		match string_value {

			"single" => Some (GroupProfile::Single),
			"raid0" => Some (GroupProfile::Raid0),
			"raid1" => Some (GroupProfile::Raid1),
			"raid5" => Some (GroupProfile::Raid5),
			"raid6" => Some (GroupProfile::Raid6),
			"dup" => Some (GroupProfile::Dup),
			"raid10" => Some (GroupProfile::Raid10),
			"unknown" => Some (GroupProfile::Unknown),

			_ => None,

		}

	}

}

impl From <u64> for GroupProfile {

	fn from (
		flags: u64,
	) -> GroupProfile {

		match flags & BLOCK_GROUP_PROFILE_MASK {

			0 =>
				GroupProfile::Single,

			BLOCK_GROUP_RAID0 =>
				GroupProfile::Raid0,

			BLOCK_GROUP_RAID1 =>
				GroupProfile::Raid1,

			BLOCK_GROUP_RAID5 =>
				GroupProfile::Raid5,

			BLOCK_GROUP_RAID6 =>
				GroupProfile::Raid6,

			BLOCK_GROUP_DUP =>
				GroupProfile::Dup,

			BLOCK_GROUP_RAID10 =>
				GroupProfile::Raid10,

			_ =>
				GroupProfile::Unknown,

		}

	}

}

// ex: noet ts=4 filetype=rust
