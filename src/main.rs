mod blob;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use sha1::{Sha1, Digest};
use flate2::Compression;
use flate2::write::ZlibEncoder;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 { panic!("Select mode"); }

  println!("{:?}", args);
  let mode: &str = &args[1];

  match mode {
    "add" => {
      // パスを指定して
      if args.len() < 3 { panic!("Add file path"); }
      let path: &str = &args[2];

      // ファイルを開いて
      let mut file = File::open(path).expect("Invalid path");
      let mut contents = String::new();
      file.read_to_string(&mut contents)
      // 読み込みに失敗したよ
      .expect("Something went wrong reading the file");

      // 中身をblob objectに変換
      let data = blob::encode(String::from(contents));

      match data.expect("Failed to unwrap blob object") {
        blob::Blob(object) => {
          let bytes = object.as_bytes();

          // SHA-1を通してIDを生成
          let mut id = Sha1::new();
          id.update(bytes);
          println!("{:?}", id.finalize());

          // Zlibで圧縮してcontentsを生成
          let mut zlib = ZlibEncoder::new(Vec::new(), Compression::default());
          zlib.write_all(bytes);
          let contents = zlib.finish();
          println!("{:?}", contents);
        }
      }
    },

    _ => {
      println!("you can learn about how to use with help")
    }
  }
}

