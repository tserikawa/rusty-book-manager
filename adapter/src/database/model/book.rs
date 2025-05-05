use kernel::model::book::Book;
use uuid::Uuid;

/// データベースのレコードを読み取る時に使用する型
pub struct BookRow {
    pub book_id: Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

// BookRowはメソッド内部で利用する構造体なので、
// 戻り値にする時はkernelで定義したBook構造体にする。
impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        // パターンマッチを使って`BookRow`の中身を取り出す。
        // = 構造体の中身を直接取り出している。
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            id: book_id,
            title,
            author,
            isbn,
            description,
        }
    }
}
