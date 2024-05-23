use serialport;
use serialport::{SerialPort, TTYPort, ClearBuffer::Input};
use std::path::PathBuf;
use std::error::Error;
use std::io::Read;
use std::{thread, time::Duration};

const WAIT_MILLISECONDS: u64 = 500;

pub struct Serial (TTYPort);

impl Serial {
    pub fn new(
        path: &PathBuf,
        baud_rate: u32
    ) -> Result<Serial, Box<dyn Error>> {
        let path = path.as_path().as_os_str().to_str().ok_or("unreachable")?;
        Ok(Serial (serialport::new(path, baud_rate).open_native()?))
    }

    pub fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut data: Vec<u8> = vec![0; 18];

        self.0.clear(Input)?;
        thread::sleep(Duration::from_millis(WAIT_MILLISECONDS));

        match self.0.read(data.as_mut_slice()) {
            Ok(18) => Ok(data),
            Ok(_) => Err("ERR_INTEGRITY".into()),
            Err(_) => Err("ERR_INTEGRITY".into())
        }
    }
}
