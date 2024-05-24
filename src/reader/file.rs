use std::fs::read;
use std::path::PathBuf;
use std::error::Error;

pub struct File {
    data: Vec<u8>,
    index: usize
}

impl File {
    pub fn new(path: &PathBuf) -> Result<File, Box<dyn Error>> {
        Ok(File {
            data: read(path)?,
            index: 0
        })
    }

    pub fn read(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let a = self.index;
        let b = self.index+18;
        let l = self.data.len();
        self.index = if b >= l {0} else {b};
        if b > l {
            Err("ERR_INTEGRITY".into())
        } else {
            Ok(self.data[a..b].iter().cloned().collect())
        }
    }
}
