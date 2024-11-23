# INSTALL
このソフトウェアは[tauri](https://github.com/tauri-apps/tauri)をベースに開発されています。<br>
基本的に[tauriのビルド方法](https://tauri.app/v1/guides/building/)に従うことでビルドできます。
## windows
1. [Bun js](https://bun.sh/)をインストールします。
2. [Rust and Cargo](https://www.rust-lang.org/ja)をインストールします。
3. [tauri app](https://tauri.app/v1/guides/getting-started/setup)Cargoによってビルドする場合、Cargoをインストールします。
4. `cargo tauri build`を実行することで、`src-tauri/target/release`内にインストーラーと実行ファイルが生成されます。

`cargo tauri build -f portable`とすることで、設定ファイルなどが実行ファイルの同階層に保存されるポータブルバージョンの実行ファイルが生成されます。
<!-- ### Install Bun js -->

