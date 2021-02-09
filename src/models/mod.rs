pub mod inmemdb;

use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Record {
  pub uuid: Option<String>,
  pub title: String,
  pub artist: String,
  pub format: String,
  pub year: u16,
}

impl Record {
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
  pub fn new(title: String, artist: String, format: String, year: u16) -> Record {
    Record {
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
  fn get_records(&self) -> Vec<Record>;
  fn add_record(&mut self, record: Record) -> Record;
  fn get_record(&self, uuid: String) -> &Record;
  fn delete_record(&mut self, uuid: String) -> Result<(), String>;
  fn update_record(&mut self, uuid: String, record: Record) -> Result<(), String>;
}

#[cfg(test)]
mod tests {

  use super::*;
  use std::collections::HashMap;
  #[test]
  fn empty_database() {
    let db = inmemdb::Database {
      record_table: HashMap::new(),
    };

    let records = db.get_records();

    assert_eq!(records.len(), 0)
  }

  #[test]
  fn add_and_get_person() {
    let mut db = inmemdb::Database {
      record_table: HashMap::new(),
    };

    let record = db.add_record(Record {
      uuid: None,
      title: "Papua New Guinea".to_string(),
      artist: "Future Sound of London".to_string(),
      format: "vinyl".to_string(),
      year: 1991,
    });

    assert_eq!(record.year, 1991);

    let get = db.get_record(record.uuid.unwrap());

    assert_eq!(get.format, "vinyl");
  }

  #[test]
  fn add_and_delete_person() {
    let mut db = inmemdb::Database {
      record_table: HashMap::new(),
    };

    let record = db.add_record(Record {
      uuid: None,
      title: "Dark and Long".to_string(),
      artist: "Underworld".to_string(),
      format: "tape".to_string(),
      year: 1993,
    });

    assert_eq!(record.year, 1993);

    let _ = db.delete_record(record.uuid.unwrap());

    assert_eq!(db.record_table.len(), 0);
  }
}
