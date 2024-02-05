名刺カーネル

# 実行
1. QEMUのインストール
   1. [QEMU](https://www.qemu.org/)
2. nightlyチャンネルのRustをインストール
   1. Rustをインストールする
   2. nightlyチャンネルのコンパイラをインストールする `rustup install nightly`
   3. rust-srcコンポーネントをインストールする `rustup component add rust-src`
3. 実行コマンド `cargo run`

ブートまで時間がかかるか，うまくブートされないことがある
その時はホストコンソールで ctrl+c などで停止し，再度 `cargo run` を実行する

