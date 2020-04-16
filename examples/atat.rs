use std::io::{ErrorKind, Read, Write};
use std::thread;
use std::time::Duration;
use std::{env, str};

use serialport::{self, SerialPortSettings};

use nb;

use embedded_hal as hal;

use atat::ATATInterface;

use esp01::atat as eatat;

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

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path-to-serial>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];

    let settings = SerialPortSettings {
        baud_rate: 115200,
        data_bits: serialport::DataBits::Eight,
        parity: serialport::Parity::None,
        stop_bits: serialport::StopBits::One,
        flow_control: serialport::FlowControl::None,
        timeout: Duration::from_secs(5),
    };

    let mut port =
        serialport::open_with_settings(path, &settings).expect("Could not open serial port");
    let mut serial_rx = port.try_clone().expect("Could not clone serial port");

    let mut s = Serial(port);
    let timer = timer::SysTimer::new();
    let config = atat::Config::new(atat::Mode::Blocking);
    let (mut client, mut ingress) = atat::new(s, timer, config);

    // Launch reading thread
    thread::Builder::new()
        .name("serial_read".to_string())
        .spawn(move || loop {
            let mut buffer = [0; 32];
            match serial_rx.read(&mut buffer[..]) {
                Ok(0) => println!("here"),
                Ok(bytes_read) => {
                    println!(
                        "Got data: {}",
                        str::from_utf8(&buffer[0..bytes_read]).unwrap()
                    );
                    ingress.write(&buffer[0..bytes_read]);
                    ingress.parse_at();
                    ingress.parse_at();
                }
                Err(e) => match e.kind() {
                    ErrorKind::Interrupted => {}
                    _ => {
                        log::error!("Serial reading thread error while reading: {}", e);
                    }
                },
            }
        })
        .unwrap();

    //let cmd = eatat::GetCWMODECUR;
    let cmd = eatat::GetCWMODECUR;
    println!("Sending command {:?}", cmd);
    println!("Result: {:?}", client.send(&cmd));
}

mod timer {
    use std::time::{Duration, Instant};

    use embedded_hal::timer::{CountDown, Periodic};

    /// A timer with milliseconds as unit of time.
    pub struct SysTimer {
        start: Instant,
        duration_ms: u32,
    }

    impl SysTimer {
        pub fn new() -> SysTimer {
            SysTimer {
                start: Instant::now(),
                duration_ms: 0,
            }
        }
    }

    impl CountDown for SysTimer {
        type Time = u32;

        fn start<T>(&mut self, count: T)
        where
            T: Into<Self::Time>,
        {
            self.start = Instant::now();
            self.duration_ms = count.into();
        }

        fn wait(&mut self) -> nb::Result<(), void::Void> {
            if (Instant::now() - self.start) > Duration::from_millis(self.duration_ms as u64) {
                // Restart the timer to fulfil the contract by `Periodic`
                self.start = Instant::now();
                Ok(())
            } else {
                Err(nb::Error::WouldBlock)
            }
        }
    }

    impl Periodic for SysTimer {}
}
