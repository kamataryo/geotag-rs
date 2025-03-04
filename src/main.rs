use clap::Parser;

mod geotag;
use crate::geotag::Geotag;

#[derive(Parser, Debug)]
struct Args {
  #[arg(help = "GPX file path")]
  gpx_path: String,
  #[arg(help = "Image files path. You can use glob pattern")]
  image_path: String,
  #[clap(short, long)]
  #[arg(help = "Image files output")]
  output_dir: String,
}

fn main() {

  let args = Args::parse();

  Geotag::new(
    args.gpx_path,
    args.image_path,
    args.output_dir,
  );
}
