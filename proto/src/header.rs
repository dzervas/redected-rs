#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FrameHeader {
	pub frame_type: FrameType,
	pub data: FrameHeaderData,
	pub poll_final: bool,
	pub rx_seq: u8,
}

impl FrameHeader {
	pub fn from_byte(byte: u8) -> FrameHeader {
		let frame_type = match byte & 0b1000_0000 {
			0 => FrameType::Information,
			1 => FrameType::Control,
			_ => panic!("Invalid frame type"),
		};

		let poll_final = (byte & 0b0010_0000) != 0;
		let rx_seq = byte & 0b0001_1111;

		let data = match frame_type {
			FrameType::Information => FrameHeaderData::Information{ tx_seq: byte & 0b0111_0000},
			FrameType::Control => {
				if byte & 0b0100_0000 != 0 {
					let un_id = match byte & 0b0011_0000 {
						0 => UnnumberedType::AsyncBalanced,
						_ => panic!("Invalid unnumbered type"),
					};

					FrameHeaderData::Unnumbered { un_id }
				} else {
					let su_id = match byte & 0b0011_0000 {
						0 => SupervisoryType::ReceiveReady,
						1 => SupervisoryType::ReceiveNotReady,
						2 => SupervisoryType::Reject,
						_ => panic!("Invalid supervisory type"),
					};

					FrameHeaderData::Supervisory{ su_id }
				}
			},
		};

		FrameHeader {
			frame_type,
			data,
			poll_final,
			rx_seq,
		}
	}

	pub fn to_byte(&self) -> u8 {
		let mut byte = match self.frame_type {
			FrameType::Information => 0,
			FrameType::Control => 0b1000_0000,
		};

		byte |= self.rx_seq;

		match self.frame_type {
			FrameType::Information => {
				if let FrameHeaderData::Information{ tx_seq } = self.data.clone() {
					byte |= tx_seq << 4;
				}
			},
			FrameType::Control => {
				if let FrameHeaderData::Supervisory{ su_id } = self.data.clone() {
					byte |= match su_id {
						SupervisoryType::ReceiveReady => 0,
						SupervisoryType::ReceiveNotReady => 1,
						SupervisoryType::Reject => 2,
					} << 4;
				} else if let FrameHeaderData::Unnumbered{ un_id } = self.data.clone() {
					byte |= 0b0100_0000;
					byte |= match un_id {
						UnnumberedType::AsyncBalanced => 0,
					} << 4;
				}
			},
		}

		byte |= if self.poll_final { 0b0000_1000 } else { 0 };

		byte
	}
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FrameHeaderData {
	Information{ tx_seq: u8 },
	Supervisory{ su_id: SupervisoryType },
	Unnumbered{ un_id: UnnumberedType },
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum FrameType {
	Information = 0,
	Control = 1,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ControlType {
	Supervisory = 0,
	Unnumbered = 1,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum SupervisoryType {
	ReceiveReady = 0,
	ReceiveNotReady = 1,
	Reject = 2,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum UnnumberedType {
	AsyncBalanced = 0,
}
