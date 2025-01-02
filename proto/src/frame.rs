use crate::header::FrameHeader;
use crate::mail::Mail;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct BusMail {
	pub header: FrameHeader,
	pub length: u16,
	pub data: Mail,
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
}
