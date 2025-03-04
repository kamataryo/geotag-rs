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

## 使い方

### バイナリのダウンロード

[GitHub のリリースページ](https://github.com/kamataryo/geotag-rs/releases/latest) から、最新バージョンのバイナリをダウンロードできます。  
利用可能なプラットフォームに応じて、適切なバイナリを選択してください。

ダウンロードしたバイナリを実行可能にし、適切なディレクトリに移動して使用してください。

```shell
$ curl -sL https://github.com/kamataryo/geotag-rs/releases/download/v0.0.2/geotag_x86_64-apple-darwin > ./geotag
$ chmod +x ./geotag
```

```shell
$ ./geotag --help                                                      
Usage: geotag --output-dir <OUTPUT_DIR> <GPX_PATH> <IMAGE_PATH>

Arguments:
  <GPX_PATH>    GPX file path
  <IMAGE_PATH>  Image files path. You can use glob pattern

Options:
  -o, --output-dir <OUTPUT_DIR>  Image files output
  -h, --help                     Print help
```

以下のように実行することで、 コピーされた画像に内挿された緯度経度が付きます。

```shell
$ geotag --output-dir=output ./your.gpx "./path/to/images/*.jpg"
```
