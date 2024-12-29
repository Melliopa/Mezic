// src/storage/rocksdb.rs

use rocksdb::{DB, Options};

pub struct RocksDBStorage {
    db: DB,
}

impl RocksDBStorage {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).expect("Failed to open RocksDB");
        RocksDBStorage { db }
    }

    pub fn put(&self, key: &str, value: &str) {
        self.db.put(key, value).expect("Failed to put data");
    }

    pub fn get(&self, key: &str) -> Option<String> {
        match self.db.get(key) {
            Ok(Some(value)) => Some(String::from_utf8(value).expect("Invalid UTF-8")),
            _ => None,
        }
    }

    pub fn delete(&self, key: &str) {
        self.db.delete(key).expect("Failed to delete data");
    }
}
