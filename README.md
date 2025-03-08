# rust-webapp

## 第1章

### フロントエンドの環境構築

```shell
# リポジトリのクローン
git clone https://github.com/rust-web-app-book/rusty-book-manager
# 依存関係のインストール
cd frontend
npm install
# ローカルでの立ち上げ
npm run dev
# ブラウザなどでlocalhost:3000にアクセスする
```

## 第2章

### クレートの追加

```shell
cargo add axum@0.7.5 --features macros
cargo add tokio@1.37.0 --features full
cargo add --dev rstest@0.18.2
```

### p44 実行

```shell
# サーバーを起動する
cargo run
# リクエストを送る
curl localhost:8080/hello -v
```

### p48 anyhowの追加

```shell
cargo add anyhow@1.0.75
```

### p49 impl IntoResponce

ハンドラの返り値はIntoResponceトレイトを実装している。

https://docs.rs/axum/0.7.5/axum/response/trait.IntoResponse.html


### p51 動作確認

``` shell
cargo run
curl localhost:8080/health -v
```

### p53 cargo-nextest

https://nexte.st/

``` shell
cargo install cargo-nextest
```

``` shell
cargo nextest run
```

### p54 sqlxのインストール

``` shell
cargo add sqlx@0.7.3 --features runtime-tokio,uuid,chrono,macros,postgres,migrate
```

### p55 コネクションプール

https://wa3.i-3-i.info/word12762.html

### p56 axum State機能

ハンドラ間で状態を共有できる機能

### p59 データベースのユニットテスト

``` shell
# PostgresSQLを起動していなければ起動する
cargo make before-build
# テストの実行
cargo make test
```

### p59 エンドポイントの動作確認

``` shell
cargo make run
curl localhost:8080/health/db -v
```

``` shell
cargo make run
cargo make compose-down
curl localhost:8080/health/db -v
```


ポート5432が使用済みなのでエラーが発生した。
Visual Studio CodeでStopで対応した。

```
 ⠋ Container rusty-book-manager-postgres-1  Starting                                                                           0.0s 
Error response from daemon: driver failed programming external connectivity on endpoint rusty-book-manager-postgres-1 (e7b2423801579b0681d8b4904808777a5ceea44b030c3c229292cdc1d1e91837): Bind for 0.0.0.0:5432 failed: port is already allocated
Error while executing command, exit code: 1
```

```shell
# ポート5432を使用しているプロセスの確認
lsof -i :5432
# PostgreSQLコンテナが動いているか確認
docker ps | grep postgres  
# 該当するコンテナを停止
docker stop <コンテナID>   
```

## 第4章

### p95, 96

``` shell
cargo make run
curl -v http://localhost:8080/health/db
```