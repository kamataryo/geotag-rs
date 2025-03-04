use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::fs;
use std::process::exit;

use chrono::{DateTime, Utc, Local};

use glob::glob;
use little_exif::metadata::Metadata;
use little_exif::exif_tag::ExifTag;
use little_exif::rational::uR64;
use little_exif::u8conversion::U8conversion;

#[derive(Debug)]
enum GeotagError {
  GpxFileOpenError,
  GpxFileParseError,
}

impl From<gpx::errors::GpxError> for GeotagError {
    fn from(_: gpx::errors::GpxError) -> Self {
        GeotagError::GpxFileParseError
    }
}

impl From<std::io::Error> for GeotagError {
    fn from(_: std::io::Error) -> Self {
        GeotagError::GpxFileOpenError
    }
}

#[derive(Debug)]
pub struct Geotag {
  gpx_path: String,
  image_path: String,
  output_dir: String,
  timeline: Vec<TimePoint>,
  pub target_images: Vec<String>,
}

#[derive(Debug)]
struct TimePoint {
  lat: f64,
  lon: f64,
  elevation: Option<f64>,
  time: i64, // UnixTime
}

impl Geotag {
  pub fn new(
    gpx_path: String,
    image_path: String,
    output_dir: String,
  ) -> Self {
    let mut geotag = Self {
      gpx_path,
      image_path,
      output_dir,
      timeline: Vec::new(),
      target_images: Vec::new(),
    };

    if let Err(err) = geotag.generate_timeline() {
        eprintln!("Error generating timeline: {:?}", err);
        exit(1);
    }

    geotag.parse_glob();
    geotag.exec();

    return geotag;
  }

  fn generate_timeline(&mut self) -> Result<(), GeotagError> {
    let file = File::open(&self.gpx_path)
      .map_err(|_| GeotagError::GpxFileOpenError)?;
    let file_reader = BufReader::new(file);
    let gpx_reader = gpx::read(file_reader)?;

    for track in gpx_reader.tracks {
      for segment in track.segments {
        for way_point in segment.points.clone() {
          let time = way_point.time.ok_or(GeotagError::GpxFileParseError)?;
          let iso_time_string = time.format().unwrap();
          let datetime: DateTime<Utc> = DateTime::parse_from_rfc3339(&iso_time_string)
            .map_err(|_| GeotagError::GpxFileParseError)?
            .into();
          let unix_timestamp = datetime.timestamp();

          let point = TimePoint {
            lat: way_point.point().y(),
            lon: way_point.point().x(),
            elevation: way_point.elevation,
            time: unix_timestamp,
          };
          self.timeline.push(point);
        }
      }
    }
    self.timeline.sort_by(|a, b| a.time.cmp(&b.time));
    Ok(())
}

  fn parse_glob(&mut self) {
    let paths: Vec<_> = glob(&self.image_path).expect("Failed to read glob pattern").collect();
    if paths.is_empty() {
      eprintln!("No image files found");
      exit(0);
    }
    for path in paths {
      let path = path.unwrap();
      let path_str = path.to_str().unwrap().to_string();
      self.target_images.push(path_str);
    }
  }

