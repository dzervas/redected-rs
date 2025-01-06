// TODO: Get rid of Vec
// #![no_std]

pub mod frame;
pub mod header;
pub mod mail;
pub mod primitives;

pub use frame::BusMail;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_frame() {
        let data = [0x10, 0x00, 0x01, 0xC8, 0xC8];
        let mail = BusMail::from_bytes(&data).unwrap();

        assert_eq!(mail.header.rx_seq, 8);
        assert_eq!(mail.header.poll_final, false);
        assert_eq!(mail.header.data, header::FrameHeaderData::Unnumbered { un_id: header::UnnumberedType::AsyncBalanced });
        assert_eq!(mail.length, 0x01);
        assert_eq!(mail.data, None);
        assert_eq!(mail.checksum, 0xC8);
    }
}
