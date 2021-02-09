use super::*;
use std::collections::HashMap;

pub struct Database {
  pub record_table: HashMap<String, Record>,
}

impl DB for Database {
  fn get_records(&self) -> Vec<Record> {
    self.record_table.values().cloned().collect::<Vec<Record>>()
  }
  fn get_record(&self, uuid: String) -> &Record {
    self.record_table.get(&uuid).unwrap()
  }
  fn add_record(&mut self, mut record: Record) -> Record {
    let uuid = record.add_uuid().unwrap();

    self.record_table.insert(uuid, record.clone());
    record
  }
  fn delete_record(&mut self, uuid: String) -> Result<(), String> {
    self.record_table.remove(&uuid).unwrap();
    Ok(())
  }
  fn update_record(&mut self, uuid: String, record: Record) -> Result<(), String> {
    self.record_table.insert(uuid, record);
    Ok(())
  }
}

pub fn init_db(seed: Option<Vec<Record>>) -> SafeDB {
  match seed {
    None => Arc::new(Mutex::new(Box::new(Database {
      record_table: HashMap::new(),
    }))),
    Some(mut s) => {
      let mut hm = HashMap::new();
      for record in s.iter_mut() {
        let uuid = record.add_uuid().unwrap();
        hm.insert(uuid, record.clone());
      }
      Arc::new(Mutex::new(Box::new(Database { record_table: hm })))
    }
  }
}
