use axum::{extract::State, http::StatusCode};
use registry::AppRegistry;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

/// `State`に登録されている`AppRegistry`を取り出す。
pub async fn health_check_db(State(registory): State<AppRegistry>) -> StatusCode {
    // health_check_repositoryメソッド経由でリポジトリの処理を呼び出せる
    if registory.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
