# Actix Web Playgroun

Rust の Web フレームワーク Actix Web の検証用リポジトリ

## 構成環境

- Rust: 1.54.0
- Actix Web: 3.3.2

## ファイル構成

```sh
.
├── README.md
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain
├── src
│   ├── main.rs
│   ├── routes.rs
│   ├── controllers.rs
│   └── models.rs
└── target/
```

## ローカル環境構築

### パッケージ・ライブラリのインストール

```sh
# パッケージのインストール
cargo install --path .
# ライブリロード用のライブラリをインストールする
cargo install cargo-watch
```

### Actix Web の起動

```sh
cargo watch -x run
```

### 動作確認

```sh
curl http://localhost:8000
```

## ドキュメント

- https://actix.rs/docs/
