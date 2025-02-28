fn main() {
  print!("Hello, world!");
  // TODO
  // 1. clap でコマンドライン引数を取得する　
  // 2. 第一引数の gpx を使いやすい形にパース
  // 3. 第二引数の画像を取得
  // 4. 画像を output のディレクトリにコピー
  // 5. メタデータを並列でかきこみ
}

struct TimePoint {
  lat: f64,
  lon: f64,
  time: i64, // UnixTime
}

struct Timeline {
  data: Vec<TimePoint>
}

impl Timeline {
  fn new() -> Self {
    Self {
      data: Vec::new()
    }
  }
  fn import_gpx(&mut self, path: &str) {
    // TODO
    // gpx ファイルを読み込む
    // xml をパース
    // タイムライン形成
  }

  fn interpolate(&self, time: i64) -> TimePoint {
    // TODO
    // time に最も近い2つの TimePoint を取得
    // それらの間を線形補間
    // 補間した TimePoint を返す
    return TimePoint {
      lat: 0.0,
      lon: 0.0,
      time: 0
    }
  }
}
