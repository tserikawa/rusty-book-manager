# Docker Composeでの起動

基本的な実行方法

```shell
docker compose up app --build
```

常駐プログラム(デーモン)として動作させる場合

```shell
# デーモンとして実行する
docker compose up -d app --build
# ログを表示する
docker compose logs
# コンテナを停止する
docker compose stop
```

PstgreSQL, Redi

```shell
# 実行
docker compose up -d redis postgres
# コマンドの実行にはインストールが必要
redis-cli -h localhost -p 6379
```

# タスクランナー cargo-make

複数コマンドの一括実行や依存関係の整理が簡単にできるようになる。

```shell
# インストール
cargo install --force cargo-make
# バージョン確認
cargo make --version
# 実行
cargo make run
```