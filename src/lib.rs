//#![no_std]

use embedded_hal::serial::{Read, Write};
use nb::block;

const CR: u8 = 0x0d;
const LF: u8 = 0x0a;
const OK: [u8; 3] = [b'O', b'K', CR];
const AT: [u8; 3] = [b'A', b'T', b'+'];

pub struct Esp01<S> {
    serial: S,
    read_buf: [u8; 512],
}

pub fn esp01<S, E>(serial: S) -> Esp01<S>
where
    S: Read<u8, Error = E> + Write<u8, Error = E>,
{
    Esp01 {
        serial,
        read_buf: [0; 512],
    }
}

impl<S, E> Esp01<S>
where
    S: Read<u8, Error = E> + Write<u8, Error = E>,
{
    pub fn get_version(&mut self) -> Result<&[u8], E> {
        self.send_command("GMR")?;
        self.read_until_ok()
    }

    fn send_command(&mut self, command: &str) -> Result<(), E> {
        for b in AT.iter() {
            block!(self.serial.write(*b))?;
        }
        for b in command.as_bytes() {
            block!(self.serial.write(*b))?;
        }
        block!(self.serial.write(CR))?;
        block!(self.serial.write(LF))?;

        Ok(())
    }

    pub fn read_until_ok(&mut self) -> Result<&[u8], E> {
        let mut i = 0;

        while i < self.read_buf.len() {
            match block!(self.serial.read())? {
                LF if i > 2 && self.read_buf[(i - 3)..i] == OK => {
                    return Ok(&self.read_buf[0..i]);
                }
                other => {
                    self.read_buf[i] = other;
                    i += 1;
                }
            }
        }

        Ok(&self.read_buf[0..i])
    }
}
