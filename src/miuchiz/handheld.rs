use super::st2205u;
use crate::memory::AddressSpace;
use std::error::Error;

pub type Flash = [u8; 0x200000];

enum AddressType {
    Video,
    Otp,
    Flash,
}

impl AddressType {
    pub fn parse_machine_addr(address: usize) -> (Self, usize) {
        let selection_bits = address >> 21;
        let address_bits = address & ((1 << 21) - 1);

        let addr_type = match selection_bits {
            0b00011 => AddressType::Video,
            0b00000 | 0b11111 => AddressType::Otp,
            _ => AddressType::Flash,
        };

        (addr_type, address_bits)
    }
}

pub struct HandheldAddressSpace {
    otp: Box<st2205u::Otp>,
    flash: Box<Flash>,
}

impl HandheldAddressSpace {
    pub fn new(otp: &[u8], flash: &[u8]) -> Result<Self, ConfigurationError> {
        let otp_box = Box::new(
            st2205u::Otp::try_from(otp)
                .map_err(|err| ConfigurationError::InvalidOtp(err.into()))?,
        );

        let flash_box = Box::new(
            Flash::try_from(flash).map_err(|err| ConfigurationError::InvalidFlash(err.into()))?,
        );

        Ok(Self {
            otp: otp_box,
            flash: flash_box,
        })
    }
}

impl AddressSpace for HandheldAddressSpace {
    fn read_u8(&mut self, address: usize) -> u8 {
        match AddressType::parse_machine_addr(address) {
            (AddressType::Video, _vid_addr) => {
                todo!("Read video registers")
            }
            (AddressType::Otp, otp_addr) => self.otp[otp_addr % self.otp.len()],
            (AddressType::Flash, flash_addr) => self.flash[flash_addr % self.flash.len()],
        }
    }

    fn write_u8(&mut self, address: usize, value: u8) {
        match AddressType::parse_machine_addr(address) {
            (AddressType::Video, vid_addr) => {
                // todo!("Write video registers")
                println!("Video write {value:02X} to {vid_addr:X}")
            }
            (AddressType::Otp, _otp_addr) => {
                todo!("Write OTP???")
            }
            (AddressType::Flash, _flash_addr) => {
                todo!("Write Flash")
            }
        }
    }
}

#[derive(Debug)]
pub enum ConfigurationError {
    InvalidOtp(Box<dyn Error>),
    InvalidFlash(Box<dyn Error>),
}

pub struct Handheld {
    pub mcu: st2205u::Mcu<HandheldAddressSpace>,
}

impl Handheld {
    pub fn new(otp: &[u8], flash: &[u8]) -> Result<Self, ConfigurationError> {
        let machine_address_space = HandheldAddressSpace::new(otp, flash)?;

        let mcu = Self {
            mcu: st2205u::Mcu::new(machine_address_space),
        };

        Ok(mcu)
    }
}
