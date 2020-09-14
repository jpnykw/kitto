use super::ext;

use std::io::prelude::*;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;

pub fn compress(object: String) -> String {
  let bytes = object.as_bytes();

  // 圧縮して
  let mut zlib = ZlibEncoder::new(Vec::new(), Compression::default());
  zlib.write_all(bytes);

  // objectを取り出し
  let object = match zlib.finish() {
    Ok(content) => content,
    Err(_) => panic!("Failed compress with zlib-encode"),
  };

  // 16進数にして結合する
  ext::join_iter(bytes.iter())
}

pub fn decompress(object: String) -> String {
  let mut chars = object.chars();
  let len = object.chars().count() / 2;

  let mut stack: Vec<u8> = Vec::new();
  for _ in 0 .. len {
    // 2桁ずつ取り出して
    let high = &chars.next().unwrap().to_string();
    let low = &chars.next().unwrap().to_string();
    // くっつけて数値に変換
    let code = u8::from_str_radix(&format!("{}{}", high, low), 16);
    stack.push(code.expect("Failed to unwarap u8 at decompress"));
  }

  // 元に戻すよ
  String::from_utf8(stack).unwrap()
}

