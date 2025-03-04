# geotag-rs

`geotag-rs` は、 GPX ファイルから画像ファイルの位置情報を内挿して修正するツールです。

## ビルド方法

### Cargo を使ったビルド

1. まず、[Rust](https://www.rust-lang.org/tools/install) をインストールしてください
2. 次に、リポジトリをクローンします

  ```shell
  $ git clone https://github.com/kamataryo/geotag-rs.git
  ```

3. ディレクトリに移動して、Cargo を使ってビルドします
　
  ```shell
  $ cd geotag-rs
  $ cargo build --release
  ```

ビルドが完了すると、実行可能なバイナリが `target/release` ディレクトリに生成されますので適切なディレクトリに移動して使用してください。

### バイナリのダウンロード

[GitHub のリリースページ](https://github.com/kamataryo/geotag-rs/releases/latest) から、最新バージョンのバイナリをダウンロードできます。  
利用可能なプラットフォームに応じて、適切なバイナリを選択してください。

ダウンロードしたバイナリを実行可能にし、適切なディレクトリに移動して使用してください。

```shell
$ curl -sL https://github.com/kamataryo/geotag-rs/releases/download/v0.1.0/geotag-rs_x86_64-unknown-linux-gnu > ./geotag
$ chmod +x ./geotag
```
