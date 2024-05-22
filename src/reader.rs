use std::fs::read;
use std::error::Error;

pub struct Reader {
    data: String
};

impl Reader {
    pub fn new(path: &str) -> Result<Reader, Box<dyn Error>> {
        Ok(Reader {
            data: read(&cli.path)?
        })
    }

    pub fn read(&self) -> String {
        self.data.to_string()
    }
}
