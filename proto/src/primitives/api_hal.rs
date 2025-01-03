#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LedCmdIdType {
	Off = 0,
	On = 1,
	RepeatSequence = 2,
	Invalid = 0xFF,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LedCmdType {
	pub command: LedCmdIdType,
	pub duration: u16,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct LedReq {
	pub led_nr: u8,
	pub cmd_count: u8,
	pub commands: Vec<LedCmdType>,
}
