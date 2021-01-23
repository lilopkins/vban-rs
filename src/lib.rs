mod header;

pub const HEADER_SIZE: usize = 28;
pub const MAX_PACKET_SIZE: usize = 1464;
use std::convert::TryFrom;

pub use header::{Codec, Header, SubProtocol};

pub enum Error {
    MissingMagicNumber,
    MalformedFormat,
}

pub struct Packet {
    header: Header,
    pub data: Vec<u8>,
}

impl Packet {
    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut Header {
        &mut self.header
    }
}

impl TryFrom<&[u8]> for Packet {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let pkt = Packet {
            header: Header::try_from(value)?,
            data: Vec::from(&value[HEADER_SIZE..]),
        };
        Ok(pkt)
    }
}
