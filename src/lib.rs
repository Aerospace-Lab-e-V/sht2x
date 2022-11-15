//! # SHT2x library
#![no_std]

#[macro_use]
extern crate num_derive;

use byteorder::{ByteOrder, LittleEndian};

use embedded_hal::blocking::i2c::{Read, Write, WriteRead};

const I2C_ADDR: u8 = 0x40;

#[allow(dead_code)]
enum Command {
    TempMeasurementHoldMaster = 0xE3,
    RelHumMeasurementHoldMaster = 0xE5,
    TempMeasurement = 0xF3,
    RelHumMeasurement = 0xF5,
    WriteUserRegister = 0xE6,
    ReadUserRegister = 0xE7,
    SoftReset = 0xFE,
}

#[derive(FromPrimitive)]
pub enum Resolution {
    Bits8 = 0x01,
    Bits10 = 0x80,
    Bits11 = 0x81,
    Bits12 = 0x00,
}

struct UserRegister {
    reserved_bits: u8,
    onchip_heater: bool,
    otp_reload: bool,
    end_of_battery: bool,
    resolution: Resolution,
}

impl UserRegister {
    pub fn from_bytes(b: u8) -> Self {
        Self {
            reserved_bits: 0b0011_1000 & b,
            otp_reload: b & (1 << 1) != (1 << 1),
            onchip_heater: b & (1 << 2) == (1 << 2),
            end_of_battery: b & (1 << 6) == (1 << 6),
            resolution: num::FromPrimitive::from_u8(0x81 & b).unwrap(),
        }
    }
    pub fn to_bytes(self) -> u8 {
        self.resolution as u8
            & self.reserved_bits
            & (!self.otp_reload as u8) << 1
            & (self.onchip_heater as u8) << 2
            & (self.end_of_battery as u8) << 6
    }
}
pub struct SHT2x<I2C, D> {
    i2c: I2C,
    delay: D,
}

impl<I2C, E, D> SHT2x<I2C, D>
where
    I2C: Write<Error = E> + WriteRead<Error = E> + Read<Error = E>,
    D: embedded_hal::blocking::delay::DelayMs<u16>,
{
    pub fn new(i2c: I2C, delay: D) -> Self {
        Self { i2c, delay }
    }

    pub fn reset(&mut self) {
        let _ = self.i2c.write(I2C_ADDR, &[Command::SoftReset as u8]);
        self.delay.delay_ms(15);
    }

    pub fn temperature(&mut self) -> f32 {
        let _ = self
            .i2c
            .write(I2C_ADDR, &[Command::TempMeasurementHoldMaster as u8]);
        let mut data = [0_u8; 3];
        let _ = self.i2c.read(I2C_ADDR, &mut data);

        if Self::crc(&data[0..1]) != data[2] {
            todo!()
        }

        -46.85 + 175.72 * LittleEndian::read_u16(&data[0..1]) as f32 / 65536.0
    }

    pub fn humidity(&mut self) -> f32 {
        let _ = self
            .i2c
            .write(I2C_ADDR, &[Command::RelHumMeasurementHoldMaster as u8]);
        let mut data = [0_u8; 3];
        let _ = self.i2c.read(I2C_ADDR, &mut data);

        if Self::crc(&data[0..1]) != data[2] {
            todo!()
        }

        -6.0 + 125.0 * LittleEndian::read_u16(&data[0..1]) as f32 / 65536.0
    }

    pub fn enable_heater(&mut self) {
        let mut ur = self.read_user_register();
        ur.onchip_heater = true;
        self.write_user_register(ur);
    }

    pub fn disable_heater(&mut self) {
        let mut ur = self.read_user_register();
        ur.onchip_heater = false;
        self.write_user_register(ur);
    }

    pub fn end_of_battery(&mut self) -> bool {
        let ur = self.read_user_register();
        ur.end_of_battery
    }

    fn write_user_register(&mut self, ur: UserRegister) {
        let _ = self
            .i2c
            .write(I2C_ADDR, &[Command::WriteUserRegister as u8, ur.to_bytes()]);
    }

    fn read_user_register(&mut self) -> UserRegister {
        let mut buf = [0u8; 1];
        let _ = self
            .i2c
            .write_read(I2C_ADDR, &[Command::ReadUserRegister as u8], &mut buf);

        UserRegister::from_bytes(buf[0])
    }

    pub fn crc(data: &[u8]) -> u8 {
        const POLY: u8 = 0x31;
        let mut crc = 0;

        for b in data {
            crc ^= b;

            for _ in 0..8 {
                if crc & 0x80 != 0 {
                    crc = (crc << 1) ^ POLY;
                } else {
                    crc <<= 1;
                }
            }
        }

        crc
    }
}
