use proto::frame::BusMail;

fn main() {
	let ports = serialport::available_ports().expect("No serial ports found!");

	if ports.len() == 0 {
		println!("No serial ports found!");
		return;
	}

	if ports.len() > 1 {
		println!("Multiple serial ports found, using the first one: {}", ports[0].port_name);
	}

	let mut s = serialport::new(ports[0].port_name.as_str(), 115200)
		.timeout(std::time::Duration::from_secs(1))
		.open()
		.expect("Failed to open serial port");

	let mut init_mail = BusMail {
		header: proto::header::FrameHeader {
			frame_type: proto::header::FrameType::Control,
			rx_seq: 0,
			poll_final: false,
			data: proto::header::FrameHeaderData::Unnumbered {
				un_id: proto::header::UnnumberedType::AsyncBalanced,
			},
		},
		length: 0,
		data: None,
		checksum: 0,
	};
	init_mail.calculate();

	s.write(&init_mail.to_bytes()).expect("Failed to write init_mail to serial port");

	loop {
		let mut len_buf = [0; 3];
		let n = s.read(&mut len_buf).expect("Failed to read from serial port");

		if n != 3 {
			println!("Failed to read frame length - read {} bytes", n);
			continue;
		}

		if len_buf[0] != 0x10 {
			println!("Invalid frame character: 0x{:02X}", len_buf[0]);
			continue;
		}

		let length = u16::from_be_bytes([len_buf[1], len_buf[2]]);
		let mut buf = vec![0; length as usize + 1]; // +1 for checksum

		let n = s.read(&mut buf).expect("Failed to read from serial port");

		if n == 0 {
			continue;
		}

		let mut new_buf = len_buf.to_vec();
		new_buf.extend_from_slice(&buf);
		println!("Received frame: {:02X?}", new_buf);
		let mail = BusMail::from_bytes(&new_buf).expect("Failed to parse mail");
		println!("< {:?}", mail);

		let resp = mail.handle_response();

		// if resp.is_none() && mail.header.data == proto::header::FrameHeaderData::Supervisory { proto::header::SupervisoryType::ReceiveReady } {
		// 	println!("Received final frame, exiting");
			// break;
		// }

		if let Some(resp) = resp {
			println!("> {:?}", resp);
			s.write_all(&resp.to_bytes()).expect("Failed to write response to serial port");
		}
	}
}
