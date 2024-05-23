mod file;
mod serial;

use file::File;
use serial::Serial;
use std::path::PathBuf;
use std::error::Error;

pub enum Reader {
    File(File),
    Serial(Serial)
}

impl Reader {
    pub fn new(path: &PathBuf, baud_rate: u32) -> Result<Reader, Box<dyn Error>> {
        match baud_rate {
            0 => Ok(Reader::File(File::new(path)?)),
            baud_rate => Ok(Reader::Serial(Serial::new(path, baud_rate)?))
        }
    }

    pub fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        match self {
            Reader::File(file) => file.read(),
            Reader::Serial(serial) => serial.read()
        }
    }
}
