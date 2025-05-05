use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use kernel::model::book::{event::CreateBook, Book};
use kernel::repository::book::BookRepository;
use uuid::Uuid;

use crate::database::ConnectionPool;

#[derive(new)]
pub struct BookRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl BookRepository for BookRepositoryImpl {
    async fn create(&self, event: CreateBook) -> Result<()> {
        // マクロを使用するとビルド時に有効なSQLかチェックされる
        sqlx::query!(
            r#"
                INSERT INTO books (title, author, isbn, description)
                VALUES($1, $2, $3, $4)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description            
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Book>> {
        todo!()
    }

    async fn find_by_id(&self, book_id: Uuid) -> Result<Option<Book>> {
        todo!()
    }
}
