pub mod inmemdb;

use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Item {
    pub uuid: Option<String>,
    pub title: String,
    pub artist: String,
    pub format: String,
    pub year: u16,
}

impl Item {
    pub fn add_uuid(&mut self) -> Result<String, &str> {
        match &self.uuid {
            None => {
                let _uuid = Uuid::new_v4().to_string();
                self.uuid = Some(_uuid.clone());
                Ok(_uuid)
            }
            Some(_) => Err("UUID is already set"),
        }
    }
    pub fn new(title: String, artist: String, format: String, year: u16) -> Item {
        Item {
            uuid: None,
            title,
            artist,
            format,
            year,
        }
    }
}

pub type SafeDB = Arc<Mutex<Box<dyn DB + Send>>>;

pub trait DB {
    fn get_items(&self) -> Vec<Item>;
    fn add_item(&mut self, item: Item) -> Item;
    fn get_item(&self, uuid: String) -> Result<&Item, &str>;
    fn delete_item(&mut self, uuid: String) -> Result<(), String>;
    fn update_item(&mut self, uuid: String, item: Item) -> Result<(), String>;
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn uuid_already_set() {
        let mut item = Item {
            uuid: Some("some_uuid".to_string()),
            artist: "Orbital".to_string(),
            title: "Chime".to_string(),
            year: 1991,
            format: "vinyl".to_string(),
        };

        assert_eq!(item.add_uuid(), Err("UUID is already set"))
    }
}
