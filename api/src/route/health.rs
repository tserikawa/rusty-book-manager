use axum::{routing::get, Router};
use registry::AppRegistry;

use crate::handler::health::{health_check, health_check_db};

/// RouterのStateがAppRegistoryとなるため、Routerの型引数に指定する
pub fn build_health_check_routers() -> Router<AppRegistry> {
    // ヘルスチェック関連のルートhealthに個別のパスをネストする。
    let routers = Router::new()
        .route("/", get(health_check))
        .route("/db", get(health_check_db));
    Router::new().nest("/health", routers)
}