  fn exec(&self) {
    let output_dir = Path::new(&self.output_dir);
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let mut output_paths: Vec<PathBuf> = Vec::new();
    for image_path in &self.target_images {
      let image_path = Path::new(image_path);
      let image_name = image_path.file_name().unwrap();
      let output_path = output_dir.join(image_name);
      // コピー実行
      fs::copy(image_path, &output_path).expect("Failed to copy image file");
      output_paths.push(output_path);
    }

    for output_path in output_paths  {
      let mut metadata = Metadata::new_from_path(&output_path).expect("Failed to read metadata");

      let date_time_original = metadata.get_tag(&ExifTag::DateTimeOriginal(String::new())).next();
      if date_time_original.is_none() {
        // skip
        continue;
      } else {
        let endian = metadata.get_endian();
        let date_time_string = String::from_u8_vec(
          &date_time_original.unwrap().value_as_u8_vec(&endian),
          &endian,
        );
        let date_part = &date_time_string[0..10].replace(":", "-");
        let time_part = &date_time_string[11..19];

        let offset = Local::now().offset().local_minus_utc();
        let iso_time_string = format!("{}T{}Z", date_part, time_part);
        let date_time = DateTime::parse_from_rfc3339(&iso_time_string).unwrap().with_timezone(&Utc);
        let timestamp = date_time.timestamp() - offset as i64;

        let interpolated_timepoint = self.interpolate(timestamp);
        if let Some(interpolated_timepoint) = interpolated_timepoint {
          let mut lat = interpolated_timepoint.lat;
          let mut lon = interpolated_timepoint.lon;
          let lat_ref = (if lat >= 0.0 { "N" } else { "S" }).to_string();
          let lon_ref = (if lon >= 0.0 { "E" } else { "W" }).to_string();
          lat = lat.abs();
          lon = lon.abs();
          let lat_degree = lat as u32;
          let lng_degree = lon as u32;
          let lat_minute = ((lat - lat_degree as f64) * 60.0) as u32;
          let lng_minute = ((lon - lng_degree as f64) * 60.0) as u32;
          let lat_second = (100000.0 * ((lat - lat_degree as f64) * 60.0 - lat_minute as f64) * 60.0) as u32;
          let lng_second = (100000.0 * ((lon - lng_degree as f64) * 60.0 - lng_minute as f64) * 60.0) as u32;

          let lat_vec = vec![
            uR64 {  nominator: lat_degree,  denominator: 1 },
            uR64 {  nominator: lat_minute,  denominator: 1 },
            uR64 {  nominator: lat_second,  denominator: 100000 },
          ];
          let lon_vec = vec![
            uR64 {  nominator: lng_degree,  denominator: 1 },
            uR64 {  nominator: lng_minute,  denominator: 1 },
            uR64 {  nominator: lng_second,  denominator: 100000 },
          ];

          let elevation = interpolated_timepoint.elevation;

          metadata.set_tag(ExifTag::GPSLatitudeRef(lat_ref));
          metadata.set_tag(ExifTag::GPSLatitude(lat_vec));
          metadata.set_tag(ExifTag::GPSLongitudeRef(lon_ref));
          metadata.set_tag(ExifTag::GPSLongitude(lon_vec));
          if let Some(elevation) = elevation {
            let elevation_vec = vec![uR64 {  nominator: elevation as u32,  denominator: 1 }];
            metadata.set_tag(ExifTag::GPSAltitude(elevation_vec));
          }
          metadata.write_to_file(&output_path).expect("Failed to write metadata");
        }
      }
    }
  }

  fn interpolate(&self, timestamp: i64) -> Option<TimePoint> {
    // time に最も近い2つの TimePoint を取得
    // それらの間を線形補間
    // 補間した TimePoint を返す

    let mut timestamp1: Option<i64> = None;
    let mut timestamp2: Option<i64> = None;

    let mut count = 0;

    for _ in &self.timeline {
      if count < self.timeline.len() - 1 {
        if
          self.timeline[count].time < timestamp &&
          self.timeline[count + 1].time > timestamp
        {
          timestamp1 = Some(self.timeline[count].time);
          timestamp2 = Some(self.timeline[count + 1].time);
          break;
        }
        count += 1;
      } else {
        count += 1;
      }
    }

    if timestamp1.is_none() || timestamp2.is_none() {
      return None;
    } else {
      let time1 = timestamp1.unwrap();
      let time2 = timestamp2.unwrap();

      let time_diff = time2 - time1;
      let time_diff1 = timestamp - time1;
      let time_diff2 = time2 - timestamp;

      let lat1 = self.timeline[count].lat;
      let lon1 = self.timeline[count].lon;
      let elevation1 = self.timeline[count].elevation;

      let lat2 = self.timeline[count + 1].lat;
      let lon2 = self.timeline[count + 1].lon;
      let elevation2 = self.timeline[count + 1].elevation;

      let lat = (lat1 * time_diff2 as f64 + lat2 * time_diff1 as f64) / time_diff as f64;
      let lon = (lon1 * time_diff2 as f64 + lon2 * time_diff1 as f64) / time_diff as f64;
      let elevation = if elevation1.is_some() && elevation2.is_some() {
        let elevation1 = elevation1.unwrap();
        let elevation2 = elevation2.unwrap();
        Some((elevation1 * time_diff2 as f64 + elevation2 * time_diff1 as f64) / time_diff as f64)
      } else {
        None
      };

      return Some(TimePoint {
        lat,
        lon,
        elevation,
        time: timestamp,
      })
    }
  }
}
