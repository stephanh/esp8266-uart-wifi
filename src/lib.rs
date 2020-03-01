#![no_std]

pub mod errors;

use core::marker::PhantomData;

use embedded_hal::serial::{Read, Write};

use nb::block;

use crate::errors::EResult;
use crate::errors::Error;

const CR: u8 = b'\r';
const LF: u8 = b'\n';
const LINE_END: [u8; 2] = [CR, LF];
const RESPONSE_END: [u8; 4] = [CR, LF, CR, LF];
const OK: [u8; 3] = *b"OK\r";
const ERROR: [u8; 6] = *b"ERROR\r";
const FAIL: [u8; 5] = *b"FAIL\r";
const AT: [u8; 3] = *b"AT+";

pub struct Esp01<S, MODE> {
    serial: S,
    read_buf: [u8; 512],
    _mode: PhantomData<MODE>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    StationMode,
    SoftAPMode,
    StationAndAPMode,
}

pub struct UnknownMode {}
pub struct StationMode<MODE> {
    _mode: PhantomData<MODE>,
}

pub struct APConnected {}
pub struct APDisconnected {}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Persist {
    DontSave,
    SaveInFlash,
}

impl Persist {
    pub fn as_str(&self) -> &'static str {
        match self {
            Persist::DontSave => "_CUR=",
            Persist::SaveInFlash => "_DEF=",
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum QueryMode {
    Current,
    SavedInFlash,
}

impl QueryMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            QueryMode::Current => "_CUR",
            QueryMode::SavedInFlash => "_DEF",
        }
    }
}

pub fn esp01<S, E>(serial: S) -> Esp01<S, UnknownMode>
where
    S: Read<u8, Error = E> + Write<u8, Error = E>,
{
    Esp01 {
        serial,
        read_buf: [0; 512],
        _mode: PhantomData,
    }
}

