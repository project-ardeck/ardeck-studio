# INSTALL
このソフトウェアは[tauri](https://github.com/tauri-apps/tauri)をベースに開発されています。<br>
基本的に[tauriのビルド方法](https://tauri.app/v1/guides/building/)に従うことでビルドできます。

## windows

1. [Bun js](https://bun.sh/)をインストールします。
2. [Rust and Cargo](https://www.rust-lang.org/ja)をインストールします。
3. [tauri app](https://tauri.app/v1/guides/getting-started/setup)Cargoによってビルドする場合、Cargoをインストールします。
4. `cargo tauri build`を実行することで、`src-tauri/target/release`内にインストーラーと実行ファイルが生成されます。

## ポータブルバージョン
`cargo tauri build -f portable`とすることで、ポータブルバージョンの実行ファイルを生成できます。これには、以下のような特徴があります。

- 設定ファイルが実行ファイルと同じディレクトリに保存されます
- USBメモリなどの外部メディアから実行可能です
- インストール不要で、設定を保持したまま別のPCで使用できます

注意：現在のバージョンでは未完成であり、ログファイルの出力先はデフォルトのままです。
