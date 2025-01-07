use crate::header::{FrameHeader, FrameHeaderData, FrameType, SupervisoryType, UnnumberedType};
use crate::mail::Mail;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BusMail {
	pub header: FrameHeader,
	pub length: u16,
	pub data: Option<Mail>,
	pub checksum: u8,
}

impl BusMail {
	// pub fn new<'a>(header: FrameHeader, data: &'a[u8]) -> BusMail<'a> {
	// 	let length = data.len() as u16;
	// 	let mut checksum = header.to_byte();
	// 	for byte in data {
	// 		checksum = checksum.wrapping_add(*byte);
	// 	}

	// 	BusMail {
	// 		header,
	// 		length,
	// 		data,
	// 		checksum,
	// 	}
	// }

	pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<BusMail, String> {
		if bytes.len() < 4 {
			return Err("Not enough bytes".to_string());
		}

		if bytes[0] != 0x10 {
			return Err("Frame character".to_string());
		}

		let length = u16::from_be_bytes([bytes[1], bytes[2]]);
		let header = FrameHeader::from_byte(bytes[3]);
		let data = if length > 1 {
			Some(Mail::from_bytes(&bytes[4..4+(length - 1) as usize]))
		} else {
			None
		};
		let checksum = *bytes.last().unwrap();

		let busmail = BusMail {
			header,
			length,
			data,
			checksum,
		};

		if busmail.calc_checksum() != checksum {
			// return Err("Checksum error".to_string());
			println!("Checksum error: {:02X} != {:02X}", busmail.calc_checksum(), checksum);
		}

		Ok(busmail)
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.push(0x10);
		bytes.extend(self.length.to_be_bytes());
		bytes.push(self.header.to_byte());
		if let Some(data) = &self.data {
			bytes.extend(data.to_bytes());
		}
		bytes.push(self.checksum);

		bytes
	}

	pub fn calc_checksum(&self) -> u8 {
		let mut checksum = self.header.to_byte();
		if let Some(data) = &self.data {
			for byte in data.to_bytes() {
				checksum += byte;
			}
		}

		checksum
	}

	pub fn calculate(&mut self) {
		self.length = self.data.as_ref().map(|d| d.to_bytes().len() as u16).unwrap_or(0) + 1;
		self.checksum = self.calc_checksum();
	}

	pub fn handle_response(&self) -> Option<BusMail> {
		let mut response = self.clone();

		match &self.header.data {
			FrameHeaderData::Unnumbered { .. } => {
				response.header.frame_type = FrameType::Control;
				response.header.data = FrameHeaderData::Unnumbered { un_id: UnnumberedType::AsyncBalanced };
				response.header.rx_seq = 0;
				response.header.poll_final = false;
				response.data = None;
			},
			FrameHeaderData::Supervisory { .. } => {
				return None;
			},
			FrameHeaderData::Information { .. } => {
				response.header.frame_type = FrameType::Control;
				// TODO: Handle incorrect checksum with a reject frame
				// TODO: Resend rejected frames here
				response.header.data = FrameHeaderData::Supervisory { su_id: SupervisoryType::ReceiveReady };
				response.header.rx_seq = self.header.rx_seq.wrapping_add(1);
				response.data = None;
			},
		};

		response.calculate();
		Some(response)
	}
}
