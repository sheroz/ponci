use std::collections::HashMap;
use crate::server::items::item_type::{complex::ItemComplexType, storage::ItemStorageType};

pub struct StorageItem {
    _item_type: ItemComplexType,
    _data: Box<Vec<u8>>,
    _description: String,
    _tags: Vec<String>,
    _metadata: HashMap<String, String>,
    _may_expire: bool,
    _expires_on: std::time::Instant,
    _storage: Vec<ItemStorageType>,
    _redundancy: u8, // min number of required replications in the claster: 0,1,2, …
}
