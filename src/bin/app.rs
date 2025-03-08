use adapter::database::connect_database_with;
use anyhow::{Error, Result};
use api::route::health::build_health_check_routers;
use axum::Router;
use registry::AppRegistry;
use shared::config::AppConfig;
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

/// サーバーの起動処理はmain関数から分離する
async fn bootstrap() -> Result<()> {
    // `AppConfig`を生成する
    let app_config = AppConfig::new()?;
    // データベースへ接続する。コネクションプールを取り出す。
    let pool = connect_database_with(&app_config.database);

    // `AppRegistry`を生成する
    let registry = AppRegistry::new(pool);

    // `AppRegistry`を`Router`に登録する
    let app = Router::new()
        .merge(build_health_check_routers())
        .with_state(registry);

    // サーバーを起動する
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(&addr).await?;

    println!("Listening on {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)

    // // データベースの接続設定を定義する。
    // let database_cfg = DatabaseConfig {
    //     host: "localhost".into(),
    //     port: 5432,
    //     username: "app".into(),
    //     password: "passwd".into(),
    //     database: "app".into(),
    // };
    // // コネクションプールを作る。
    // let conn_pool = connect_database_with(database_cfg);

    // let app = Router::new()
    //     .route("/health", get(health_check))
    //     // データベースチェック用のハンドラ
    //     .route("/health/db", get(health_check_db))
    //     // 各ハンドラで使えるようにする。
    //     .with_state(conn_pool);
    // let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    // let listener = TcpListener::bind(addr).await?;
    // println!("Listening on {}", addr);
    // Ok(axum::serve(listener, app).await?)
}
