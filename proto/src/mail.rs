#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Mail {
	pub program_id: u8,
	pub task_id: u8,
	pub primitive: u16,
	pub parameters: Vec<u8>,
}

impl Mail {
	pub fn from_bytes(bytes: &[u8]) -> Mail {
		Mail {
			program_id: bytes[0],
			task_id: bytes[1],
			// TODO: Is that really le?
			primitive: u16::from_le_bytes([bytes[2], bytes[3]]),
			parameters: bytes[4..].to_vec(),
		}
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		let mut bytes = Vec::new();
		bytes.push(self.program_id);
		bytes.push(self.task_id);
		bytes.extend(self.primitive.to_le_bytes());
		bytes.extend(self.parameters.iter());
		bytes
	}
}