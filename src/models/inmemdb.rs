use super::*;
use std::collections::HashMap;

pub struct Database {
  pub item_table: HashMap<String, Item>,
}

impl DB for Database {
  fn get_items(&self) -> Vec<Item> {
    self.item_table.values().cloned().collect::<Vec<Item>>()
  }
  fn get_item(&self, uuid: String) -> &Item {
    self.item_table.get(&uuid).unwrap()
  }
  fn add_item(&mut self, mut item: Item) -> Item {
    let uuid = item.add_uuid().unwrap();

    self.item_table.insert(uuid, item.clone());
    item
  }
  fn delete_item(&mut self, uuid: String) -> Result<(), String> {
    self.item_table.remove(&uuid).unwrap();
    Ok(())
  }
  fn update_item(&mut self, uuid: String, item: Item) -> Result<(), String> {
    self.item_table.insert(uuid, item);
    Ok(())
  }
}

pub fn init_db(seed: Option<Vec<Item>>) -> SafeDB {
  match seed {
    None => Arc::new(Mutex::new(Box::new(Database {
      item_table: HashMap::new(),
    }))),
    Some(mut s) => {
      let mut hm = HashMap::new();
      for item in s.iter_mut() {
        let uuid = item.add_uuid().unwrap();
        hm.insert(uuid, item.clone());
      }
      Arc::new(Mutex::new(Box::new(Database { item_table: hm })))
    }
  }
}
