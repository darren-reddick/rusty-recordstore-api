use super::*;
use std::collections::HashMap;

pub struct Database {
    pub item_table: HashMap<String, Item>,
}

impl DB for Database {
    fn get_items(&self) -> Vec<Item> {
        self.item_table.values().cloned().collect::<Vec<Item>>()
    }
    fn get_item(&self, uuid: String) -> Result<&Item, &str> {
        let ret = self.item_table.get(&uuid);

        match ret {
            None => Err("Could not get item"),
            Some(_) => Ok(ret.unwrap()),
        }
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

#[cfg(test)]
mod tests {

    use super::*;
    use std::collections::HashMap;
    #[test]
    fn empty_database() {
        let db = inmemdb::Database {
            item_table: HashMap::new(),
        };

        let items = db.get_items();

        assert_eq!(items.len(), 0)
    }

    #[test]
    fn add_and_get_item() {
        let mut db = inmemdb::Database {
            item_table: HashMap::new(),
        };

        let item = db.add_item(Item {
            uuid: None,
            title: "Papua New Guinea".to_string(),
            artist: "Future Sound of London".to_string(),
            format: "vinyl".to_string(),
            year: 1991,
        });

        assert_eq!(item.year, 1991);

        let get = db.get_item(item.uuid.unwrap()).unwrap();

        assert_eq!(get.format, "vinyl");
    }

    #[test]
    #[should_panic]
    fn get_nonexist_item() {
        let db = inmemdb::Database {
            item_table: HashMap::new(),
        };

        db.get_item("xxx-xxxx-xxxx-xxxx".to_string()).unwrap();
    }

    #[test]
    fn add_and_delete_item() {
        let mut db = inmemdb::Database {
            item_table: HashMap::new(),
        };

        let item = db.add_item(Item {
            uuid: None,
            title: "Dark and Long".to_string(),
            artist: "Underworld".to_string(),
            format: "tape".to_string(),
            year: 1993,
        });

        assert_eq!(item.year, 1993);

        let _ = db.delete_item(item.uuid.unwrap());

        assert_eq!(db.item_table.len(), 0);
    }

    #[test]
    fn add_and_update_item() {
        let mut db = inmemdb::Database {
            item_table: HashMap::new(),
        };

        let item = db.add_item(Item {
            uuid: None,
            title: "Two Months Off".to_string(),
            artist: "Underworld".to_string(),
            format: "cd".to_string(),
            year: 1993,
        });

        let uuid = &item.uuid.unwrap();

        assert_eq!(item.year, 1993);

        let _ = db.update_item(
            uuid.clone(),
            Item {
                uuid: None,
                title: "Two Months Off".to_string(),
                artist: "Underworld".to_string(),
                format: "cd".to_string(),
                year: 2002,
            },
        );

        let item = db.get_item(uuid.clone()).unwrap();

        assert_eq!(item.year, 2002)
    }
}
