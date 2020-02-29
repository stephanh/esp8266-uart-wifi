use std::io::{ErrorKind, Read, Write};
use std::time::Duration;
use std::{env, str};

use serial::{self, core::SerialPort};

use nb;

use embedded_hal as hal;

use esp01::esp01;
use esp01::Mode::*;
use esp01::Persist::*;

pub struct Serial<T: Read + Write>(pub T);

impl<T: Read + Write> hal::serial::Read<u8> for Serial<T> {
    type Error = ErrorKind;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        let mut buffer = [0; 1];
        let bytes_read = self.0.read(&mut buffer).map_err(translate_io_errors)?;
        if bytes_read == 1 {
            Ok(buffer[0])
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

/// Helper to convert std::io::Error to the nb::Error
fn translate_io_errors(err: std::io::Error) -> nb::Error<ErrorKind> {
    match err.kind() {
        ErrorKind::WouldBlock | ErrorKind::TimedOut | ErrorKind::Interrupted => {
            nb::Error::WouldBlock
        }
        err => nb::Error::Other(err),
    }
}

impl<T: Read + Write> hal::serial::Write<u8> for Serial<T> {
    type Error = ErrorKind;

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.0.write(&[word]).map_err(translate_io_errors)?;
        Ok(())
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        self.0.flush().map_err(translate_io_errors)
    }
}

impl<T: Read + Write> core::fmt::Write for Serial<T> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        use embedded_hal::serial::Write;
        let _ = s
            .as_bytes()
            .iter()
            .map(|c| nb::block!(self.write(*c)))
            .last();
        Ok(())
    }
}

fn main() -> Result<(), ErrorKind> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path-to-serial>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];

    let settings = serial::PortSettings {
        baud_rate: serial::Baud115200,
        char_size: serial::Bits8,
        parity: serial::ParityNone,
        stop_bits: serial::Stop1,
        flow_control: serial::FlowNone,
    };

    let mut port = serial::open(path).expect("Could not open serial port");
    port.configure(&settings)
        .expect("Could not configure serial port");
    port.set_timeout(Duration::from_secs(1))
        .expect("Could not set serial port timeout");

    let mut s = Serial(port);
    let mut esp01 = esp01(s);
    let r = esp01.get_version()?;
    println!("{}", str::from_utf8(r).unwrap());

    let r = esp01.set_mode(StationMode, DontSave)?;
    println!("{}", str::from_utf8(r).unwrap());

    Ok(())

    /*loop {
        use hal::serial::Read;
        let c = nb::block!(s.read()).unwrap() as char;
        print!("{}", c);
    }*/
}
