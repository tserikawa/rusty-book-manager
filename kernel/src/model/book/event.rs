/// Bookに対する処理を行う際に使用する型(イベント)
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