impl<S, E, MODE> Esp01<S, MODE>
where
    S: Read<u8, Error = E> + Write<u8, Error = E>,
{
    /// Writes a byte to the serial port
    fn write_byte(&mut self, byte: u8) -> EResult<()> {
        block!(self.serial.write(byte)).map_err(|_| Error::SerialWrite)
    }

    /// Reads a byte from the serial port
    fn read_byte(&mut self) -> EResult<u8> {
        block!(self.serial.read()).map_err(|_| Error::SerialRead)
    }

    /// Writes line end sequence
    fn write_line_end(&mut self) -> EResult<()> {
        self.write_byte(CR)?;
        self.write_byte(LF)
    }

    /// Reads the response for a command
    pub fn read_response(&mut self) -> EResult<&[u8]> {
        let mut i = 0;

        while i < self.read_buf.len() {
            match self.read_byte()? {
                LF if i > 2 && self.read_buf[(i - 3)..i] == OK => {
                    if i > 6 && self.read_buf[(i - 7)..(i - 3)] == RESPONSE_END {
                        return Ok(&self.read_buf[0..(i - 7)]);
                    } else {
                        return Ok(&self.read_buf[0..(i - 3)]);
                    }
                }
                LF if i > 5 && self.read_buf[(i - 6)..i] == ERROR => {
                    return Err(Error::CommandError);
                }
                LF if i > 4 && self.read_buf[(i - 5)..i] == FAIL => {
                    return Err(Error::CommandFailed);
                }
                other => {
                    self.read_buf[i] = other;
                    i += 1;
                }
            }
        }

        Ok(&self.read_buf[0..i])
    }

    /// Reads a byte and checks that it is the expected byte
    fn read_byte_back(&mut self, byte: u8) -> EResult<()> {
        if self.read_byte()? == byte {
            Ok(())
        } else {
            Err(Error::CommandReadFail)
        }
    }

    /// Checks that the response starts with the command that was sent
    fn read_command_back(&mut self, command: &[&str], query: bool) -> EResult<()> {
        for b in AT.iter() {
            self.read_byte_back(*b)?;
        }
        for part in command {
            for b in part.as_bytes() {
                self.read_byte_back(*b)?;
            }
        }
        if query {
            self.read_byte_back(b'?')?;
        }

        self.read_byte_back(CR)?;
        self.read_byte_back(CR)?;
        self.read_byte_back(LF)
    }

    /// Sends a command
    fn send_command(&mut self, command: &[&str]) -> EResult<()> {
        for b in AT.iter() {
            self.write_byte(*b)?;
        }
        for part in command {
            for b in part.as_bytes() {
                self.write_byte(*b)?;
            }
        }
        self.write_line_end()?;

        self.read_command_back(command, false)
    }

    /// Sends a query
    fn send_query(&mut self, command: &[&str]) -> EResult<&[u8]> {
        for b in AT.iter() {
            self.write_byte(*b)?;
        }
        for part in command {
            for b in part.as_bytes() {
                self.write_byte(*b)?;
            }
        }
        self.write_byte(b'?')?;
        self.write_line_end()?;
        self.read_command_back(command, true)?;
        self.read_byte_back(b'+')?;
        for part in command {
            for b in part.as_bytes() {
                self.read_byte_back(*b)?;
            }
        }
        self.read_byte_back(b':')?;
        self.read_response()
    }

    /// Gets ESP01 version information
    pub fn get_version(&mut self) -> EResult<&[u8]> {
        self.send_command(&["GMR"])?;
        self.read_response()
    }

    /// Sets the Wi-Fi mode
    pub fn set_mode(
        mut self,
        mode: Mode,
        persist: Persist,
    ) -> EResult<Esp01<S, StationMode<APDisconnected>>> {
        let mode_str = match mode {
            Mode::StationMode => "1",
            Mode::SoftAPMode => "2",
            Mode::StationAndAPMode => "3",
        };

        self.send_command(&["CWMODE", persist.as_str(), mode_str])?;
        self.read_response()?;

        Ok(Esp01 {
            serial: self.serial,
            read_buf: self.read_buf,
            _mode: PhantomData,
        })
    }

    /// Sets the MAC address for the station
    pub fn set_station_mac(&mut self, mac: &str, persist: Persist) -> EResult<&[u8]> {
        self.send_command(&["CIPSTAMAC", persist.as_str(), "\"", mac, "\""])?;
        self.read_response()
    }

    /// Gets the MAC address for the station
    pub fn get_station_mac(&mut self, query_mode: QueryMode) -> EResult<&[u8]> {
        let r = self.send_query(&["CIPSTAMAC", query_mode.as_str()])?;
        Ok(&r[1..(r.len() - 1)])
    }
}

impl<S, E> Esp01<S, StationMode<APDisconnected>>
where
    S: Read<u8, Error = E> + Write<u8, Error = E>,
{
    //TODO deal with the optional args
    /// Connects to an access point
    pub fn connect_ap(
        mut self,
        ssid: &str,
        password: &str,
        persist: Persist,
    ) -> EResult<Esp01<S, StationMode<APConnected>>> {
        self.send_command(&[
            "CWJAP",
            persist.as_str(),
            "\"",
            ssid,
            "\",\"",
            password,
            "\"",
        ])?;
        self.read_response()?;

        Ok(Esp01 {
            serial: self.serial,
            read_buf: self.read_buf,
            _mode: PhantomData,
        })
    }
}

impl<S, E> Esp01<S, StationMode<APConnected>>
where
    S: Read<u8, Error = E> + Write<u8, Error = E>,
{
    /// Disconnects from the access point
    pub fn disconnect_ap(mut self) -> EResult<Esp01<S, StationMode<APDisconnected>>> {
        self.send_command(&["CWQAP"])?;
        self.read_response()?;

        Ok(Esp01 {
            serial: self.serial,
            read_buf: self.read_buf,
            _mode: PhantomData,
        })
    }

    /// Enables/Disables autoconnection to the accesspoint on power up
    /// This configuration is saved in flash.
    pub fn autoconnect_ap(&mut self, enable: bool) -> EResult<()> {
        let param = match enable {
            true => "1",
            false => "0",
        };

        self.send_command(&["CWAUTOCONN=", param])?;
        self.read_response()?;

        Ok(())
    }
}
