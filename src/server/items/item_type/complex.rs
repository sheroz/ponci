use crate::server::items::item_type::basic::ItemBasicType;

pub enum ItemComplexType {
    Array(ItemBasicType),
    Set(ItemBasicType),
    Map(ItemBasicType, ItemBasicType),
    Blob,
    Json,
    Xml,
    File,
    Folder,
    Path,
}
