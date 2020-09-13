mod blob;

use std::env;
use std::fs::File;
use std::io::prelude::*;

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
      println!("{:?}", data.unwrap());
    },

    _ => {
      println!("you can learn about how to use with help")
    }
  }
}

