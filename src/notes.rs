use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Notes {
    pub dict : BTreeMap<u32, String>
}

impl Notes {
    pub fn new() -> Notes {
        Notes {
            dict : BTreeMap::new(),
        }
    }

    fn realign(&mut self) {
        let mut temp : BTreeMap<u32, String> = BTreeMap::new();

        let mut i : u32 = 1;

        for item in &self.dict {
            temp.insert(i, item.1.clone());
            i += 1;
        }
        
        self.dict = temp;
    }

    pub fn add(&mut self, item : String) {
        let next_key = match self.dict.last_key_value() {
            Some((&last_key, _)) => last_key + 1,
            None => 1,
        };

        self.dict.insert(next_key, item);
    }

    pub fn remove(&mut self, item : u32) {
        self.dict.remove(&item);
        self.realign();
    }

    pub fn edit(&mut self, item : u32,  value : String) {
        self.dict.insert(item, value);
    }

    pub fn save(&self) -> Result<u8, std::io::Error> {
        let serialized : String = match serde_json::to_string(self) {
            Ok(serialized) => serialized,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        };
        
        match std::fs::write(PathBuf::from("./notes.json"), serialized.as_bytes()) {
            Ok(_) => {},
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        };

        Ok(0)
    }

    pub fn load() -> Result<Notes, std::io::Error> {
        let contents : String = match std::fs::read_to_string(PathBuf::from("./notes.json")) {
            Ok(string) => string,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
        };

        let notes: Notes = match serde_json::from_str(&contents) {
            Ok(n) => n,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())),
        };
        Ok(notes)
    }
}