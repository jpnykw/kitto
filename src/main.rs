mod blob;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use sha1::{Sha1, Digest};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::slice::Iter;

fn join_iter(iter: Iter<'_, u8>) -> String {
  iter.map(|&byte| format!("{:<02x}", byte)).collect::<String>()
}

fn sha1(content: &String) -> String {
  let mut sha1 = Sha1::new();
  sha1.update(content.as_bytes());
  join_iter(sha1.finalize().iter())
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 { panic!("Select mode"); }

  println!("{:?}", args);
  let mode: &str = &args[1];
  println!("git {}", mode);

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
      let blob_object = blob::encode(String::from(contents));
      println!("object {:?}", blob_object);

      match blob_object.expect("Failed to unwrap blob object") {
        blob::Blob(object) => {
          let bytes = object.as_bytes();

          // SHA-1を通してIDを生成
          let id = sha1(&object);
          println!("blob {}", id);

          // Zlibで圧縮してcontentsを生成
          let mut zlib = ZlibEncoder::new(Vec::new(), Compression::default());
          zlib.write_all(bytes);
          let contents = match zlib.finish() {
            Ok(content) => join_iter(content.iter()),
            Err(_) => panic!("Failed compress with zlib-encode"),
          };
          println!("content {}", contents);
        }
      }
    },

    _ => {
      println!("you can learn about how to use with help")
    }
  }
}

#[test]
fn sha1_case_a() {
  assert_eq!(
    sha1(&String::from("a")),
    String::from("86f7e437faa5a7fce15d1ddcb9eaeaea377667b8")
  );
}

#[test]
fn sha1_case_b() {
  assert_eq!(
    sha1(&String::from("b")),
    String::from("e9d71f5ee7c92d6dc9e92ffdad17b8bd49418f98")
  );
}

#[test]
fn sha1_case_c() {
  assert_eq!(
    sha1(&String::from("c")),
    String::from("84a516841ba77a5b4648de2cd0dfcb30ea46dbb4")
  );
}

#[test]
fn sha1_case_git() {
  assert_eq!(
    sha1(&String::from("git")),
    String::from("46f1a0bd5592a2f9244ca321b129902a06b53e03")
  );
}

