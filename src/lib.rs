#![no_std]

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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    StationMode,
    SoftAPMode,
    StationAndAPMode,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Persist {
    DontSave,
    StoreInFlash,
}

fn persist_str(persist: Persist) -> &'static str {
    match persist {
        Persist::DontSave => "_CUR=",
        Persist::StoreInFlash => "_DEF=",
    }
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
    /// Gets ESP01 version information
    pub fn get_version(&mut self) -> Result<&[u8], E> {
        self.send_command(&["GMR"])?;
        self.read_until_ok()
    }

    /// Sends a command
    fn send_command(&mut self, command: &[&str]) -> Result<(), E> {
        for b in AT.iter() {
            block!(self.serial.write(*b))?;
        }
        for part in command {
            for b in part.as_bytes() {
                block!(self.serial.write(*b))?;
            }
        }
        block!(self.serial.write(CR))?;
        block!(self.serial.write(LF))?;

        Ok(())
    }

    /// Reads the output until OK\r\n or end of buffer
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

    /// Sets the Wi-Fi mode
    pub fn set_mode(&mut self, mode: Mode, persist: Persist) -> Result<&[u8], E> {
        let mode_str = match mode {
            Mode::StationMode => "1",
            Mode::SoftAPMode => "2",
            Mode::StationAndAPMode => "3",
        };

        self.send_command(&["CWMODE", persist_str(persist), mode_str])?;
        self.read_until_ok()
    }

    // TODO: Make it so that it requires a station mode through the type system
    /// Connects to an access point
    pub fn connect_ap(&mut self, ssid: &str, password: &str, persist: Persist) -> Result<&[u8], E> {
        self.send_command(&[
            "CWJAP",
            persist_str(persist),
            "\"",
            ssid,
            "\",\"",
            password,
            "\"",
        ])?;
        self.read_until_ok()
    }
}
