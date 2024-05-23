use toml;
use std::collections::HashMap;
use std::error::Error;

pub struct Message (HashMap<String, String>);

impl Message {
    pub fn new(lang: &str) -> Result<Message, Box<dyn Error>> {
        let en = include_str!("lang/en.toml");
        let pt = include_str!("lang/pt.toml");
        let lang = if lang == "pt" { pt } else { en };

        let lang: HashMap<String, String> = toml::from_str(lang)?;

        Ok(Message(lang))
    }

    pub fn err(&self, error: &str) -> String {
        self.0.get(error).unwrap_or(&error.to_string()).to_string()
    }
}
