use uuid::Uuid;

pub mod event;

/// 読み込み時に使用する型
#[derive(Debug)]
pub struct Book {
    pub id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
