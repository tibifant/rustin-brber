use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct DungeonPlayerError {
    message: String,
}

impl DungeonPlayerError {
    pub fn new(message: &str) -> DungeonPlayerError {
        DungeonPlayerError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for DungeonPlayerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for DungeonPlayerError {}