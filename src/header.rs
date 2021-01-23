use std::{borrow::Cow, convert::TryFrom};

use num_derive::{FromPrimitive, ToPrimitive};

pub struct Header {
    sample_rate: SampleRate,
    sub_protocol: SubProtocol,
    num_samples: u8,
    num_channels: u8,
    bit_resolution: BitResolution,
    codec: Codec,
    stream_name: [u8; 16],
    frame_number: u32,
}

impl Header {
    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    pub fn stream_name(&self) -> Cow<'_, str> {
        String::from_utf8_lossy(&self.stream_name)
    }

    pub fn sub_protocol(&self) -> SubProtocol {
        self.sub_protocol
    }

    pub fn num_samples(&self) -> u8 {
        self.num_samples
    }

    pub fn num_channels(&self) -> u8 {
        self.num_channels
    }

    pub fn bit_resolution(&self) -> BitResolution {
        self.bit_resolution
    }

    pub fn codec(&self) -> Codec {
        self.codec
    }

    pub fn frame_number(&self) -> u32 {
        self.frame_number
    }
}

impl TryFrom<&[u8]> for Header {
    type Error = super::Error;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        use byteorder::{BigEndian, ByteOrder};
        use num_traits::FromPrimitive;
        if &data[0..4] != "VBAN".as_bytes() {
            return Err(super::Error::MissingMagicNumber);
        }
        let sr_sp = data[4];
        let sample_rate = SampleRate::from_u8(sr_sp & 0b00011111).unwrap();
        let sub_protocol = SubProtocol::from_u8(sr_sp & 0b11100000).unwrap();
        let samples_per_frame = data[5];
        let channels = data[6];
        let format_codec = data[7];
        if format_codec & 0b00001000 != 0b00001000 {
            return Err(super::Error::MalformedFormat);
        }
        let bit_resolution = BitResolution::from_u8(format_codec & 0b00000111).unwrap();
        let codec = Codec::from_u8(format_codec & 0b11110000).unwrap();
        let mut stream_name: [u8; 16] = [0; 16];
        stream_name.copy_from_slice(&data[8..24]);
        let frame_number = BigEndian::read_u32(&data[24..28]);
        Ok(Self {
            sample_rate,
            sub_protocol,
            num_samples: samples_per_frame + 1,
            num_channels: channels + 1,
            bit_resolution,
            codec,
            stream_name,
            frame_number,
        })
    }
}

impl Into<[u8; 28]> for Header {
    fn into(self) -> [u8; 28] {
        use byteorder::{BigEndian, ByteOrder};
        use num_traits::ToPrimitive;
        let mut header = [0; 28];
        // Magic number
        'V'.encode_utf8(&mut header[0..]);
        'B'.encode_utf8(&mut header[1..]);
        'A'.encode_utf8(&mut header[2..]);
        'N'.encode_utf8(&mut header[3..]);
        header[4] = self.sample_rate.to_u8().unwrap();
        header[5] = self.num_samples - 1;
        header[6] = self.num_channels - 1;
        header[7] = 0b00001000 + self.bit_resolution.to_u8().unwrap() + self.codec.to_u8().unwrap();
        for i in 0..16 {
            header[8 + i] = self.stream_name[i];
        }
        BigEndian::write_u32(&mut header[24..28], self.frame_number);

        header
    }
}

#[derive(Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum SampleRate {
    Hz6000 = 0,
    Hz12000,
    Hz24000,
    Hz48000,
    Hz96000,
    Hz192000,
    Hz384000,
    Hz8000,
    Hz16000,
    Hz32000,
    Hz64000,
    Hz128000,
    Hz256000,
    Hz512000,
    Hz11025,
    Hz22050,
    Hz44100,
    Hz88200,
    Hz176400,
    Hz352800,
    Hz705600,
}

#[derive(Clone, Copy, FromPrimitive)]
pub enum SubProtocol {
    Audio = 0x00,
    Serial = 0x20,
    Text = 0x40,
    Service = 0x60,
    Undefined1 = 0x80,
    Undefined2 = 0xa0,
    Undefined3 = 0xc0,
    User = 0xe0,
}

#[derive(Clone, Copy, ToPrimitive, FromPrimitive)]
pub enum BitResolution {
    Unsigned8Bit = 0,
    Signed16Bit,
    Signed24Bit,
    Signed32Bit,
    Float32Bit,
    Float64Bit,
    Signed12Bit,
    Signed10Bit,
}

#[derive(Clone, Copy, ToPrimitive, FromPrimitive)]
pub enum Codec {
    PCM = 0x00,
    VBCA = 0x10,
    VBCV = 0x20,
    Undefined1 = 0x30,
    Undefined2 = 0x40,
    Undefined3 = 0x50,
    Undefined4 = 0x60,
    Undefined5 = 0x70,
    Undefined6 = 0x80,
    Undefined7 = 0x90,
    Undefined8 = 0xa0,
    Undefined9 = 0xb0,
    Undefined10 = 0xc0,
    Undefined11 = 0xd0,
    Undefined12 = 0xe0,
    User = 0xf0,
}
